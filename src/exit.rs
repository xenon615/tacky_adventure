use bevy::{
   prelude::*, 
   render::render_resource::AsBindGroup,
   shader::ShaderRef
};
use avian3d::prelude::*;
use crate::shared::{vec_rnd, Exit,  OptionIndex, Player};

pub struct ExitPlugin;
impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<ExitMaterial>::default())
        .add_systems(Startup, start)
        // .add_systems(OnExit(GameStage::Intro), change_shader)

        .add_systems(Update, opt_index_changed.run_if(resource_changed::<OptionIndex>))
        .add_systems(Update, move_exit.run_if(any_with_component::<MoveExit>))
        ;  
    }
}



#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ExitMaterial {
    // #[uniform(0)]
    // color: LinearRgba,
    #[uniform(0)]
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
) {
    let emh = materials.add(ExitMaterial {stage_index: 0});
    cmd.insert_resource(ExitMaterialHandle(emh.clone()));
    cmd.spawn((
        Exit,
        Mesh3d(meshes.add(Cuboid::from_length(4.))),
        MeshMaterial3d(emh.clone()),
        Transform::from_translation(Vec3::new(0., 3., -100.)),
        RigidBody::Kinematic,
        Collider::cuboid(4., 4., 4.),
        CollisionEventsEnabled,
        // Sensor,
        Name::new("Exit")

    ))
    .observe(on_collide)
    ;
}

// ---

fn on_collide(
    tr: On<CollisionStart>,
    tr_q: Single<&mut AngularVelocity, With<Exit>>,
    mut option_index: ResMut<OptionIndex>,
    player_q: Query<&Player>,
    mut cmd: Commands
) {
    let Some(body2) = tr.body2 else {return;};

    if player_q.get(body2).is_err() {
        return;
    }
    option_index.0 += 1;
    // println!("------option index----- {}", option_index.0);
    let max = if option_index.0 > 2 {40} else {20};
    let Some(me) = tr.body1 else {return;};
    tr_q.into_inner().0 = Vec3::Y * 2.;
    cmd.entity(me).insert(MoveExit(vec_rnd(-max .. max, 0 .. max, -max .. max)));
    // cmd.entity(me).remove::<Sensor>();
}


// ---

fn opt_index_changed(
    mh: Res<ExitMaterialHandle>,
    mut materials: ResMut<Assets<ExitMaterial>>,
    opt_index: Res<OptionIndex>
) {
    if opt_index.0 == 1 {
        let Some(m) = materials.get_mut(&mh.0) else {
            return;
        };
        m.stage_index = 1;
    }

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
        // cmd.entity(e).insert(Sensor);
        av.0 = Vec3::ZERO;
    } else {
        trans.translation = trans.translation.lerp(me.0, time.delta_secs() * 1.);
    }

}



