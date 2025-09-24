use bevy:: {
    color, log::tracing_subscriber::fmt::time, pbr::Material, prelude::*, render::{
        mesh::{SphereKind, VertexAttributeValues}, render_resource::{AsBindGroup, ShaderRef},
        view::VisibilityClass,
    }, time::common_conditions::on_timer
};

use avian3d::prelude::*;
use std::{ops::{Add, Mul}, time::Duration};
use crate::shared::{vec_rnd, Player, SetDamage};

use crate::shared::{GameStage, fibonacci_sphere, closest, SetMonologueText};

pub struct VirusPlugin;
impl Plugin for VirusPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<VirusMaterial>::default())
        .add_systems(OnEnter(GameStage::Build), startup)
        .add_systems(Update, chase.run_if(resource_exists::<EnabledVirus>))
        .add_systems(Update, spawn_next.run_if(on_timer(Duration::from_secs(10))))

        ;
    }
}

// ---

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct VirusMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    stage_index: u32
}

impl Material for VirusMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/platform.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}

// ---

#[derive(Component, Clone)]
pub struct Virus;

#[derive(Component)]
pub struct VirusSample;

#[derive(Resource)]
pub struct VirusMaterialHandle(Handle<VirusMaterial>);

#[derive(Resource)]
pub struct EnabledVirus;


// ---

const DAMAGE: f32 = 1.;

// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    let mut mesh =  Mesh::from(Sphere::new(1.).mesh().kind(SphereKind::Ico { subdivisions: 6 }));
    let Some(VertexAttributeValues::Float32x3(verticis)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) else {
        return;
    };

    for point in fibonacci_sphere(32) {
        let scale = fastrand::f32().add(1.).mul(1.1).clamp(1.1, 2.);
        closest(verticis, point, scale);
    }

    mesh.compute_normals();
    cmd.spawn((
        Transform::from_xyz(1000., 0., 0.),
        Visibility::Hidden,
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::hsl(130., 1., 0.5))),
        RigidBody::Kinematic,
        ColliderConstructor::TrimeshFromMesh,
        AngularVelocity(Vec3::new(1., 1., 1.)),
        LinearVelocity(Vec3::Y),
        Sensor,
        CollisionEventsEnabled,
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
    // mut gi: Gizmos
) {
    let player_t = player_q.into_inner();
    for (vt, mut vv) in &mut virus_q  {
        let to_target = player_t.translation - vt.translation;
        let d = vv.0.normalize().dot(to_target.normalize());
        if d < 0.9 {
            vv.0 = to_target.normalize() * 5.;
        }
        // println!("{:?}", vv);
        // gi.ray(vt.translation, vv.0 * 10., color::palettes::css::DARK_RED);
    }
}

// ---

fn touch(
    tr: Trigger<OnCollisionStart>,
    player_q: Query<&Player>,
    mut cmd: Commands
) {
    if let Some(ce)  = tr.body {
        if player_q.get(ce).is_ok() {
            cmd.trigger(SetDamage(DAMAGE));
            cmd.entity(tr.target()).despawn();
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
    .clone_and_spawn_with(|b| {
        b.deny::<VisibilityClass>();
    })
    .insert((
        Virus,
        Visibility::Visible,   
        Position::new(vec_rnd(-20 .. 20, -10 .. 10, -20 .. 20)),
    )).observe(touch)
    ;
}