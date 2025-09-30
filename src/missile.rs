use std::time::Duration;

use bevy::{
    prelude::*,
    render::view::VisibilityClass,
};
use avian3d::prelude::*;

use crate::shared::{Damage, LifeTime, SetDamage, Shot}; 

pub struct MissilePlugin;
impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, init)
        .add_systems(Update, check_lifetime.run_if(any_with_component::<LifeTime>))
        .add_observer(fire)

        ;
    }
}

// ---

#[derive(Resource)]
struct MissileSample(Entity);

#[derive(Component)]
struct Missile;

const DAMAGE_VALUE: f32 = 10.;
const VELOCITY_VALUE: f32 = 10.;
const LIFETIME_VALUE: f32 = 10.;

// ---

fn init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cmd: Commands
) {
    
    let id = cmd.spawn((
        Visibility::Hidden,
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_xyz(1000., 1000., 1000.),
        Sensor,
        Collider::sphere(0.5),
        CollisionEventsEnabled,
        RigidBody::Kinematic
    )).id();

    cmd.insert_resource(MissileSample(id));
}


fn fire(
    tr: Trigger<Shot>,
    sample: Res<MissileSample>,
    mut cmd: Commands
) {
    let shot = tr.event();
    info!("{:?}", shot);

    cmd.entity(sample.0)
        .clone_and_spawn_with(|b| {
        b.deny::<VisibilityClass>();
    })
    .insert((
        Missile,
        Visibility::Visible,
        Position(shot.position),
        // .looking_to(shot.direction, Vec3::Y),
        LinearVelocity(shot.direction * VELOCITY_VALUE),
        LifeTime(Timer::from_seconds(LIFETIME_VALUE, TimerMode::Once))
    )).observe(touch);

}

// ---

fn touch(
    tr: Trigger<OnCollisionStart>,
    damageable_q: Query<&Damage>,
    mut cmd: Commands
) {
    if let Some(ce)  = tr.body {
        if damageable_q.get(ce).is_ok() {
            cmd.trigger(SetDamage(DAMAGE_VALUE));
            cmd.entity(tr.target()).despawn();
        }
    }
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
