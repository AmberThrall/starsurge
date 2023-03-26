use bevy::{
    prelude::*,
};
use bevy_common_assets::ron::RonAssetPlugin;
use crate::{
    Terrain,
    TerrainBuilder,
    GameObject,
    SpawnEvent,
};

/// Comopnent to mark entities that should not be unloaded on map change.
#[derive(Component, Debug)]
pub struct DontUnload;

/// Component for terrain quad position. (0,0) is the center of the map.
#[derive(Component, Debug, Reflect, serde::Deserialize, bevy::reflect::TypeUuid, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[reflect(Component)]
#[uuid = "48613d5d-f1d2-46b8-8d9f-026c27fe8700"]
pub struct Position {
    pub x: i32,
    pub y: i32,
    #[serde(default)]
    pub layer: i32,
}

impl Position {
    pub const ZERO: Position = Position { x: 0, y: 0, layer: 0 };

    pub fn new(x: i32, y: i32, layer: i32) -> Self {
        Self {
            x, y, layer,
        }
    }

    pub fn distance_squared(&self, other: &Position) -> u32 {
        ((self.x - other.x) * (self.x - other.x) +
            (self.y - other.y) * (self.y - other.y) +
            (self.layer - other.layer) * (self.layer - other.layer)) as u32
    }

    pub fn distance(&self, other: &Position) -> f32 {
        (self.distance_squared(other) as f32).sqrt()
    }
}


/// Component that moves an entity to a specific position only once.
#[derive(Component, Debug, Reflect, serde::Deserialize, bevy::reflect::TypeUuid, Default, Copy, Clone, PartialEq, Eq)]
#[reflect(Component)]
#[uuid = "f99dfd68-64bb-4abe-b44a-edfb07e83e80"]
pub struct SpawnPosition(pub Position);

/// Component for an entities angle about the y-axis.
#[derive(Component, Debug, Reflect, Default, Copy, Clone, PartialEq)]
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
pub struct ObjectData {
    /// Object's grid position
    #[serde(default)]
    pub position: Position,

    /// Angle of entity about y-axis
    #[serde(default)]
    pub angle: f32,

    /// Object data
    #[serde(default)]
    pub object: GameObject,
}

impl ObjectData {
    pub fn to_spawn_event(self) -> SpawnEvent {
        SpawnEvent {
            position: self.position,
            angle: self.angle,
            object: self.object,
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
    pub objects: Vec<ObjectData>,

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
            .add_system(handle_spawn_positions)
            .add_system(handle_angles);
    }
}

fn map_loader(
    mut commands: Commands,
    mut spawn_ev: EventWriter<SpawnEvent>,
    asset_server: Res<AssetServer>,
    mut map_data: ResMut<Assets<MapData>>,
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

                // Reload the map files
                if let Some(map_path) = asset_server.get_handle_path(&map.data) {
                    asset_server.reload_asset(map_path.path()); // .map.ron file
                }
                
                // For some reason bevy crashes when reloading a DynamicScene.
                // if let Some(data) = map_data.get(&map.data) {
                //     if let Some(path) = &data.dynamic_scene {
                //         asset_server.reload_asset(path); // .scn.ron file
                //     }
                // }

                // Update the map's state.
                map.state = MapAssetState::NotLoaded;
            },
            MapAssetState::NotLoaded => {
                if let Some(data) = map_data.get_mut(&map.data) {
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
                    while let Some(object_data) = data.objects.pop() {
                        spawn_ev.send(object_data.to_spawn_event());
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

fn handle_spawn_positions(
    mut commands: Commands,
    terrains: Query<&Terrain>,
    mut query: Query<(Entity, &mut Transform, &SpawnPosition)>
) {
    if let Ok(terrain) = terrains.get_single() {
        for (entity, mut transform, pos) in query.iter_mut() {
            let world_pos = terrain.get_world_position(&pos.0);
            transform.translation = world_pos;

            // Remove the SpawnPosition component.
            commands.entity(entity).remove::<SpawnPosition>();
        }
    }
}

fn handle_angles(
    mut query: Query<(&mut Transform, &Angle)>
) {
    for (mut transform, angle) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(angle.0.to_radians());
    }
}