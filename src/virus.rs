use bevy:: {
    pbr::Material, prelude::*, render::render_resource::AsBindGroup, 
    mesh::VertexAttributeValues, 
    shader::ShaderRef,
    time::common_conditions::on_timer,
};
use bevy_hanabi::{EffectAsset, EffectMaterial, EffectSpawner, ParticleEffect};
use avian3d::prelude::*;
use std::{ops::{Add, Mul}, time::Duration};
use crate::{effects::scattering, shared::{DamageDeal, GameState, HealthMax, MonologueAddLine, OptionIndex, Player, Targetable, Threat, closest, fibonacci_sphere, vec_rnd}};

// ---

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
        .add_observer(on_despawn)
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
        AlphaMode::Blend
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

#[derive(Component)]
pub struct Scattering;


// ---

const DAMAGE_VALUE: f32 = 1.;
const MAX_HEALTH: f32 = 1.;

// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<VirusMaterial>>,
    assets: ResMut<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
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

    let image_h = assets.load("textures/spark1.png");
    cmd.spawn((
        ParticleEffect::new(effects.add(scattering())),
        LinearVelocity::default(),
        EffectMaterial{
            images: vec![
                image_h.clone()
            ]
        },
        Scattering
    ));

    cmd.insert_resource(EnabledVirus);
    cmd.trigger(MonologueAddLine::new("Virus?!!!!"));
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

const OPTION_INDEX: usize = 1;

fn opt_index_changed(
    opt_index: Res<OptionIndex>,
    mut cmd: Commands
) {
    if opt_index.0 == OPTION_INDEX {
        cmd.insert_resource(EnabledVirus);
    }
} 


// ---

fn on_despawn(
    tr: On<Remove, Virus>,
    victim_q: Query<(&Transform, &LinearVelocity), (With<Virus>, Without<Scattering>)>,
    scatt_q: Single<(&mut Transform, &mut EffectSpawner, &mut LinearVelocity), (With<Scattering>, Without<Virus>)>
) {


    let Ok((v_trans, v_lv)) = victim_q.get(tr.event_target()) else {
        return;
    };

    let (mut e_trans, mut e_es, mut e_lv) = scatt_q.into_inner();
    e_trans.translation = v_trans.translation;
    e_lv.0 = v_lv.0;
    e_es.reset();


    // println!("removed virus entity {} on {}", tr.event_target(), v_trans.translation );
}