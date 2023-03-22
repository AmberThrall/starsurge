use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin},
    prelude::*, 
};
use starsurge::*;

mod camera;
use camera::*;

mod dev;
use dev::*;

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(CameraPlugin)
        .add_plugin(DevPlugin)
        .add_startup_system(setup)
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