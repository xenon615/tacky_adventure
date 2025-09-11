use bevy::{
    gizmos,  pbr::Material, prelude::*, render::{render_resource::{AsBindGroup, ShaderRef}, view::VisibilityClass}
};
use avian3d::{math::Quaternion, prelude::*};

use crate::shared::{Build, BuildAction, Exit, GameStage, PLATFORM_DIM};

pub struct PlatformPlugin;
impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<PlatformMaterial>::default())
        .add_systems(Startup, startup)
        // .add_systems(Update, gismos)
        .add_systems(OnEnter(GameStage::Two), change_stage)
        .add_observer(build_single)
        ;
    }
}

// ---


pub const PITCH_ANGLE: f32 = 30.0_f32.to_radians();
pub const YAW_ANGLE: f32 = 90.0_f32.to_radians();
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

enum PDir {
    Left,
    Right,
    Up,
    Down,
    Forward,
    Back,
    BackUp,
    BackDown,
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
}
impl PDir {
    fn get_rotation(r: &PDir ) -> Quat {
        match r {
            Self::Left => Quat::from_rotation_y(YAW_ANGLE),
            Self::Right => Quat::from_rotation_y(-YAW_ANGLE),
            Self::Up => Quat::from_rotation_x(PITCH_ANGLE),
            Self::Down => Quat::from_rotation_x(-PITCH_ANGLE),
            Self::Forward => Quat::IDENTITY,
            Self::Back =>  Quat::from_rotation_y(2. * YAW_ANGLE),
            Self::BackUp =>  Quat::from_rotation_y(2. * YAW_ANGLE) * Quat::from_rotation_x(PITCH_ANGLE),
            Self::BackDown =>  Quat::from_rotation_y(2. * YAW_ANGLE) * Quat::from_rotation_x(-PITCH_ANGLE),
            Self::LeftDown => Quat::from_rotation_y(YAW_ANGLE) * Quat::from_rotation_x(-PITCH_ANGLE),
            Self::LeftUp => Quat::from_rotation_y(YAW_ANGLE) * Quat::from_rotation_x(PITCH_ANGLE),
            Self::RightDown => Quat::from_rotation_y(-YAW_ANGLE) * Quat::from_rotation_x(-PITCH_ANGLE),
            Self::RightUp => Quat::from_rotation_y(-YAW_ANGLE) * Quat::from_rotation_x(PITCH_ANGLE),
        }
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
        stage_index:GameStage::get_index(&stage) 
    }); 
    cmd.insert_resource(PlatformMaterialHandle(material.clone()));
    
    let rotations = vec![
        PDir::Forward,
        PDir::Forward,
        PDir::Forward,
        PDir::Forward,
        PDir::Forward,
        PDir::Forward,
        PDir::Forward,
        PDir::Forward,
        PDir::Forward,
        PDir::Forward,        
    ];

    let mut pos = Vec3::ZERO;
    let mut trans = Transform::IDENTITY;

    for (idx, rs) in  rotations.iter().enumerate() {
        if idx > 0 {
            let dir =  match *rs {
                PDir::Right |  PDir::RightDown | PDir::RightUp => trans.right(),
                PDir::Left | PDir::LeftDown | PDir::LeftUp => trans.left(),
                _ => trans.forward()
            };

            let (step, shift) = if ![trans.forward(), trans.back()].contains(&dir) {
                (PLATFORM_DIM.x, trans.forward() * 0.5 * (PLATFORM_DIM.z - PLATFORM_DIM.x))
            } else {
                (PLATFORM_DIM.z, Vec3::ZERO)
            };

            let connect_point = trans.translation + dir * step * 0.5 + shift;    
            trans.rotate_local(PDir::get_rotation(rs));
            pos = connect_point + trans.rotation.mul_vec3(-Vec3::Z * PLATFORM_DIM.z * (0.5 + GAP));
        }

        trans = Transform::from_translation(pos).with_rotation(trans.rotation);
        cmd.spawn((
            trans,
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Collider::cuboid(PLATFORM_DIM.x, PLATFORM_DIM.y, PLATFORM_DIM.z),
            RigidBody::Static,
            Platform,
            Name::new("Platform")
        ));
    }


}

// ---

fn build_single(
    tr: Trigger<Build>,
    mut cmd: Commands,
    pt_q: Query<&Transform>,
    spatial: SpatialQuery,
) {
    let Build(act, p_e, d) = tr.event();
    let Ok(pt) = pt_q.get (*p_e) else {
        return;
    };
    let Some(face_to) = [pt.forward(), pt.back(), pt.right(), pt.left()]
    .into_iter().max_by(|a, b| {
        d.dot(**a).total_cmp(&d.dot(**b))
    }) else {
        return;
    };
   
    let add = Quat::from_rotation_arc(*pt.forward(), *face_to).normalize();

    let rotation = pt.rotation * add *  match act {
        BuildAction::Up => PDir::get_rotation(&PDir::Up),
        BuildAction::Down => PDir::get_rotation(&PDir::Down),
        BuildAction::Forward | BuildAction::Delete => PDir::get_rotation(&PDir::Forward)
    };

    let (step, shift) = if ![pt.forward(), pt.back()].contains(&face_to) {
        (PLATFORM_DIM.x, pt.forward() * 0.5 * (PLATFORM_DIM.z - PLATFORM_DIM.x))
    } else {
        (PLATFORM_DIM.z, Vec3::ZERO)
    };

    let connect_point = pt.translation + *face_to * step * 0.5 + shift;
    let pos = connect_point +  rotation.mul_vec3(-Vec3::Z *  PLATFORM_DIM.z * (0.5 + GAP));

    // println!("{:?}  {}  {}", *face_to, connect_point, pos);
    if *act != BuildAction::Delete {
        cmd.entity(*p_e)
        .clone_and_spawn_with(|b| {
            b.deny::<VisibilityClass>();
        })
        .insert((   
            Position::new(pos),
            Rotation(rotation)
        ))
        ;
    } else {
        spatial.shape_intersections(&Collider::sphere(0.5), connect_point, Quaternion::IDENTITY, &SpatialQueryFilter::default())
        .iter()
        .filter(| e | **e != *p_e)
        .for_each(|e| cmd.entity(*e).despawn());
    }

}

// ---

fn change_stage(
    mh: Res<PlatformMaterialHandle>,
    mut materials: ResMut<Assets<PlatformMaterial>>
) {
    let Some(m) = materials.get_mut(&mh.0) else {
        return;
    };
    m.stage_index = 1;
}