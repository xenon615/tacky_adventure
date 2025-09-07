use bevy::prelude::*;
use avian3d::prelude::*;
pub struct ExitPlugin;
impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, start)
        ;  
    }
}

fn start(
    mut cmd : Commands,
    assets: ResMut<AssetServer> 
) {
    cmd.spawn((
        SceneRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("models/exit.glb"))),
        Transform::from_xyz(0., 0., 100.),
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Name::new("Exit")
    ));
}