use bevy:: {
    prelude::*, 
    mesh::VertexAttributeValues, 
};

use noise::{BasicMulti, Perlin, NoiseFn};
use std::ops::{Add, Mul};
use crate::shared::{fibonacci_sphere, StageIndex};



pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(Update, movement)
        .add_systems(Update, opt_index_changed.run_if(resource_changed::<StageIndex>))
        ;
    }
}

// ---

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Orbit(f32, f32, f32);

#[derive(Resource)]
pub struct AsteroidMaterial(Handle<StandardMaterial>, Handle<Image>);



// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: ResMut<AssetServer>
) {

    let mut mesh = Sphere::new(10.).mesh().ico(4).unwrap();
    let noise = BasicMulti::<Perlin>::default();
    if let Some(VertexAttributeValues::Float32x3(verticis)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)  {
        for v in verticis.iter_mut() {
            let val = noise.get([v[0] as f64, v[1] as f64, v[2] as f64]);
            v.iter_mut().for_each(|c| {
                *c *= (val as f32).abs().add(1.).mul(1.1).clamp(0.9, 2.1)
            })
        };
    };

    mesh.compute_normals();

    let mat = materials.add(
        StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        }
    );

    cmd.insert_resource(AsteroidMaterial(mat.clone(), assets.load("textures/lava.png")));
    for f in fibonacci_sphere(8) {
        cmd.spawn((
            Transform::from_translation(f * 200.),
            Mesh3d(meshes.add(mesh.clone())),
            MeshMaterial3d(mat.clone()),
            Asteroid,
            Orbit(fastrand::f32(), fastrand::f32(), fastrand::f32())
        ));
    }

} 

// ---

fn movement(
    mut asteroid_q: Query<(&mut Transform, &Orbit), With<Asteroid>>,
    time: Res<Time>
) {
    for (mut t, o) in &mut asteroid_q {
        let orbit_rot_coef = time.delta_secs() * 0.05;
        t.rotate_around(Vec3::ZERO, Quat::from_euler(EulerRot::XZY, o.0 * orbit_rot_coef, o.1 * orbit_rot_coef, o.2 * orbit_rot_coef));
        let self_rot_coeff = time.delta_secs() * 0.5;
        t.rotate(Quat::from_euler(EulerRot::XYZ, self_rot_coeff, self_rot_coeff, self_rot_coeff));

    }
}


// ---

const OPTION_INDEX: usize = 1;

fn opt_index_changed(
    mh: Res<AsteroidMaterial>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    opt_index: Res<StageIndex>,
) {
    if opt_index.0 == OPTION_INDEX {
        if let Some(m) = materials.get_mut(&mh.0) {
            m.base_color_texture = Some(mh.1.clone());
        };
    }
} 
