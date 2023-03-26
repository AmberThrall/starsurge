use bevy::prelude::*;

pub mod bevy_config;
pub mod terrain;
pub mod map;
pub mod map_registry;
pub mod spawner;
pub mod input;
pub mod ai;
pub mod movement;

pub use crate::bevy_config::BevyConfigPlugin;
pub use crate::terrain::*;
pub use crate::map::*;
pub use crate::map_registry::*;
pub use crate::spawner::*;
pub use crate::input::*;
pub use crate::ai::*;
pub use crate::movement::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let mut map_registry = MapRegistry::default();
        map_registry.register_map("test_map", "maps/test_map.map.ron");
        map_registry.register_map("test_map2", "maps/test_map2.map.ron");

        app.add_plugin(BevyConfigPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(TerrainPlugin)
            .add_plugin(SpawnerPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(AiPlugin)
            .insert_resource(map_registry);
    }
}