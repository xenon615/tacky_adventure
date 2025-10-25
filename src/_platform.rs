use bevy::{
    // gizmos, 
    pbr::Material, prelude::*, render::{
        render_resource::AsBindGroup, 
    },
    shader::ShaderRef
};
use avian3d::{math::Quaternion, prelude::*};

use crate::{
    help::SetHelpData,
    shared::{get_platform, CastBuild, Exit, OptionIndex,  Player, MonologueAddLine, PLATFORM_DIM}
};

pub struct PlatformPlugin;
impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<PlatformMaterial>::default())
        .add_systems(Startup, startup)
        .add_systems(Update, (change_color, set_help).chain().run_if(resource_added::<EnabledBuild>))
        .add_systems(
            Update, build_single.run_if(
                resource_exists::<EnabledBuild>
                .and(resource_changed::<ButtonInput<KeyCode>>)
            )
        )
        .add_systems(Update, opt_index_changed.run_if(resource_changed::<OptionIndex>))
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

#[derive(Resource)]
struct EnabledBuild;


// ---

#[allow(dead_code)]
fn gismos(
    mut gizmos: Gizmos,
    t_q: Query<&Transform, Or<(With<Platform>, With<Exit>)>>,
    // t_q: Query<&Transform, With<Player>>
) {
    for t in &t_q {
        gizmos.ray(t.translation, t.forward() * PLATFORM_DIM.z /2., Color::srgb(0., 0., 1.));
        gizmos.ray(t.translation, t.right() * PLATFORM_DIM.z /2., Color::srgb(1., 0., 0.));
        gizmos.ray(t.translation, t.up() * PLATFORM_DIM.z /2., Color::srgb(0., 1., 0.));
    }
}

// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlatformMaterial>>,
) {

    let mesh = meshes.add(Cuboid::from_size(PLATFORM_DIM));
    let material = materials.add(PlatformMaterial {
        color: Color::srgba (0., 0., 1., 0.1).into(), 
        stage_index: 0 
    }); 
    cmd.insert_resource(PlatformMaterialHandle(material.clone()));

    cmd.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(material.clone()),
        Collider::cuboid(PLATFORM_DIM.x, PLATFORM_DIM.y, PLATFORM_DIM.z),
        RigidBody::Static,
        Platform,
        Name::new("Platform")
    ));
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

// ---

fn build_single(
    player_q: Single<(Entity, &Transform), With<Player>>,
    mut cmd: Commands,
    spatial: SpatialQuery,
    keys: Res<ButtonInput<KeyCode>>,
    trans_q: Query<&Transform, Without<Player>>
) {
    if !(keys.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) && keys.any_just_pressed([KeyCode::KeyQ, KeyCode::KeyA, KeyCode::KeyZ, KeyCode::KeyX])) {
        return;
    }
    
    let  (_player_e, player_t) = player_q.into_inner();
    let Some(RayHitData { entity: platform_e, distance: _ , normal: _ }) = get_platform(player_t, &spatial) else {
        return;
    };

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

    let _le = build_platform(&mut cmd, &spatial, platform_e, player_t.forward(), build_action, trans_q);
   
    cmd.trigger(CastBuild);
}


// ---

fn build_platform(
    cmd: &mut Commands,
    spatial: &SpatialQuery,
    platform_e: Entity, 
    build_dir: Dir3,
    build_action: BuildAction,
    trans_q: Query<&Transform, Without<Player>>
) {
    let Ok(platform_t) = trans_q.get(platform_e) else {
        warn!("No platform");
        return;
    };

    let Some(face_to) = [platform_t.forward(), platform_t.back(), platform_t.right(), platform_t.left()]
    .into_iter().max_by(|a, b| {
        build_dir.dot(**a).total_cmp(&build_dir.dot(**b))
    }) else {
        warn!("No face");
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
    let pos = connect_point + rotation.mul_vec3(-Vec3::Z *  PLATFORM_DIM.z * (0.5 + GAP));
    let intersect: Vec<_> = spatial.shape_intersections(&Collider::sphere(0.5), connect_point, Quaternion::IDENTITY, &SpatialQueryFilter::default())
        .into_iter()
        .filter(| e | ![platform_e].contains(e))
        .collect();

    if build_action != BuildAction::Delete{
        if intersect.len() != 0 {
            warn!("No intersect");
            return;
        }
        cmd.entity(platform_e)
        .clone_and_spawn()
        .insert((   
            Position::new(pos),
            Rotation(rotation)
        ))
        ;
    } else {
        intersect.iter().for_each(|e| cmd.entity(*e).despawn());
    }

}

// ---

fn change_color(
    mh: Res<PlatformMaterialHandle>,
    // mh_q: Single<&MeshMaterial3d<PlatformMaterial>>,
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
    cmd.trigger(MonologueAddLine::new("Platform Builder is available, check out the help"));
}

// --

const OPTION_INDEX: usize = 1;

fn opt_index_changed(
    opt_index: Res<OptionIndex>,
    mut cmd: Commands
) {
    if opt_index.0 == OPTION_INDEX {
        cmd.insert_resource(EnabledBuild);
    }
} 