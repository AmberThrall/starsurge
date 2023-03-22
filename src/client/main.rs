use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*, 
};
use starsurge::*;

mod camera;
use camera::*;

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin)
        .add_startup_system(setup)
        .add_system(reload_map)
        .add_system(log_query)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Light
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
        Name::new("Sun"),
        DontUnload,
    ));

    // Camera
    commands.spawn((
        Camera3dBundle::default(),
        MainCamera::default(),
        Name::new("Camera"),
        DontUnload,
    ));

    // Map
    commands.insert_resource(Map::from_handle(asset_server.load("maps/test_map.map.ron")));
}

fn reload_map(
    keys: Res<Input<KeyCode>>,
    map: Option<ResMut<Map>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(mut map) = map {
        if keys.just_pressed(KeyCode::F12) {
            info!("Loading map...");
            map.load(asset_server.load("maps/test_map2.map.ron"));
        }
    }
}

fn log_query(world: &World) {
    let keys = world.get_resource::<Input<KeyCode>>().unwrap();

    if keys.just_pressed(KeyCode::F11) {
        let mut text = "Entities:".to_string();

        for entity_ref in world.iter_entities() {
            if world.get::<Parent>(entity_ref.id()).is_none() {
                text.push_str(&format!("\n{},", log_query_inner(entity_ref.id(), world, 0)));
            }
        }

        info!("{}", text);
    }
}

fn log_query_inner(root: Entity, world: &World, indent: usize) -> String {
    let mut indent_str = String::default();
    for _ in 0..indent {
        indent_str.push_str("  ");
    }

    let mut text = match world.get::<Name>(root) {
        Some(name) => format!("{}+ {:?} ({})", indent_str, root, name.as_str()),
        None => format!("{}+ {:?}", indent_str, root),
    };

    let components = world.inspect_entity(root);
    for component in components {
        text.push_str(&format!("\n{}  - {}", indent_str, component.name()));
    }

    match world.get::<Children>(root) {
        Some(children) => {
            for child in children.iter() {
                text.push_str(&format!("\n{}", log_query_inner(*child, world, indent + 1)));
            }
        },
        None => {},
    }

    text
}