pub mod camera;

use bevy::prelude::*;
use crate::AppState;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Editor), camera::spawn_editor_camera)
            .add_systems(OnExit(AppState::Editor), camera::cleanup_editor_scene);
    }
}