

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
        .add_systems(Update, move_exit.run_if(any_with_component::<MoveExit>))
        ;  
    }
}

use crate::shared::{Exit, GameStage, vec_rnd};

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


#[derive(Component)]
struct MoveExit(Vec3);

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
    ))
    .observe(on_collide)
    ;
}

// ---

fn on_collide(
    tr: Trigger<OnCollisionStart>,
    mut next: ResMut<NextState<GameStage>>,
    // tr_q: Single<&mut Transform, With<Exit>>,
    tr_q: Single<&mut AngularVelocity, With<Exit>>,
    state: Res<State<GameStage>>,
    mut cmd: Commands
) {
    let stage_index = GameStage::get_index_by_state(&state) + 1;
    println!("------stage index----- {}", stage_index);
    next.set(GameStage::get_state_by_index(stage_index));
    // let mut t = tr_q.into_inner();
    let mut max = 20;

    if stage_index > 2 {
        max *= 2;
    }
    
    tr_q.into_inner().0 = Vec3::Y * 2.;
    cmd.entity(tr.target()).insert(MoveExit(vec_rnd(-max .. max, 0 .. max, -max .. max)));
    cmd.entity(tr.target()).remove::<Sensor>();
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

// ---

fn move_exit(
    mut cmd : Commands,
    tr_q: Single<(Entity, &mut Transform, &mut AngularVelocity, &MoveExit), With<Exit>>,  
    time: Res<Time>
) {
    let (e, mut trans, mut av, me) =  tr_q.into_inner();
    if trans.translation.distance_squared(me.0) < 0.2 {
        cmd.entity(e).remove::<MoveExit>();
        cmd.entity(e).insert(Sensor);
        av.0 = Vec3::ZERO;
    } else {
        trans.translation = trans.translation.lerp(me.0, time.delta_secs() * 1.);
    }

}



