use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource, Default)]
pub struct MapRegistry(pub HashMap<String, String>);

impl MapRegistry {
    pub fn register_map(&mut self, id: &str, path: &str) -> Option<String> {
        self.0.insert(String::from(id), String::from(path))
    }

    pub fn get(&self, id: &str) -> Option<&String> {
        self.0.get(id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}