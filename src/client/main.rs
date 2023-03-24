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
        .add_startup_system(setup)
        .add_system(click_response);
    
    #[cfg(feature = "dev")]
    app.add_plugin(DevPlugin);

    app.run();
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
        bevy_mod_raycast::RaycastSource::<CameraRaycastSet>::new_transform_empty(),
        Name::new("Camera"),
        DontUnload,
    ));

    // Map
    commands.insert_resource(Map::from_handle(asset_server.load("maps/test_map.map.ron")));
}

fn click_response(
    mut click_event: EventReader<MouseClickEvent>,
) {
    for ev in click_event.iter() {
        info!("Just clicked: {:?}", ev);
    }
}