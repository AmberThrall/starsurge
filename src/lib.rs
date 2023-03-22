use bevy::prelude::*;

pub mod bevy_config;
pub mod terrain;

pub use crate::bevy_config::BevyConfigPlugin;
pub use crate::terrain::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BevyConfigPlugin)
            .add_plugin(TerrainPlugin);
    }
}