use bevy::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use crate::{
    Position, Terrain,
};

const MAX_A_STAR_ITERATIONS: usize = 100;

/// Component for entities to follow a path.
#[derive(Component)]
pub struct Path(pub VecDeque<Position>);

/// Component that generates a path using A* pathfinding. It will try to get as close as possible.
#[derive(Component, Default)]
pub struct PathBuilder {
    pub target: Position,
}

/// Component to mark solid entities that take up a position
#[derive(Component, Debug)]
pub struct Block;

/// Plugin handling ai systems
pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(a_star);
    }
}

fn a_star(
    mut commands: Commands,
    terrain_query: Query<&Terrain>,
    path_builders: Query<(Entity, &Transform, &PathBuilder)>,
    blocks_query: Query<&Position, With<Block>>,
) {
    if let Ok(terrain) = terrain_query.get_single() {
        for (entity, trans, path_builder) in path_builders.iter() {
            commands.entity(entity).remove::<PathBuilder>();

            let start = terrain.get_grid_position(&trans.translation);
            let mut target = path_builder.target;
            if start.layer != target.layer { // Can't form a path between layers.
                continue;
            }

            // Make sure we aren't moving to a block.
            let mut target_is_block = false;
            for p in blocks_query.iter() {
                if p == &target {
                    target_is_block = true;
                    break;
                }
            }

            if target_is_block {
                let mut closest_target = None;
                let mut closest_target_dist = f32::INFINITY;

                // Check each neighbor of target and get closest one to start if possible.
                for x in -1..2 {
                    for y in -1..2 {
                        let neighbor = Position::new(target.x + x, target.y + y, target.layer);
                        if x == 0 && y == 0 {
                            continue;   
                        }
    
                        // Look through the block query to see if a block is at neighbor of target.
                        let mut is_block = false;
                        for p in blocks_query.iter() {
                            if p == &neighbor {
                                is_block = true;
                                break;
                            }
                        }

                        if !is_block {
                            let dist = neighbor.distance_squared(&start) as f32;
                            if dist < closest_target_dist {
                                closest_target = Some(neighbor);
                                closest_target_dist = dist;
                            }
                        }
                    }
                }

                // Make sure the new target is not a block.
                if closest_target.is_none() {
                    continue;
                }

                target = closest_target.unwrap();
            }

            

            // ================
            // A* Algorithm
            // ================
            // Our heuristic function: simple linear distance squared
            let h = |n: &Position| -> f32 {
                n.distance_squared(&target) as f32
            };

            // The set of discovered nodes
            let mut open_set: HashSet<Position> = HashSet::new();
            open_set.insert(start);

            // For node n, came_from[n] is the node preceding it on the cheapest path from start currently known.
            let mut came_from: HashMap<Position, Position> = HashMap::new();

            // For node n, g_score[n] is the cheapest cost from start to n currently known.
            let mut g_score: HashMap<Position, f32> = HashMap::new();
            g_score.insert(start, 0.0);
            
            // For node n, f_score[n] = g_score[n] + h(n)
            let mut f_score: HashMap<Position, f32> = HashMap::new();
            f_score.insert(start, h(&start));

            let mut current = start;
            let mut iteration_num = 0;
            while !open_set.is_empty() {
                // If it takes to long to find the path, assume it doesn't exist.
                iteration_num += 1;
                if iteration_num > MAX_A_STAR_ITERATIONS {
                    break;
                }

                // get the node with lowest f_score.
                let mut lowest_score = f32::INFINITY;
                for pos in open_set.iter() {
                    let f = f_score.get(pos).unwrap_or(&f32::INFINITY);
                    if f < &lowest_score {
                        lowest_score = *f;
                        current = *pos;
                    }
                }

                // If the current node is the target, we are done.
                if current == target {
                    break;
                }

                // remove current from open_set
                open_set.remove(&current);

                // get current's neighbors.
                let mut neighbors: HashSet<Position> = HashSet::new();
                for x in -1..2 {
                    for y in -1..2 {
                        let neighbor = Position::new(current.x + x, current.y + y, current.layer);
                        if x == 0 && y == 0 {
                            continue;   
                        }

                        // Look through the block query to see if a block is at neighbor.
                        let mut is_block = false;
                        for p in blocks_query.iter() {
                            if p == &neighbor {
                                is_block = true;
                                break;
                            }
                        }

                        if !is_block {
                            neighbors.insert(neighbor);
                        }
                    }
                }

                for neighbor in neighbors.iter() {
                    // g is the distance from start to neighbor through current.
                    let g = g_score.get(&current).unwrap_or(&f32::INFINITY) + 1.0;

                    // this path seems better than the previous one found.
                    if &g < g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
                        came_from.insert(*neighbor, current);
                        f_score.insert(*neighbor, g + h(neighbor));
                        g_score.insert(*neighbor, g);
                        open_set.insert(*neighbor);
                    }
                }
            }

            // Reconstruct the path if we found one.
            if iteration_num < MAX_A_STAR_ITERATIONS {
                let mut path: VecDeque<Position> = VecDeque::new();
                path.push_back(current);
                while let Some(from) = came_from.get(&current) {
                    current = *from;
                    path.push_front(current);
                }
    
                // Actually insert the Path component.
                commands.entity(entity).insert(Path(path));
            }
        }
    }
}