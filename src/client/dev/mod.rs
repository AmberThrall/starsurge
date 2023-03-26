use bevy::{
    prelude::*,
    pbr::wireframe::WireframePlugin,
};
use bevy_egui::EguiPlugin;
use bevy_console::ConsolePlugin;
use bevy_prototype_debug_lines::*;
use crate::{
    Terrain, Position, Map, MousePosition, Path,
};

pub mod commands;

use commands::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(WireframePlugin)
            .add_plugin(ConsolePlugin)
            .add_plugin(DevCommandsPlugin)
            .add_plugin(DebugLinesPlugin::with_depth_test(true))
            .add_system(reload_map_hotkey)
            .add_system(draw_paths);
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

fn draw_paths(
    terrain_query: Query<&Terrain>,
    path_query: Query<&Path>,
    mut lines: ResMut<DebugLines>,
) {
    if let Ok(terrain) = terrain_query.get_single() {
        // Draw each path
        for path in path_query.iter() {

            let mut last_node = None;
            for node in path.0.iter() {
                if last_node.is_none() {
                    last_node = Some(terrain.get_world_position(&node));
                } else {
                    let next = terrain.get_world_position(&node);
                    lines.line_colored(last_node.unwrap(), next, 0.0, Color::CYAN);
                    last_node = Some(next);
                }
            }
        }
    }
}