use bevy::{
    prelude::*,
};
use crate::{
    Terrain, Path,
};

/// Component that allows entities to follow paths.
/// Note: If the entity also has a Position component, nothing will happen.
#[derive(Component)]
pub struct PathFollower {
    /// Movement speed multiplier.
    pub speed: f32,
    progress: f32,
    start_position: Option<Vec3>,
}

impl Default for PathFollower {
    fn default() -> Self {
        Self {
            speed: 1.0,
            progress: 0.0,
            start_position: None,
        }
    }
}

/// Plugin handling movement.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(follow_path);
    }
}

fn follow_path(
    mut commands: Commands,
    terrain_query: Query<&Terrain>,
    mut query: Query<(Entity, &mut Transform, &mut Path, &mut PathFollower)>,
    time: Res<Time>,
) {
    if let Ok(terrain) = terrain_query.get_single() {
        for (entity, mut trans, mut path, mut path_follower) in query.iter_mut() {
            if let Some(next_node) = path.0.get(0) { // Get the next node
                let target_pos = terrain.get_world_position(next_node);

                // Check if we are already at the next node:
                if trans.translation.distance_squared(target_pos) < 0.000001 {
                    path_follower.progress = 1.0;
                }

                // Get the starting position for lerping
                let start_position = match path_follower.start_position {
                    Some(v3) => v3,
                    None => {
                        path_follower.start_position = Some(trans.translation);
                        trans.translation
                    } 
                };

                // Increment the PathFollower's lerp value
                path_follower.progress += path_follower.speed * time.delta_seconds();

                // Translate the entity using lerp
                trans.translation = start_position.lerp(target_pos, path_follower.progress.clamp(0.0, 1.0));

                // Are we at the node?
                if path_follower.progress >= 1.0 {
                    path.0.pop_front();
                    path_follower.progress = 0.0;
                    path_follower.start_position = Some(target_pos);
                }
            } else { // If no such node is found, we are at the end. Remove the path.
                commands.entity(entity).remove::<Path>();
            }
        }
    }
}