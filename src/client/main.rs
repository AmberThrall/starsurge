use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use starsurge::*;

mod camera;
use camera::*;

#[cfg(feature = "dev")]
mod dev;
#[cfg(feature = "dev")]
use dev::*;

fn main() {
    let mut app = App::new();
    app.add_plugin(GamePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(CameraPlugin)
        .add_startup_system(setup);
    
    #[cfg(feature = "dev")]
    app.add_plugin(DevPlugin);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut spawn_ev: EventWriter<SpawnEvent>,
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

    // Test spawner
    spawn_ev.send(SpawnEvent {
        object: Object::Decor("models/tree_palmTall.glb".to_string()),
        position: Position::new(3, 3),
        ..default()
    });
}