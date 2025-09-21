use std::ops::Range;

use bevy::{
   prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, 
};


use avian3d::prelude::*;
pub struct ExitPlugin;
impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<ExitMaterial>::default())
        .add_systems(Startup, start)
        .add_systems(OnExit(GameStage::Intro), change_shader)
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

// ---

#[derive(Resource)]
pub struct ExitMaterialHandle(Handle<ExitMaterial>);


// ---

fn start(
    mut cmd : Commands,
    mut materials: ResMut<Assets<ExitMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    stage: Res<State<GameStage>>
) {
    let emh = materials.add(ExitMaterial {stage_index: GameStage::get_index_by_state(&stage)});
    cmd.insert_resource(ExitMaterialHandle(emh.clone()));
    cmd.spawn((
        Exit,
        Mesh3d(meshes.add(Cuboid::from_length(4.))),
        MeshMaterial3d(emh.clone()),
        Transform::from_translation(Vec3::new(0., 3., -3.)),
        RigidBody::Kinematic,
        Collider::cuboid(4., 4., 4.),
        CollisionEventsEnabled,
        Sensor
    ));

}

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
    tr_q: Single<&mut Transform, With<Exit>>,
    mut stage_index: Local<usize>
) {
    *stage_index += 1; 
    println!("------stage index----- {}", *stage_index);
    next.set(GameStage::get_state_by_index(*stage_index));
    let mut t = tr_q.into_inner();
    let mut max = 20;

    if *stage_index > 2 {
        max *= 5;
    }

    t.translation = vec_rnd(-max .. max, 0 .. max, -max .. max);
}

// ---

fn change_shader(
    mh: Res<ExitMaterialHandle>,
    mut materials: ResMut<Assets<ExitMaterial>>
) {
    let Some(m) = materials.get_mut(&mh.0) else {
        return;
    };
    m.stage_index = 1;
}
