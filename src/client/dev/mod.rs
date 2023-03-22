use bevy::{
    prelude::*,
    pbr::wireframe::WireframePlugin,
};
use bevy_egui::EguiPlugin;
use bevy_console::ConsolePlugin;

pub mod commands;

use commands::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(WireframePlugin)
            .add_plugin(ConsolePlugin)
            .add_plugin(DevCommandsPlugin);
    }
}