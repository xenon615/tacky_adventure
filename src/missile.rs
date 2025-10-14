use bevy::{
    prelude::*,
    color::palettes::css
};
use avian3d::prelude::*;
use bevy_hanabi::{EffectAsset, EffectMaterial, EffectSpawner, ParticleEffect};

use crate::shared::{DamageDeal, HealthMax, LifeTime, Shot}; 
use crate::effects::{blast, jet_stream};

pub struct MissilePlugin;
impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, init)
        .add_systems(Update, check_lifetime.run_if(any_with_component::<LifeTime>))
        .add_observer(fire)
        // .add_systems(Update, gizmos)
        .add_observer(on_destroy)
        ;
    }
}

// ---

#[derive(Resource)]
struct MissileSample(Entity);

#[derive(Resource)]
struct EffectStuff{
    image: Handle<Image>,
    jet: Handle<EffectAsset>,
    // blast: Handle<EffectAsset>
}

#[derive(Component)]
struct Missile;

#[derive(Component)]
struct Blast;


const VELOCITY_VALUE: f32 = 60.;
const LIFETIME_VALUE: f32 = 5.;

// ---

fn init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: ResMut<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut cmd: Commands
) {
    
    let id = cmd.spawn((
        Visibility::Hidden,
        Mesh3d(meshes.add(Sphere::new(0.2))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(1000., 1000., 1000.),
        Sensor,
        Collider::sphere(0.2),
        CollisionEventsEnabled,
        RigidBody::Kinematic
    )).id()
    ;
    let image_h = assets.load("textures/spark1.png");

    cmd.spawn((
        ParticleEffect::new(effects.add(blast())),
        EffectMaterial{
            images: vec![
                image_h.clone()
            ]
        },
        Blast
    ));


    cmd.insert_resource(MissileSample(id));
    cmd.insert_resource(EffectStuff {
        image: image_h.clone(),
        jet: effects.add(jet_stream()),
        // blast: effects.add(blast())
    });




}

// ---

fn fire(
    tr: On<Shot>,
    sample: Res<MissileSample>,
    e_stuff: Res<EffectStuff>, 
    mut cmd: Commands
) {
    let shot = tr.event();
    cmd.entity(sample.0)
    .clone_and_spawn()

    .insert((
        Missile,
        Visibility::Visible,
        Position(shot.position),
        Rotation(Quat::from_rotation_arc(-Vec3::Z, *shot.direction)),
        LinearVelocity(shot.direction * VELOCITY_VALUE),
        LifeTime(Timer::from_seconds(LIFETIME_VALUE, TimerMode::Once)),
        HealthMax(1.),
        DamageDeal(0.1),
        children![
            (
                ParticleEffect::new(e_stuff.jet.clone()),
                Transform::IDENTITY.with_rotation(Quat::from_rotation_x(90.0_f32.to_radians())),
                EffectMaterial{
                    images: vec![
                        e_stuff.image.clone()
                    ]
                },
            ),
        ]
    ));

}

// ---

fn check_lifetime(
    mut cmd: Commands,
    mut lt_q: Query<(Entity, &mut LifeTime)>,
    time: Res<Time>
) {
    for (e, mut l) in &mut lt_q {
        l.0.tick(time.delta());
        if l.0.is_finished() {
            cmd.entity(e).despawn();
        }
    }
}

// ---

fn on_destroy(
    tr: On<Remove, Missile>,
    trans_q: Query<&Transform, Without<Blast>>,
    blast_q: Single<(&mut Transform, &mut EffectSpawner), With<Blast>>,
) {

    let Ok(trans) = trans_q.get(tr.entity)  else {
        return;
    };

    let (mut t, mut es) = blast_q.into_inner();
    t.translation = trans.translation;
    es.reset();


    // println!("destroyed {:?}", trans.translation);
}



// ---

#[allow(dead_code)]
fn gizmos(
    mut gizmos: Gizmos,
    q: Query<&Transform, With<Missile>>
) {
    for t in &q {
        // gizmos.axes(*t, 10.);
        gizmos.ray(t.translation, t.forward() * 100., css::BLUE);
    }
}


// ---

// #[allow(dead_code)]
// fn init_demo(
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut effects: ResMut<Assets<EffectAsset>>,
//     assets: ResMut<AssetServer>,
//     mut cmd: Commands
// ) {
    
//     cmd.spawn((
//         Mesh3d(meshes.add(Sphere::new(0.2))),
//         MeshMaterial3d(materials.add(Color::WHITE)),
//         Transform::from_xyz(1., 1., 1.),
//         Sensor,
//         Collider::sphere(0.2),
//         CollisionEventsEnabled,
//         RigidBody::Kinematic,
//         Missile,
//         HealthMax(1.),
//         DamageDeal(0.1),
//         children![
//             (
//                 Name::new("jet2"),
//                 ParticleEffect::new(effects.add(jet_stream())),
//                 Transform::IDENTITY
//                 .with_rotation(Quat::from_rotation_x(90.0_f32.to_radians()))
//                 ,
//                 EffectMaterial{
//                     images: vec![
//                         assets.load("textures/cloud.png")
//                     ]
//                 },
//             ),

//             (
//                 ParticleEffect::new(e_stuff.blast.clone()),
//                 EffectMaterial{
//                     images: vec![
//                         e_stuff.image.clone()
//                     ]
//                 },
//                 Blast
//             )



//         ]
//     ))

    
//     ;

// }
