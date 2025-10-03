use std::time::Duration;

use bevy::{
    prelude::*,
    render::view::VisibilityClass,
    color::palettes::css
};
use avian3d::prelude::*;
use bevy_hanabi::{ParticleEffect, EffectAsset, EffectMaterial};

use crate::shared::{DamageDeal, HealthMax, LifeTime, Shot}; 
use crate::effects::jet_stream;

pub struct MissilePlugin;
impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_systems(Startup, init_demo)
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
struct EffectImage(Handle<Image>);


#[derive(Component)]
struct Missile;

// const DAMAGE_VALUE: f32 = 10.;
const VELOCITY_VALUE: f32 = 60.;
const LIFETIME_VALUE: f32 = 10.;

// ---

fn init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: ResMut<AssetServer>,
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
    )).id();

    cmd.insert_resource(MissileSample(id));
    cmd.insert_resource(EffectImage(assets.load("textures/cloud.png")));

}

#[allow(dead_code)]
fn init_demo(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut effects: ResMut<Assets<EffectAsset>>,
    assets: ResMut<AssetServer>,
    mut cmd: Commands
) {
    
    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(0.2))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(1., 1., 1.),
        Sensor,
        Collider::sphere(0.2),
        CollisionEventsEnabled,
        RigidBody::Kinematic,
        Missile,
        HealthMax(1.),
        DamageDeal(0.1)
    ))

    .with_child((
        Name::new("jet2"),
        ParticleEffect::new(effects.add(jet_stream())),
        Transform::IDENTITY
        .with_rotation(Quat::from_rotation_x(90.0_f32.to_radians()))
        ,
        EffectMaterial{
            images: vec![
                assets.load("textures/cloud.png")
            ]
        },
    ))
    ;

}



// ---

fn fire(
    tr: Trigger<Shot>,
    sample: Res<MissileSample>,
    mut effects: ResMut<Assets<EffectAsset>>,
    image: Res<EffectImage>, 
    mut cmd: Commands
) {
    let shot = tr.event();
    // info!("{:?}", shot);

    cmd.entity(sample.0)
        .clone_and_spawn_with(|b| {
        b.deny::<VisibilityClass>();
    })
    .insert((
        Missile,
        Visibility::Visible,
        Position(shot.position),
        // Transform::IDENTITY.looking_to(shot.direction, Vec3::Y),
        Rotation(Quat::from_rotation_arc(-Vec3::Z, *shot.direction)),
        LinearVelocity(shot.direction * VELOCITY_VALUE),
        LifeTime(Timer::from_seconds(LIFETIME_VALUE, TimerMode::Once)),
        HealthMax(1.),
        DamageDeal(0.1)
    ))
    .with_child((
        Name::new("jet2"),
        ParticleEffect::new(effects.add(jet_stream())),
        Transform::IDENTITY.with_rotation(Quat::from_rotation_x(90.0_f32.to_radians())),
        EffectMaterial{
            images: vec![
                image.0.clone()
            ]
        },
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
        if l.0.finished() {
            cmd.entity(e).despawn();
        }
    }
}

// ---

fn on_destroy(
    tr: Trigger<OnRemove, Missile>,
    trans_q: Query<&Transform>
) {
    let Ok(trans) = trans_q.get(tr.target())  else {
        return;
    };
    println!("destroied {:?}", trans.translation);
}

// ---

fn gizmos(
    mut gizmos: Gizmos,
    q: Query<&Transform, With<Missile>>
    // q : Query<&GlobalTransform, With<Spot>>

) {
    for t in &q {
        // gizmos.axes(*t, 10.);
        gizmos.ray(t.translation, t.forward() * 100., css::BLUE);
    }
}
