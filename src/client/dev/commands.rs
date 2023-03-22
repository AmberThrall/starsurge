use bevy::{
    prelude::*,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    pbr::wireframe::WireframeConfig,
};
use bevy_console::{reply, reply_ok, reply_failed, AddConsoleCommand, ConsoleCommand};
use clap::Parser;
use crate::Map;
use crate::MapRegistry;

/// Displays the current fps
#[derive(Parser, ConsoleCommand)]
#[command(name = "fps")]
struct FpsCommand;

fn fps_command(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut log: ConsoleCommand<FpsCommand>
) {
    if let Some(Ok(_)) = log.take() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }

        reply!(log, "{fps:.1} fps ({frame_time:.3} ms/frame)")
    }
}

/// Loads a map
#[derive(Parser, ConsoleCommand)]
#[command(name = "map")]
struct MapCommand {
    /// Map's id
    map_id: String,
}

fn map_command(
    map: Option<ResMut<Map>>,
    asset_server: Res<AssetServer>,
    map_registry: Res<MapRegistry>,
    mut log: ConsoleCommand<MapCommand>,
) {
    if let Some(Ok(MapCommand { map_id })) = log.take() {
        if let Some(mut map) = map {
            match map_registry.get(&map_id) {
                Some(path) => {
                    map.load(asset_server.load(path));
                    reply_ok!(log, "Loading map '{}' ({})...", map_id, path);
                },
                None => {
                    let mut available = String::new();
                    for (id, _) in map_registry.0.iter() {
                        if available.len() > 0 {
                            available.push_str(", ");
                        }
                        available.push_str(&id);
                    }
                    reply_failed!(log, "Unknown map '{}'. Available maps: {}", map_id, available);
                }
            }
        }
    }
}

/// Reloads a specified asset
#[derive(Parser, ConsoleCommand)]
#[command(name = "reload_asset")]
struct ReloadAssetCommand {
    /// Asset path
    path: String,
}

fn reload_asset_command(
    asset_server: Res<AssetServer>,
    mut log: ConsoleCommand<ReloadAssetCommand>,
) {
    if let Some(Ok(ReloadAssetCommand { path })) = log.take() {
        asset_server.reload_asset(&path);
        reply_ok!(log, "Reloading '{}'", path);
    }
}

/// Toggles wireframe display
#[derive(Parser, ConsoleCommand)]
#[command(name = "wireframe")]
struct WireframeCommand;

fn wireframe_command(
    mut wireframe_config: ResMut<WireframeConfig>,
    mut log: ConsoleCommand<WireframeCommand>,
) {
    if let Some(Ok(_)) = log.take() {
        wireframe_config.global = !wireframe_config.global;
        reply!(log, "Turning wireframe {}", if wireframe_config.global { "on" } else { "off" });
    }
}

pub struct DevCommandsPlugin;

impl Plugin for DevCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<FpsCommand, _>(fps_command)
            .add_console_command::<MapCommand, _>(map_command)
            .add_console_command::<ReloadAssetCommand, _>(reload_asset_command)
            .add_console_command::<WireframeCommand, _>(wireframe_command);
    }
}