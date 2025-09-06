use bevy::{
    gizmos, pbr::Material, prelude::*, render::{render_resource::{AsBindGroup, ShaderRef}, view::VisibilityClass}
};
use avian3d::prelude::*;

use crate::shared::{Build, BuildAction, Player};

pub struct PlatformPlugin;
impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<PlatformMaterial>::default())
        .add_systems(Startup, startup)
        // .add_systems(Update, gismos)
        .add_observer(build_single)
        ;
    }
}

// ---

pub const PLATFORM_DIM: Vec3 = Vec3::new(5., 0.1, 10.);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlatformMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
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

// ---

#[allow(dead_code)]
fn gismos(
    mut gizmos: Gizmos,
    // t_q: Query<&Transform, With<Platform>>,
    t_q: Query<&Transform, With<Player>>
) {
    for t in &t_q {
        gizmos.ray(t.translation, t.forward() * PLATFORM_DIM.z /2., Color::srgb(0., 0., 1.));
        gizmos.ray(t.translation, t.right() * PLATFORM_DIM.z /2., Color::srgb(1., 0., 0.));
        gizmos.ray(t.translation, t.up() * PLATFORM_DIM.z /2., Color::srgb(0., 1., 0.));
        // gizmos.ray(t.translation + t.forward() * PLATFORM_DIM.z /2., t.up() * 10., Color::srgb(1., 0., 0.));

    //     let r = Quat::from_rotation_y(90.0_f32.to_radians());
    //     // let v = r.mul_vec3(-Vec3::Z);
    //     let vp = r.mul_vec3(-Vec3::Z).reject_from(*t.up()).normalize();

    //     // gizmos.ray(t.translation, v * 10., Color::srgb(1., 0., 1.));
    //     gizmos.ray(t.translation, vp * PLATFORM_DIM.z / 2., Color::srgb(1., 0., 0.2));
            // gizmos.axes(*t, PLATFORM_DIM.z /2.);

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
            Self::Left => Quat::from_rotation_y(90.0_f32.to_radians()),
            Self::Right => Quat::from_rotation_y(-90.0_f32.to_radians()),
            Self::Up => Quat::from_rotation_x(30.0_f32.to_radians()),
            Self::Down => Quat::from_rotation_x(-30.0_f32.to_radians()),
            Self::Forward => Quat::IDENTITY,
            Self::Back =>  Quat::from_rotation_y(180.0_f32.to_radians()),
            Self::BackUp =>  Quat::from_rotation_y(180.0_f32.to_radians()) * Quat::from_rotation_x(30.0_f32.to_radians()),
            Self::BackDown =>  Quat::from_rotation_y(180.0_f32.to_radians()) * Quat::from_rotation_x(-30.0_f32.to_radians()),
            Self::LeftDown => Quat::from_rotation_y(90.0_f32.to_radians()) * Quat::from_rotation_x(-30.0_f32.to_radians()),
            Self::LeftUp => Quat::from_rotation_y(90.0_f32.to_radians()) * Quat::from_rotation_x(30.0_f32.to_radians()),
            Self::RightDown => Quat::from_rotation_y(-90.0_f32.to_radians()) * Quat::from_rotation_x(-30.0_f32.to_radians()),
            Self::RightUp => Quat::from_rotation_y(-90.0_f32.to_radians()) * Quat::from_rotation_x(30.0_f32.to_radians()),
        }
    }
}


fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlatformMaterial>>,
    // mut materials_s: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::from_size(PLATFORM_DIM));
    let color = materials.add(PlatformMaterial {color: Color::srgba (0.4, 0., 1., 0.1).into() }); 
    // let color = materials.add(Color::hsla(220., 1., 0.5, 0.4)); 

    
    let rotations = vec![
        PDir::Forward,
        PDir::Right,
        PDir::Down,
        PDir::Up,
        PDir::Right,
        PDir::Down,
        PDir::Up,
        // PDir::Left,
        // PDir::Up,
        // PDir::Down,
        // PDir::Right,
        // PDir::Up,
        // PDir::Down,
        // PDir::Right,
        // PDir::Up,
        // PDir::Down,
        // PDir::Down,
        // PDir::Up,
        // PDir::Right,
        // PDir::Left,
        // PDir::Forward,
        // PDir::Right,
        // PDir::Down,
        // PDir::Up,
        // PDir::Up,
        // PDir::Down,
        // PDir::Forward,
        // PDir::RightUp,
        // PDir::Down,
        // PDir::Forward,
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
            let step = if dir == trans.forward() { PLATFORM_DIM.z} else {PLATFORM_DIM.x};

            let connect_point = trans.translation + dir * step * 0.5;    
            trans.rotate_local(PDir::get_rotation(rs));
            pos = connect_point + trans.rotation.mul_vec3(-Vec3::Z * PLATFORM_DIM.z * 0.51);
        }

        trans = Transform::from_translation(pos).with_rotation(trans.rotation);
        cmd.spawn((
            trans,
            Mesh3d(mesh.clone()),
            MeshMaterial3d(color.clone()),
            Collider::cuboid(PLATFORM_DIM.x, PLATFORM_DIM.y, PLATFORM_DIM.z),
            RigidBody::Static,
            Platform,
            Name::new("Platform")
        ));
        
    }

    // cmd.spawn((
    //     Mesh3d(meshes.add(Cuboid::from_length(1.))),
    //     MeshMaterial3d(materials_s.add(Color::WHITE)),
    //     MarkerCommect
    // ));

}

// ---
#[derive(Component)]
struct MarkerCommect;

// ---

fn build_single(
    tr: Trigger<Build>,
    mut cmd: Commands,
    pt_q: Query<&Transform, Without<MarkerCommect>>,
    // m_q: Single<&mut Transform,  With<MarkerCommect>>
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
    let step = if [pt.forward(), pt.back()].contains(&face_to)  { PLATFORM_DIM.z} else {PLATFORM_DIM.x};

    let connect_point = pt.translation + *face_to * step * 0.5;
    let add = Quat::from_rotation_arc(*pt.forward(), *face_to).normalize();

    let rotation = pt.rotation * add *  match act {
        BuildAction::Up => PDir::get_rotation(&PDir::Up),
        BuildAction::Down => PDir::get_rotation(&PDir::Down),
        BuildAction::Forward => PDir::get_rotation(&PDir::Forward)
    };

    let pos = connect_point +  rotation.mul_vec3(-Vec3::Z *  PLATFORM_DIM.z * 0.5);
    println!("{:?}  {}  {}", *face_to, connect_point, pos);

    cmd.entity(*p_e)
    .clone_and_spawn_with(|b| {
        b.deny::<VisibilityClass>();
    })
    .insert((   
        Position::new(pos),
        Rotation(rotation)
    ))
    ;
}

// ---

