use avian3d::math::PI;
use bevy::{
    prelude::*,
    math::Affine2,
    image::{
        ImageAddressMode,
        ImageLoaderSettings,
        ImageSampler,
        ImageSamplerDescriptor,
        ImageFilterMode
    }
};
pub struct CityPlugin;
impl Plugin for CityPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        ;
    }
}

use crate::env::FIELD_SIZE;

// -- 

// fn startup(
//     mut cmd: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>
// ) {
    
//     let b_count = 25;
//     let circle_len = 2. * PI * FIELD_SIZE;
//     let x_size = circle_len / (b_count + 1) as f32;
//     let b_mesh = meshes.add(Cuboid::from_size(Vec3::new(x_size, 50., 10.)));
//     let b_mat = materials.add(
//         StandardMaterial {
//             base_color: Color::WHITE,
//             ..default()
//         }
//     );
//     let angle_step = (360. / b_count as f32 ).to_radians();
//     let mut angle = 0.;
//     let mut pos = Vec3::new(0., 0., FIELD_SIZE);
//     let b_step = Vec3::new(x_size, 0., 0.);
//     for i in 0 .. b_count {
//         cmd.spawn((
//             Mesh3d(b_mesh.clone()),
//             MeshMaterial3d(b_mat.clone()),
//             Transform::from_translation(pos).looking_at(Vec3::ZERO, Vec3::Y),
//         ));
//         angle +=  angle_step;
//         pos += Quat::from_rotation_y(angle).mul_vec3(b_step);
//     } 
// }

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: ResMut<AssetServer>
) {
    
    let b_count = 25;
    let circle_len = 2. * PI * FIELD_SIZE;
    let x_size = circle_len / (b_count + 1) as f32;
    let angle_step = (360. / b_count as f32 ).to_radians();
    let one = Vec3::new(0., 0., FIELD_SIZE);
    let max_level_index = 5;
    
    let b_meshes: Vec<Handle<Mesh>> = (0 .. max_level_index)
        .map(| i | {
            meshes.add(Cuboid::from_size(Vec3::new(x_size, (i + 1) as f32 * 25., 30.)))
        })
        .collect()
    ; 
    
    let texture =  Some(
        assets.load_with_settings(
            "textures/windows.png",
            |s: &mut _| {
                *s = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        address_mode_w: ImageAddressMode::Repeat,
                        ..default()
                    }),
                    ..default()
                }
            },
        )
    ); 



    let b_mat = materials.add(
        StandardMaterial {
            // base_color: Color::WHITE,
            emissive: Color::hsl(50., 1., 0.4).into(),
            emissive_texture: texture.clone(),
            // base_color_texture: texture.clone(),
            uv_transform: Affine2::from_scale(Vec2::new(8., 8.)),
            ..default()
        }
    );
    
    let mut angle = 0.;
    for i in 0 .. b_count {
        let l_num = fastrand::i8(0 .. max_level_index);
        let l_height = (l_num + 1) as f32 * 25.;
        cmd.spawn((
            Mesh3d(b_meshes[l_num as usize].clone()),
            MeshMaterial3d(b_mat.clone()),
            Transform::from_translation(Quat::from_rotation_y(angle).mul_vec3(one.with_y(l_height / 2.))).looking_at(Vec3::ZERO.with_y(l_height / 2.), Vec3::Y),
        ));
        angle +=  angle_step;
    } 
}