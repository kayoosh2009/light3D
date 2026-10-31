pub mod camera;
pub mod grid;

use bevy::prelude::*;
use crate::AppState;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Editor),
            (camera::spawn_editor_camera, grid::spawn_grid),
        )
        .add_systems(
            Update,
            camera::camera_controller_system.run_if(in_state(AppState::Editor)),
        )
        .add_systems(OnExit(AppState::Editor), camera::cleanup_editor_scene);
    }
}