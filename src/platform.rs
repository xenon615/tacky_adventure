use bevy::{
    // gizmos, 
    pbr::Material,
     prelude::*, 
     render::render_resource::AsBindGroup, 
     shader::ShaderRef
};
use avian3d::{math::Quaternion, prelude::*};

use crate::{
    help::SetHelpData, 
    info::InfoCont, 
    shared::GameState,
    monologue::MonoLines,
    player::{CastBuild, Player},
    messages::MessagesAddLine,
    stage::{StageIndex, stage_index_changed}
};

pub struct PlatformPlugin;
impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<PlatformMaterial>::default())
        .add_systems(OnEnter(GameState::Intro), startup)
        .add_systems(Update, (change_color, set_help, add_lines).run_if(resource_added::<EnabledBuild>))
        .add_systems(
            Update, apply_keys.run_if(
                resource_exists::<EnabledBuild>
                .and(resource_changed::<ButtonInput<KeyCode>>)
            )
        )
        .add_systems(Update, stage_index_changed::<1, EnabledBuild>.run_if(resource_changed::<StageIndex>))
        ;
    }
}

// ---

pub const PITCH_ANGLE: f32 = 30.0_f32.to_radians();
pub const GAP: f32 = 0.01;


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlatformMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    stage_index: u32
}

impl Material for PlatformMaterial {
    fn fragment_shader() -> ShaderRef {
         "shaders/platform.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}

// ---

#[derive(Component, Clone)]
pub struct Platform;

#[derive(Resource)]
pub struct PlatformMaterialHandle(Handle<PlatformMaterial>);

#[derive(Resource, Default)]
struct EnabledBuild;

// ---

pub fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlatformMaterial>>,
) {

    let mesh = meshes.add(Cuboid::from_size(PLATFORM_DIM));
    let material = materials.add(PlatformMaterial {
        color: Color::srgba (0., 0., 0., 0.1).into(), 
        stage_index: 0 
    }); 
    cmd.insert_resource(PlatformMaterialHandle(material.clone()));

    let id = 
    cmd.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(material.clone()),
        Collider::cuboid(PLATFORM_DIM.x, PLATFORM_DIM.y, PLATFORM_DIM.z),
        RigidBody::Static,
        Platform,
        Name::new("Platform")
    ))
    .id()
    ;
    cmd.run_system_cached_with(clone_platform, (id, Dir3::NEG_Z, BuildAction::Forward, 5));
}

// ---

#[derive(PartialEq, Debug)]
enum BuildAction {
    Up,
    Forward,
    Down,
    Delete,
    None
}


pub const PLATFORM_DIM: Vec3 = Vec3::new(10., 0.1, 10.);

// ---

pub fn get_platform(pt: &Transform, raycast_q: &SpatialQuery) -> Option<RayHitData> {
    raycast_q.cast_ray(
        pt.translation + pt.down() * 0.01, 
        Dir3::NEG_Y,
        f32::MAX,
        false, 
        &SpatialQueryFilter::default()
    )
}


fn apply_keys(
    player_q: Single<(Entity, &Transform), With<Player>>,
    mut cmd: Commands,
    spatial: SpatialQuery,
    keys: Res<ButtonInput<KeyCode>>,
    trans_q: Query<&Transform, Without<Player>>
) {
    if !(keys.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) && keys.any_just_pressed([KeyCode::KeyQ, KeyCode::KeyA, KeyCode::KeyZ, KeyCode::KeyX])) {
        return;
    }
    let build_action  =  match keys.get_just_pressed().next() {
        Some(KeyCode::KeyQ) => BuildAction::Up,
        Some(KeyCode::KeyZ) => BuildAction::Down,  
        Some(KeyCode::KeyA) => BuildAction::Forward,
        Some(KeyCode::KeyX) => BuildAction::Delete,
        _ => BuildAction::None  
    };
    if build_action == BuildAction::None {
        return;
    }
    
    let  (_player_e, player_t) = player_q.into_inner();

    let Some(RayHitData { entity: platform_e, distance: _ , normal: _ }) = get_platform(player_t, &spatial) else {
        return;
    };

    let Ok(platform_t) = trans_q.get(platform_e) else {
        return;
    };

    let Some(face_to) = [platform_t.forward(), platform_t.back(), platform_t.right(), platform_t.left()]
    .into_iter().max_by(|a, b| {
        player_t.forward().dot(**a).total_cmp(&player_t.forward().dot(**b))
    }) else {
        warn!("No face");
        return;
    };

    cmd.run_system_cached_with(clone_platform, (platform_e, face_to, build_action, 1));
   
    cmd.trigger(CastBuild);
}


// ---

fn clone_platform(
    In((platform_e, face_to, build_action, count)): In<(Entity, Dir3, BuildAction, usize)>,
    mut cmd: Commands,
    spatial: SpatialQuery,
    trans_q: Query<&Transform, Without<Player>>
 ) {
    let Ok(platform_t) = trans_q.get(platform_e) else {
        warn!("No platform");
        return;
    };

    let add = Quat::from_rotation_arc(*platform_t.forward(), *face_to).normalize();

    let rotation = platform_t.rotation * add * match build_action {
        BuildAction::Up => Quat::from_rotation_x(PITCH_ANGLE),
        BuildAction::Down => Quat::from_rotation_x(-PITCH_ANGLE),  
        BuildAction::Forward | BuildAction::Delete => Quat::IDENTITY,
        _ => Quat::IDENTITY  
    };

    let (step, shift) = if ![platform_t.forward(), platform_t.back()].contains(&face_to) {
        (PLATFORM_DIM.x, platform_t.forward() * 0.5 * (PLATFORM_DIM.z - PLATFORM_DIM.x))
    } else {
        (PLATFORM_DIM.z, Vec3::ZERO)
    };

    let connect_point = platform_t.translation + *face_to * step * 0.5 + shift;
    let intersect: Vec<_> = spatial.shape_intersections(&Collider::sphere(0.5), connect_point, Quaternion::IDENTITY, &SpatialQueryFilter::default())
        .into_iter()
        .filter(| e | ![platform_e].contains(e))
        .collect();

    if build_action != BuildAction::Delete{
        if intersect.len() != 0 {
            warn!("No intersect");
            return;
        }
        let pos = connect_point + rotation.mul_vec3(-Vec3::Z *  PLATFORM_DIM.z * (0.5 + GAP));
        for i in 0 .. count {
            cmd.entity(platform_e)
            .clone_and_spawn()
            .insert((   
                Position::new(pos * (i + 1) as f32),
                Rotation(rotation)
            ))
            ;
        }
    } else {
        intersect.iter().for_each(|e| cmd.entity(*e).despawn());
    }

}


// ---

fn change_color(
    mh: Res<PlatformMaterialHandle>,
    mut materials: ResMut<Assets<PlatformMaterial>>
) {
    let Some(m) = materials.get_mut(&mh.0) else {
        return;
    };
    m.stage_index = 1;
}

// ---

fn set_help(
    mut cmd: Commands
) {
    cmd.trigger(SetHelpData{
        title: "Platform Builder", 
        keys: "Alt + Q (Up), Alt + A (Forward), Alt + Z (Dowm), Alt + X (Delete)",
        hint: "Turn in the desired direction and build a platform"
    });
    cmd.trigger(MessagesAddLine::<InfoCont>::new("Platform Builder is available, check out the help"));
}

// --

fn add_lines(
    mut mono_lines: ResMut<MonoLines>
) {
    mono_lines.0 = 
    vec![
        "Holy shit!",
        "Goodbye, colorless world",
        "Hello world of eye-bleeding colors and annoying flickering",
        "I repeat, complete bad taste",
        "Although what previously looked like dumplings...",
        "Whatever..",
        " ",
        "Probably need to get to that flickering thing again that looks like crazy plasma",
        "You can't just approach this thing, but something tells me it can be fixed.",
    ];
}
