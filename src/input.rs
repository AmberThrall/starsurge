use bevy::{
    prelude::*, 
    input::mouse::MouseButtonInput
};
use bevy_mod_raycast::{
    DefaultRaycastingPlugin, Intersection, RaycastMesh, RaycastSource, RaycastMethod, RaycastSystem,
};
use crate::{
    Position, Terrain,
};

#[derive(Resource, Default, Debug)]
pub struct MousePosition {
    pub screen_space: Vec2,
    pub world_space: Vec3,
    pub grid_space: Position,
}

#[derive(Debug)]
pub struct MouseClickEvent {
    pub position: Position,
    pub button: MouseButton,
}

#[derive(Reflect, Clone)]
pub struct CameraRaycastSet;

pub fn raycast_mesh() -> RaycastMesh<CameraRaycastSet> {
    RaycastMesh::<CameraRaycastSet>::default()
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DefaultRaycastingPlugin::<CameraRaycastSet>::default())
            .add_event::<MouseClickEvent>()
            .insert_resource(MousePosition::default())
            .insert_resource(bevy_mod_raycast::DefaultPluginState::<CameraRaycastSet>::default().with_debug_cursor())
            .add_system(
                cursor_moved.in_base_set(CoreSet::First)
                    .before(RaycastSystem::BuildRays::<CameraRaycastSet>),
            )
            .add_system(intersection)
            .add_system(mouse_click);
    }
}

fn cursor_moved(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<CameraRaycastSet>>,
    mut mouse_position: ResMut<MousePosition>,
) {
    // get the cursor position
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    mouse_position.screen_space = cursor_position;

    // Update raycast source
    for mut raycast_source in &mut query {
        raycast_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}

fn intersection(
    query: Query<&Intersection<CameraRaycastSet>>,
    terrain_query: Query<&Terrain>,
    mut mouse_position: ResMut<MousePosition>,
) {
    for intersection in &query {
        if let Some(pos) = intersection.position() {
            mouse_position.world_space = *pos;
            mouse_position.grid_space = terrain_query.single().get_grid_position(pos);
        }
    }
}

fn mouse_click(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut click_event: EventWriter<MouseClickEvent>,
    mouse_position: Res<MousePosition>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                click_event.send(MouseClickEvent { 
                    position: mouse_position.grid_space, 
                    button: ev.button 
                });
            },
            _ => (),
        }
    }
}