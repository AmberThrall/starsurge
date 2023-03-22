use bevy::prelude::*;
use bevy::window::WindowResolution;

pub struct BevyConfigPlugin;

impl Plugin for BevyConfigPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920.0, 1080.0),
                title: "Starsurge".to_string(),
                ..default()
            }),
            ..default()
        });

        #[cfg(feature = "dev")]
        let default_plugins = default_plugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        });

        app.add_plugins(default_plugins)
            .insert_resource(Msaa::Sample4)
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)));
    }
}
