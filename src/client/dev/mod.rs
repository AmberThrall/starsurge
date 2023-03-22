use bevy::{
    prelude::*,
    pbr::wireframe::{Wireframe, WireframePlugin},
};
use bevy_egui::EguiPlugin;
use bevy_console::ConsolePlugin;

pub mod commands;

use commands::*;

#[derive(Resource)]
pub struct WireframeStatus(bool);

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WireframeStatus(false))
            .add_plugin(EguiPlugin)
            .add_plugin(WireframePlugin)
            .add_plugin(ConsolePlugin)
            .add_plugin(DevCommandsPlugin)
            .add_system(wireframe_handle);
    }
}

fn wireframe_handle(
    mut commands: Commands,
    query: Query<Entity>,
    wireframe_status: Res<WireframeStatus>,
) {
    if wireframe_status.is_changed() {
        for entity in query.iter() {
            if wireframe_status.0 {
                commands.entity(entity).insert(Wireframe);
            } else {
                commands.entity(entity).remove::<Wireframe>();
            }
        }
    }
}