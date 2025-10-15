use bevy:: {
    pbr::Material, prelude::*, render::render_resource::AsBindGroup, 
    mesh::VertexAttributeValues, 
    shader::ShaderRef,
    time::common_conditions::on_timer,
};

use avian3d::prelude::*;
use std::{ops::{Add, Mul}, time::Duration};
use crate::shared::{vec_rnd, DamageDeal, HealthMax, Player, Threat, GameState, OptionIndex};

use crate::shared::{fibonacci_sphere, closest, SetMonologueText, Targetable};

pub struct VirusPlugin;
impl Plugin for VirusPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<VirusMaterial>::default())
        .add_systems(Update, startup.run_if(resource_added::<EnabledVirus>))
        .add_systems(Update, chase.run_if(resource_exists::<EnabledVirus>)        )
        .add_systems(Update, spawn_next
            .run_if(on_timer(Duration::from_secs(5)))
            .run_if(resource_exists::<EnabledVirus>)
        )
        .add_systems(OnEnter(GameState::Over), | mut cmd: Commands | cmd.remove_resource::<EnabledVirus>() )
        .add_systems(Update, opt_index_changed.run_if(resource_changed::<OptionIndex>))
        ;
    }
}

// ---

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct VirusMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material for VirusMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/virus.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}

// ---

#[derive(Component, Clone)]
#[require(Threat, Targetable)]
pub struct Virus;

#[derive(Component)]
pub struct VirusSample;

// #[derive(Resource)]
// pub struct VirusMaterialHandle(Handle<VirusMaterial>);

#[derive(Resource)]
pub struct EnabledVirus;


// ---

const DAMAGE_VALUE: f32 = 1.;
const MAX_HEALTH: f32 = 1.;

// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>
    mut materials: ResMut<Assets<VirusMaterial>>
) {

    let mut mesh = Sphere::new(1.).mesh().ico(6).unwrap();
    let Some(VertexAttributeValues::Float32x3(verticis)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) else {
        return;
    };

    for point in fibonacci_sphere(32) {
        let scale = fastrand::f32().add(1.).mul(1.1).clamp(1.1, 2.);
        closest(verticis, point, scale);
    }

    mesh.compute_normals();
    cmd.spawn((
        Transform::from_xyz(4., 2., 0.),
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(VirusMaterial{color: Color::hsl(250., 1., 0.5).into()})),
        RigidBody::Kinematic,
        ColliderConstructor::TrimeshFromMesh,
        AngularVelocity(Vec3::new(1., 1., 1.)),
        Sensor,
        CollisionEventsEnabled,
        Name::new("Virus"),
        Visibility::Hidden,
        VirusSample, 
    ))
    ;

    cmd.insert_resource(EnabledVirus);
    cmd.trigger(SetMonologueText::new("Virus?!!!!"));
} 

// ---

fn chase(
    player_q: Single<&Transform, (With<Player>, Without<Virus>)>,
    mut virus_q: Query<(&Transform, &mut LinearVelocity), (With<Virus>, Without<Player>)>,
) {
    let player_t = player_q.into_inner();
    for (vt, mut vv) in &mut virus_q  {
        let to_target = player_t.translation - vt.translation;
        let d = vv.0.normalize().dot(to_target.normalize());
        if d < 0.9 {
            vv.0 = to_target.normalize() * 5.;
        }
    }
}

// ---

fn spawn_next(
    mut cmd: Commands,
    v_q: Single<Entity, With<VirusSample>>
) {
    let ve = v_q.into_inner();

    cmd.entity(ve)
    .clone_and_spawn()
    .insert((
        Virus,
        Visibility::Visible,   
        Position::new(vec_rnd(-80 .. 80, 5 .. 50, -80 .. 80)),
        LinearVelocity(Vec3::Y),
        Targetable,
        HealthMax(MAX_HEALTH),
        DamageDeal(DAMAGE_VALUE)
    ))
    ;
}

// ---

const OPTION_INDEX: usize = 4;

fn opt_index_changed(
    opt_index: Res<OptionIndex>,
    mut cmd: Commands
) {
    if opt_index.0 == OPTION_INDEX {
        cmd.insert_resource(EnabledVirus);
    }
} 