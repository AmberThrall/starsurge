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
#[derive(Component, Reflect, serde::Deserialize, bevy::reflect::TypeUuid, Default, Copy, Clone, PartialEq, Eq)]
#[reflect(Component)]
#[uuid = "48613d5d-f1d2-46b8-8d9f-026c27fe8700"]
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

/// Component for an entities angle about the y-axis.
#[derive(Component, Reflect, Default, Copy, Clone, PartialEq)]
pub struct Angle(pub f32);

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
    ChangingMap,
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
        self.state = MapAssetState::ChangingMap;
    }

    /// Reloads the current map.
    pub fn reload(&mut self) {
        self.state = MapAssetState::ChangingMap;
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

/// Maps' entity data structure
#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "d1faf050-efdb-474e-85cc-353e8d8c6e00"]
pub struct MapEntityData {
    /// Entity's grid position
    #[serde(default)]
    pub position: Position,

    /// Optional model path
    #[serde(default)]
    pub model: Option<String>,

    /// Angle of entity about y-axis
    #[serde(default)]
    pub angle: f32,
}

impl MapEntityData {
    pub fn spawn(&self, commands: &mut Commands) {
        let entity = commands.spawn_empty().id();
        commands.entity(entity).insert(self.position);
        commands.entity(entity).insert(Angle(self.angle));
        if let Some(model) = &self.model {
            commands.entity(entity).insert(Model::new(&model));
        }
    }
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
    pub entities: Vec<MapEntityData>,

    /// Optional dynamic scene to load
    #[serde(default)]
    pub dynamic_scene: Option<String>,
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
            .add_system(handle_positions)
            .add_system(handle_angles);
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
            MapAssetState::ChangingMap => {
                // Despawn the scene
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
                    for entity in data.entities.iter() {
                        entity.spawn(&mut commands);
                    }

                    // Load the dynamic scene?
                    if let Some(path) = &data.dynamic_scene {
                        commands.spawn(DynamicSceneBundle {
                            scene: asset_server.load(path),
                            ..default()
                        });
                    }

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

fn handle_angles(
    mut query: Query<(&mut Transform, &Angle)>
) {
    for (mut transform, angle) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(angle.0);
    }
}