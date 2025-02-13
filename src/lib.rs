#![allow(clippy::type_complexity)]

mod asset_loader;
mod menu;
mod interactables;
mod main_camera;

use crate::asset_loader::AssetLoaderPlugin;
use crate::main_camera::MainCameraPlugin;
use crate::menu::MenuPlugin;
use crate::interactables::purple_mushroom::PurpleMushroomPlugin;
use crate::interactables::spore_print::SporePrintPlugin;
use crate::interactables::circle::CirclePlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            AssetLoaderPlugin,
            MenuPlugin,
            MainCameraPlugin,
            // TODO organize "draggable thing" plugins so they all get loaded in their own plugin
            PurpleMushroomPlugin,
            SporePrintPlugin,
            CirclePlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
