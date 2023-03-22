use bevy::prelude::*;
use crate::Model;
use crate::Position;
use crate::Angle;

/// Collection of game objects
#[derive(serde::Deserialize, Debug, Default)]
pub enum GameObject {
    #[default]
    Empty,
    Decor(String)
}

#[derive(Debug, Default)]
pub struct SpawnEvent {
    pub object: GameObject,
    pub position: Position,
    pub angle: f32,
}

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>()
            .add_system(spawner_system);
    }
}

fn spawner_system(
    mut commands: Commands,
    mut events: EventReader<SpawnEvent>,
) {
    for ev in events.iter() {
        let entity = match &ev.object {
            GameObject::Empty => commands.spawn_empty(),
            GameObject::Decor(model_path) => commands.spawn(Model::new(&model_path)),
        }.id();

        commands.entity(entity).insert((
            ev.position,
            Angle(ev.angle),
        ));
    }
}