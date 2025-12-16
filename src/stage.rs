use bevy::{
   prelude::*, 
   render::render_resource::AsBindGroup,
   shader::ShaderRef
};
use avian3d::prelude::*;
use crate::{
    shared::vec_rnd,
    player::Player
};

pub struct StagePlugin;
impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<StageIndex>()
        .add_plugins(MaterialPlugin::<StageStoneMaterial>::default())
        .add_systems(Startup, start)
        .add_systems(Update, stage_index_changed_local.run_if(resource_changed::<StageIndex>))
        .add_systems(Update, move_stone.run_if(any_with_component::<MoveStageStone>))
        ;  
    }
}

#[derive(Resource, Default)]
pub struct StageIndex(pub usize);


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StageStoneMaterial {
    // #[uniform(0)]
    // color: LinearRgba,
    #[uniform(0)]
    stage_index: u32
}


impl Material for StageStoneMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/exit.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

// ---

#[derive(Resource)]
pub struct StageStoneMaterialHandle(Handle<StageStoneMaterial>);

#[derive(Component)]
struct MoveStageStone(Vec3);

#[derive(Component)]
pub struct StageStone;

// ---

fn start(
    mut cmd : Commands,
    mut materials: ResMut<Assets<StageStoneMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let emh = materials.add(StageStoneMaterial {stage_index: 0});
    cmd.insert_resource(StageStoneMaterialHandle(emh.clone()));
    cmd.spawn((
        StageStone,
        Mesh3d(meshes.add(Cuboid::from_length(4.))),
        MeshMaterial3d(emh.clone()),
        Transform::from_translation(Vec3::new(0., 3., -100.)),
        RigidBody::Kinematic,
        Collider::cuboid(4., 4., 4.),
        CollisionEventsEnabled,
        // Sensor,
    ))
    .observe(on_collide)
    ;
}

// ---

fn on_collide(
    tr: On<CollisionStart>,
    tr_q: Single<&mut AngularVelocity, With<StageStone>>,
    mut stage_index: ResMut<StageIndex>,
    player_q: Query<&Player>,
    mut cmd: Commands
) {
    let Some(body2) = tr.body2 else {return;};

    if player_q.get(body2).is_err() {
        return;
    }
    stage_index.0 += 1;
    let max = if stage_index.0 > 2 {40} else {20};
    let Some(me) = tr.body1 else {return;};
    tr_q.into_inner().0 = Vec3::Y * 2.;
    cmd.entity(me).insert(MoveStageStone(vec_rnd(-max .. max, 0 .. max, -max .. max)));
}

// ---

fn stage_index_changed_local(
    mh: Res<StageStoneMaterialHandle>,
    mut materials: ResMut<Assets<StageStoneMaterial>>,
    opt_index: Res<StageIndex>
) {
    if opt_index.0 == 1 {
        let Some(m) = materials.get_mut(&mh.0) else {
            return;
        };
        m.stage_index = 1;
    }

}

// ---

fn move_stone(
    mut cmd : Commands,
    tr_q: Single<(Entity, &mut Transform, &mut AngularVelocity, &MoveStageStone), With<StageStone>>,  
    time: Res<Time>
) {
    let (e, mut trans, mut av, me) =  tr_q.into_inner();
    if trans.translation.distance_squared(me.0) < 0.2 {
        cmd.entity(e).remove::<MoveStageStone>();
        // cmd.entity(e).insert(Sensor);
        av.0 = Vec3::ZERO;
    } else {
        trans.translation = trans.translation.lerp(me.0, time.delta_secs() * 1.);
    }

}

pub fn stage_index_changed<const STAGE_INDEX: usize, T: Resource + Default> (
    stage_index: Res<StageIndex>,
    mut cmd: Commands
) {
    if stage_index.0 == STAGE_INDEX {
        cmd.init_resource::<T>();
    }
} 

