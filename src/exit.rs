use std::ops::Range;

use bevy::{
    math::VectorSpace, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, scene::SceneInstanceReady
};


use avian3d::prelude::*;
pub struct ExitPlugin;
impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<ExitMaterial>::default())
        .add_systems(Startup, start)
        .add_observer(on_collide)
        ;  
    }
}

use crate::shared::{Exit, GameStage};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ExitMaterial {
    // #[uniform(0)]
    // color: LinearRgba,
    #[uniform(1)]
    stage_index: u32
}


impl Material for ExitMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/exit.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}


fn start(
    mut cmd : Commands,
    mut materials: ResMut<Assets<ExitMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    stage: Res<State<GameStage>>
) {
    cmd.spawn((
        Exit,
        Mesh3d(meshes.add(Cuboid::from_length(4.))),
        MeshMaterial3d(materials.add(ExitMaterial {stage_index: GameStage::get_index(&stage)})),
        Transform::from_translation(Vec3::new(0., 3., -85.)),
        RigidBody::Kinematic,
        Collider::cuboid(4., 4., 4.),
        CollisionEventsEnabled,
        Sensor
    ));

}




// fn start(
//     mut cmd : Commands,
//     assets: ResMut<AssetServer> 
// ) {
//     cmd.spawn((
//         SceneRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("models/exit.glb"))),
//         Transform::from_xyz(0., 0., -100.2).looking_to(Dir3::Z, Vec3::Y),
//         RigidBody::Static,
//         ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
//         Name::new("Exit"),
//     ))
//     .observe(on_ready)
//     ;
// }

// fn on_ready(
//     tr: Trigger<SceneInstanceReady>,
//     trans_q: Query<&Transform>,
//     mut cmd : Commands,
//     mut materials: ResMut<Assets<ExitMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     stage: Res<State<GameStage>>
// ) {

//     let Ok(trans) = trans_q.get(tr.target()) else {
//         return;
//     };

//     cmd.spawn((
//         Exit,
//         // Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(4.)))),
//         Mesh3d(meshes.add(Cuboid::from_length(4.))),
//         MeshMaterial3d(materials.add(ExitMaterial {stage_index: GameStage::get_index(&stage)})),
//         Transform::from_translation(trans.translation + Vec3::Y * 3.5),
//         RigidBody::Kinematic,
//         Collider::cuboid(4., 4., 0.1),
//         CollisionEventsEnabled,
//         Sensor
//     ));


// }

// ---

fn vec_rnd(rx: Range<i32>, ry: Range<i32>, rz: Range<i32>) -> Vec3{
    Vec3::new(
        fastrand::i32(rx) as _ , 
        fastrand::i32(ry) as _, 
        fastrand::i32(rz) as _
    )
}


// ---

fn on_collide(
    _tr: Trigger<OnCollisionStart>,
    mut next: ResMut<NextState<GameStage>>,
    tr_q: Single<&mut Transform, With<Exit>>
) {
    next.set(GameStage::Two);
    let mut t = tr_q.into_inner();
    t.translation = vec_rnd(-100 .. 100, 0 .. 100, -100 .. 100);

}