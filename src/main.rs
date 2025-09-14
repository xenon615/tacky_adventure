#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_egui::EguiPlugin;



use avian3d::{
    PhysicsPlugins,
    // debug_render::PhysicsDebugPlugin
};

use crate::shared::GameStage;

mod shared;
mod camera;
mod env;
mod eyes;
mod player;
// mod city;
mod platform;
mod lift;
mod ui;
mod effects;
mod exit;
mod monologue;
mod help;
mod aimer;

fn main() {
    let mut app = App::new();
    app
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        // PhysicsDebugPlugin::default(),
        HanabiPlugin,
        camera::CameraPlugin,
        env::EnvPlugin,
        // eyes::EyesPlugin,
        player::PlayerPlugin,
        platform::PlatformPlugin,
        lift::LiftPlugin,
        ui::UiPlugin,
        exit::ExitPlugin,
        monologue::MonologuePlugin,
        help:: HelpPlugin,
        aimer::AimerPlugin
    ))
    // .add_plugins(EguiPlugin::default() )
    // .add_plugins(WorldInspectorPlugin::new())
    .init_state::<GameStage>()
    .run()
    ;
}