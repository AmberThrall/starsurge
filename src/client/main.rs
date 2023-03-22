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
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn((
        Camera3dBundle::default(),
        MainCamera::default(),
    ));

    // Terrain
    let heightmap = asset_server.load("Heightmap.png");
    let colormap = asset_server.load("Colormap.png");
    commands.spawn((
        TerrainBuilder {
            heightmap,
            colormap,
            max_altitude: 10.0,
        },
    ));

    // Trees
    for y in -5..5 {
        for x in -5..5 {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("models/tree_pineTallD_detailed.glb#Scene0"),
                    ..default()
                },
                Position::new(x, y),
            ));
        }
    }
}