#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{
    prelude::*,
    render::{
        RenderApp,
        batching::gpu_preprocessing::{GpuPreprocessingSupport, GpuPreprocessingMode}

    }
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_egui::EguiPlugin;



use avian3d::{
    PhysicsPlugins,
    // debug_render::PhysicsDebugPlugin
};

mod shared;
mod camera;
mod env;
mod eyes;
mod player;
// mod city;
mod platform;
mod lift;
mod ui;

fn main() {
    let mut app = App::new();
    app
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        // PhysicsDebugPlugin::default(),
        camera::CameraPlugin,
        env::EnvPlugin,
        // eyes::EyesPlugin,
        player::PlayerPlugin,
        platform::PlatformPlugin,
        lift::LiftPlugin,
        ui::UiPlugin
    ))
    .add_plugins(EguiPlugin::default() )
    .add_plugins(WorldInspectorPlugin::new())
    .run()
    ;
}