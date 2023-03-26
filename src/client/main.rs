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

#[derive(Component)]
pub struct Player;

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
    mut meshes: ResMut<Assets<Mesh>>,
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

    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Capsule {
                radius: 0.15,
                depth: 1.0,
                ..default()
            }.into()),
            ..default()
        },
        Player,
        SpawnPosition(Position::ZERO),
        Angle(0.0),
        PathFollower::default(),
        PathBuilder {
            target: Position::new(3, 3, 0),
        },
        DontUnload,
    ));
}

fn click_response(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    mut click_event: EventReader<MouseClickEvent>,
) {
    for ev in click_event.iter() {
        info!("Just clicked: {:?}", ev);
        commands.entity(player.single()).insert(PathBuilder {
            target: ev.position,
        });
    }
}