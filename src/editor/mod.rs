pub mod camera;
pub mod grid;

use crate::AppState;
use bevy::prelude::*;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        // Создаем сцену заранее в фоне при Loading, чтобы избежать мигания черного экрана
        app.add_systems(
            OnEnter(AppState::Loading),
            (camera::spawn_editor_camera, grid::spawn_grid),
        )
        .add_systems(
            Update,
            camera::camera_controller_system.run_if(in_state(AppState::Editor)),
        )
        .add_systems(OnExit(AppState::Editor), camera::cleanup_editor_scene);
    }
}
