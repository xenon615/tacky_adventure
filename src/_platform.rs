use bevy::{
    gizmos, pbr::Material, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}
};
use avian3d::prelude::*;

// use crate::player::Player;

pub struct PlatformPlugin;
impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<PlatformMaterial>::default())
        .add_systems(Startup, startup)
        .add_systems(Update, gismos)
        ;
    }
}

// ---

pub const PLATFORM_DIM: Vec3 = Vec3::new(10., 0.1, 10.);

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

fn gismos(
    mut gizmos: Gizmos,
    t_q: Query<&Transform, With<Platform>>
) {
    for t in &t_q {
        gizmos.ray(t.translation, t.forward() * PLATFORM_DIM.z /2., Color::WHITE);
    //     // gizmos.ray(t.translation + t.forward() * PLATFORM_DIM.z /2., t.up() * 10., Color::srgb(1., 0., 0.));

    //     let r = Quat::from_rotation_y(90.0_f32.to_radians());
    //     // let v = r.mul_vec3(-Vec3::Z);
    //     let vp = r.mul_vec3(-Vec3::Z).reject_from(*t.up()).normalize();

    //     // gizmos.ray(t.translation, v * 10., Color::srgb(1., 0., 1.));
    //     gizmos.ray(t.translation, vp * PLATFORM_DIM.z / 2., Color::srgb(1., 0., 0.2));
        //     gizmos.axes(*t, PLATFORM_DIM.z /2.);

    }
}

// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlatformMaterial>>,
) {
    let mesh = meshes.add(Cuboid::from_size(PLATFORM_DIM));
    let color = materials.add(PlatformMaterial {color: Color::srgba (0., 0., 0., 0.1).into() }); 
    
    // let rotations = vec![
    //     Quat::IDENTITY,
    //     Quat::from_rotation_y(-90.0_f32.to_radians()),
    //     Quat::from_rotation_x(15.0_f32.to_radians()),
    //     Quat::from_rotation_x(-15.0_f32.to_radians()),
    //     Quat::from_rotation_x(-15.0_f32.to_radians()) ,
    //     Quat::from_rotation_x(15.0_f32.to_radians()),
    //     Quat::IDENTITY,
    //     Quat::from_rotation_y(90.0_f32.to_radians()),
    //     Quat::from_rotation_x(15.0_f32.to_radians()),
    //     Quat::from_rotation_x(-15.0_f32.to_radians()),
    //     Quat::from_rotation_x(-15.0_f32.to_radians()) ,
    //     Quat::from_rotation_x(15.0_f32.to_radians()),
    //     Quat::IDENTITY,
    //     Quat::from_rotation_y(90.0_f32.to_radians()),
    //     Quat::from_rotation_x(15.0_f32.to_radians()),
    //     Quat::from_rotation_x(-15.0_f32.to_radians()),
    //     Quat::from_rotation_x(-15.0_f32.to_radians()) ,
    //     Quat::from_rotation_x(15.0_f32.to_radians()),
    //     Quat::IDENTITY,
    //     Quat::from_rotation_y(90.0_f32.to_radians()),
    //     Quat::from_rotation_x(15.0_f32.to_radians()),
    //     Quat::from_rotation_x(-15.0_f32.to_radians()),
    //     Quat::from_rotation_x(15.0_f32.to_radians()) ,
    //     Quat::from_rotation_x(-15.0_f32.to_radians()),
    // ];
    let rotations = vec![
        Quat::IDENTITY,
        Quat::from_rotation_y(-90.0_f32.to_radians()),
        Quat::from_rotation_y(-90.0_f32.to_radians()).mul_quat(Quat::from_rotation_x(15.0_f32.to_radians())),
        Quat::from_rotation_x(-15.0_f32.to_radians()),
        Quat::IDENTITY,
        Quat::from_rotation_y(-90.0_f32.to_radians()),
        Quat::from_rotation_x(30.0_f32.to_radians()),
        Quat::from_rotation_x(-30.0_f32.to_radians()),
        Quat::from_rotation_y(-90.0_f32.to_radians()).mul_quat(Quat::from_rotation_x(15.0_f32.to_radians())),
        Quat::IDENTITY,
        // Quat::from_rotation_x(-15.0_f32.to_radians()),
        // Quat::from_rotation_y(-90.0_f32.to_radians()).mul_quat(Quat::from_rotation_x(15.0_f32.to_radians())),
        // Quat::from_rotation_x(-15.0_f32.to_radians()),
        // Quat::from_rotation_x(-15.0_f32.to_radians()) ,
        // Quat::from_rotation_x(15.0_f32.to_radians()),
        // Quat::IDENTITY,
        // Quat::from_rotation_y(-90.0_f32.to_radians()),
        // Quat::from_rotation_y(-90.0_f32.to_radians()),
        // Quat::from_rotation_x(15.0_f32.to_radians()),
        // Quat::from_rotation_x(-15.0_f32.to_radians()),
        // Quat::from_rotation_x(-15.0_f32.to_radians()) ,
        // Quat::from_rotation_x(15.0_f32.to_radians()),
        // Quat::IDENTITY,
        // Quat::from_rotation_y(90.0_f32.to_radians()),
        // Quat::from_rotation_x(15.0_f32.to_radians()),
        // Quat::from_rotation_x(-15.0_f32.to_radians()),
        // Quat::from_rotation_x(-15.0_f32.to_radians()) ,
        // Quat::from_rotation_x(15.0_f32.to_radians()),
        // Quat::IDENTITY,
        // Quat::from_rotation_y(90.0_f32.to_radians()),
        // Quat::from_rotation_x(15.0_f32.to_radians()),
        // Quat::from_rotation_x(-15.0_f32.to_radians()),
        // Quat::from_rotation_x(15.0_f32.to_radians()) ,
        // Quat::from_rotation_x(-15.0_f32.to_radians()),
    ];

    let mut pos = Vec3::ZERO;
    let step = Vec3::Z * PLATFORM_DIM.z;
    let mut total_rotation = Quat::IDENTITY;
    let mut trans = Transform::IDENTITY;

    for (idx, r) in  rotations.iter().enumerate() {
        // total_rotation.z = 0.;
        total_rotation = total_rotation
        // .normalize()
        .mul_quat(*r).normalize();
        if idx > 0 {
            let to = r.mul_vec3(*trans.forward()).normalize().reject_from(*trans.up()).normalize();
            let connect_point = trans.translation + to * PLATFORM_DIM.z * 0.5;
            pos = connect_point + total_rotation.mul_vec3(-step * 0.5);
            // println!("{pos}");
        
        }

        trans = Transform::from_translation(pos).with_rotation(total_rotation);
        cmd.spawn((
            trans,
            Mesh3d(mesh.clone()),
            MeshMaterial3d(color.clone()),
            Collider::cuboid(PLATFORM_DIM.x, PLATFORM_DIM.y, PLATFORM_DIM.z),
            RigidBody::Static,
            Platform
        ));
        
    }
    
}
