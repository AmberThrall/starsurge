use bevy::prelude::*;
use crate::{
    Model, Position, Angle, Block,
};

/// Collection of game objects
#[derive(Component, serde::Deserialize, Debug, Default, Clone)]
pub enum GameObject {
    #[default]
    Empty,
    Decor(String),
    DecorBlock(String),
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
            GameObject::DecorBlock(model_path) => commands.spawn((Model::new(&model_path), Block)),
        }.id();

        commands.entity(entity).insert((
            ev.object.clone(),
            ev.position,
            Angle(ev.angle),
        ));
    }
}