#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_egui::EguiPlugin;



use avian3d::{
    PhysicsPlugins,
    // debug_render::PhysicsDebugPlugin
};

use crate::shared::{
    GameState,
    NotReady,
    OptionIndex
};

mod shared;
mod camera;
mod env;
// mod city;
mod ui;
mod player;
mod platform;
mod exit;
mod monologue;
mod help;
mod lift;
mod eye;
mod effects;
mod aimer;
mod virus;
mod missile;
mod damage;
mod asteroid;
mod messages;
mod info;

fn main() {
    let mut app = App::new();
    app
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        HanabiPlugin,
        camera::CameraPlugin,
        env::EnvPlugin,

    ))
    .add_plugins((
        ui::UiPlugin,
        player::PlayerPlugin,
        platform::PlatformPlugin,
        exit::ExitPlugin,

        monologue::MonologuePlugin,
        help:: HelpPlugin,
        eye::EyesPlugin,
        lift::LiftPlugin,

        aimer::AimerPlugin,
        virus::VirusPlugin,
        missile::MissilePlugin,
        damage::DamagePlugin,

        asteroid::AsteroidPlugin,
        messages::InfoPlugin,
        info::InfoPlugin
    ))
    // .add_plugins(PhysicsDebugPlugin::default())
    // .add_plugins(EguiPlugin::default() )
    // .add_plugins(WorldInspectorPlugin::new())
    .init_state::<GameState>()
    .init_resource::<OptionIndex>()
    .add_systems(Update, check_ready.run_if(in_state(GameState::Loading)))
    
    .run()
    ;
}

// ---

fn check_ready(
    not_ready_q: Query<&NotReady>,
    mut next: ResMut<NextState<GameState>>     
) {
    if not_ready_q.is_empty() {
        next.set(GameState::Game);
    }
}

