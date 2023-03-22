use bevy::{
    prelude::*,
};
use bevy_common_assets::ron::RonAssetPlugin;
use super::{
    Terrain,
    TerrainBuilder
};

/// Comopnent to mark entities that should not be unloaded on map change.
#[derive(Component)]
pub struct DontUnload;

/// Component for terrain quad position. (0,0) is the center of the map.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x, y,
        }
    }
}

/// Helper component for models.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Model {
    pub path: String,
}

impl Model {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
        }
    }
}

/// MapAsset's current state.
pub enum MapAssetState {
    NotLoaded,
    Loading,
    Loaded,
}

impl Default for MapAssetState {
    fn default() -> Self {
        MapAssetState::NotLoaded
    }
}

/// Resource for handling the current map.
#[derive(Resource, Default)]
pub struct Map {
    pub data: Handle<MapData>,
    name: String,
    state: MapAssetState,
}

impl Map {
    /// Creates a map from a handle to MapData.
    pub fn from_handle(handle: Handle<MapData>) -> Self {
        Self {
            data: handle,
            name: String::default(),
            state: MapAssetState::NotLoaded,
        }
    }

    /// Loads the specified map.
    pub fn load(&mut self, handle: Handle<MapData>) {
        self.data = handle;
        self.name = String::default();
        self.state = MapAssetState::Loading;
    }

    /// Reloads the current map.
    pub fn reload(&mut self) {
        self.state = MapAssetState::Loading;
    }

    /// Returns the current map name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

/// Map's terrain data.
#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "018c7922-ab74-4225-885a-0f91271fd28d"]
pub struct MapTerrainData {
    pub heightmap: String,
    pub colormap: String,
    pub max_altitude: f32,
}

/// Map's data structure.
#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "e222d1d1-1044-4a49-8d1f-80d796f8645b"]
pub struct MapData {
    /// Name of the map
    pub name: String,
    /// Terrain map data
    pub terrain: MapTerrainData,
    /// File name of a bevy dynamic scene file.
    pub scene_path: String,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Model>()
            .register_type::<Position>()
            .insert_resource(Map::default())
            .add_plugin(RonAssetPlugin::<MapData>::new(&["map", "map.ron"]))
            .add_system(map_loader)
            .add_system(load_models)
            .add_system(handle_positions);
    }
}

fn map_loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_data: Res<Assets<MapData>>,
    map_res: Option<ResMut<Map>>,
    unload_query: Query<Entity, (Without<Window>, Without<DontUnload>)>,
) {
    if let Some(mut map) = map_res {
        match map.state {
            MapAssetState::Loaded => {},
            MapAssetState::Loading => {
                // Unload the scene
                for entity in unload_query.iter() {
                    commands.entity(entity).despawn();
                }

                // Update the map's state.
                map.state = MapAssetState::NotLoaded;
            },
            MapAssetState::NotLoaded => {
                if let Some(data) = map_data.get(&map.data) {
                    // Spawn the terrain
                    commands.spawn((
                        TerrainBuilder {
                            heightmap: asset_server.load(&data.terrain.heightmap),
                            colormap: asset_server.load(&data.terrain.colormap),
                            max_altitude: data.terrain.max_altitude,
                        },
                        Name::new("Terrain"),
                    ));
                
                    // Load the scene
                    commands.spawn(DynamicSceneBundle {
                        scene: asset_server.load(&data.scene_path),
                        ..default()
                    });

                    // Update the state and clean up
                    map.name = data.name.clone();
                    map.state = MapAssetState::Loaded;
                }
            }
        }
    }
}

fn load_models(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Model), Changed<Model>>,
) {
    for (entity, model) in query.iter() {
        commands.entity(entity).insert(SceneBundle {
            scene: asset_server.load(format!("{}#Scene0", model.path)),
            ..default()
        });
    }
}

fn handle_positions(
    terrains: Query<&Terrain>,
    mut query: Query<(&mut Transform, &Position)>
) {
    if let Ok(terrain) = terrains.get_single() {
        for (mut transform, pos) in query.iter_mut() {
            let world_pos = terrain.get_world_position(pos);
            transform.translation = world_pos;
        }
    }
}