use bevy::{
    prelude::*,
    pbr::wireframe::WireframePlugin,
};
use bevy_egui::EguiPlugin;
use bevy_console::ConsolePlugin;
use crate::Map;

pub mod commands;

use commands::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(WireframePlugin)
            .add_plugin(ConsolePlugin)
            .add_plugin(DevCommandsPlugin)
            .add_system(reload_map_hotkey);
    }
}

fn reload_map_hotkey(
    keys: Res<Input<KeyCode>>,
    map: Option<ResMut<Map>>,
) {
    if keys.just_pressed(KeyCode::F12) {
        info!("Reloading curent map...");
        if let Some(mut map) = map {
            map.reload();
        }
    }
}