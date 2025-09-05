use bevy::{color::palettes::css, prelude::*};
use avian3d::prelude::*;

// use crate::shared::CoLayer;

pub struct EnvPlugin;
impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        ;
    }
}

// ---

fn startup(
    mut cmd:Commands,
) {

    cmd.spawn((
        DirectionalLight {
            illuminance: 15000., 
            ..default()
        },
        Transform::from_xyz(0., 10., 0.).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}