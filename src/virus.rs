use bevy:: {
    prelude::*,
    pbr::Material, 
    render::{
        render_resource::{AsBindGroup, ShaderRef}, 
        mesh::{SphereKind, VertexAttributeValues}
        // view::VisibilityClass
    }
};
use std::ops::{Add, Mul};


use crate::shared::{GameStage, fibonacci_sphere, closest};

pub struct VirusPlugin;
impl Plugin for VirusPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<VirusMaterial>::default())
        .add_systems(OnEnter(GameStage::Virus), startup)
        ;
    }
}

// ---

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct VirusMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    stage_index: u32
}

impl Material for VirusMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/platform.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}

// ---

#[derive(Component, Clone)]
pub struct Virus;

#[derive(Resource)]
pub struct VirusMaterialHandle(Handle<VirusMaterial>);

// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mut mesh =  Mesh::from(Sphere::new(1.).mesh().kind(SphereKind::Ico { subdivisions: 6 }));
    let Some(VertexAttributeValues::Float32x3(verticis)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) else {
        return;
    };

    for point in fibonacci_sphere(32) {
        let scale = fastrand::f32().add(1.).mul(1.1).clamp(1.1, 2.);
        closest(verticis, point, scale);
    }

    mesh.compute_normals();

    cmd.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::hsl(130., 1., 0.5))),
        Virus, 
    ));

} 