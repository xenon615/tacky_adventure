use bevy::{
    gizmos,  pbr::Material, prelude::*, render::{render_resource::{AsBindGroup, ShaderRef}, view::VisibilityClass}
};
use avian3d::{math::Quaternion, prelude::*};

use crate::{
    help::SetHelpData,
    shared::{CastBuild, Exit, GameStage, Player, PLATFORM_DIM, get_platform, SetMonologueText}
};

pub struct PlatformPlugin;
impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<PlatformMaterial>::default())
        .add_systems(Startup, startup)
        // .add_systems(Update, gismos)
        .add_systems(OnEnter(GameStage::Build), (change_color, set_help))
        .add_systems(
            Update, build_single.run_if(
                not(in_state(GameStage::Intro))
                .and(resource_changed::<ButtonInput<KeyCode>>)
            )
        )
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
    stage: Res<State<GameStage>>
) {

    let mesh = meshes.add(Cuboid::from_size(PLATFORM_DIM));
    let material = materials.add(PlatformMaterial {
        color: Color::srgba (0., 0., 1., 0.1).into(), 
        stage_index:GameStage::get_index_by_state(&stage) 
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
    
    let  (player_e, player_t) = player_q.into_inner();
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
        return;
    };

    let add = Quat::from_rotation_arc(*platform_t.forward(), *face_to).normalize();

    let rotation = platform_t.rotation * add * match keys.get_just_pressed().next() {
        Some(KeyCode::KeyQ) => Quat::from_rotation_x(PITCH_ANGLE),
        Some(KeyCode::KeyZ) => Quat::from_rotation_x(-PITCH_ANGLE),  
        Some(KeyCode::KeyA) | Some(KeyCode::KeyX) => Quat::IDENTITY,
        _ => Quat::IDENTITY  
    };

    let (step, shift) = if ![platform_t.forward(), platform_t.back()].contains(&face_to) {
        (PLATFORM_DIM.x, platform_t.forward() * 0.5 * (PLATFORM_DIM.z - PLATFORM_DIM.x))
    } else {
        (PLATFORM_DIM.z, Vec3::ZERO)
    };

    let connect_point = platform_t.translation + *face_to * step * 0.5 + shift;
    let pos = connect_point +  rotation.mul_vec3(-Vec3::Z *  PLATFORM_DIM.z * (0.5 + GAP));


    let intersect: Vec<_> = spatial.shape_intersections(&Collider::sphere(0.5), connect_point, Quaternion::IDENTITY, &SpatialQueryFilter::default())
        .into_iter()
        .filter(| e | ![platform_e, player_e].contains(e))
        .collect();



    if !keys.just_pressed(KeyCode::KeyX) {
        if intersect.len() != 0 {
            return;
        }
        cmd.entity(platform_e)
        .clone_and_spawn_with(|b| {
            b.deny::<VisibilityClass>();
        })
        .insert((   
            Position::new(pos),
            Rotation(rotation)
        ))
        ;
    } else {
        intersect.iter().for_each(|e| cmd.entity(*e).despawn());
    }
    cmd.trigger(CastBuild);

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
    cmd.trigger(SetMonologueText("Platform Builder is available, check out the help"));
}

