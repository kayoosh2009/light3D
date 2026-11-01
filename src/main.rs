use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod editor;
mod project_manager;
mod ui;

use editor::EditorPlugin;
use ui::editor_ui::editor_ui_system;
use ui::hub::{hub_ui_system, refresh_hub_on_enter, HubState};
use ui::loading::{loading_ui_system, LoadingState};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Hub,
    Loading,
    Editor,
}

#[derive(Resource, Default)]
pub struct CurrentProject {
    pub name: String,
}

fn main() {
    project_manager::ProjectManager::ensure_project_dir();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "light3D - Launcher".into(),
                resolution: (800.0_f32, 550.0_f32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .init_state::<AppState>()
        .init_resource::<LoadingState>()
        .init_resource::<HubState>()
        .init_resource::<CurrentProject>()
        .add_plugins(EditorPlugin)
        // Loading системы
        .add_systems(Update, loading_ui_system.run_if(in_state(AppState::Loading)))
        // Hub системы
        .add_systems(OnEnter(AppState::Hub), refresh_hub_on_enter)
        .add_systems(Update, hub_ui_system.run_if(in_state(AppState::Hub)))
        // Editor UI системы
        .add_systems(Update, editor_ui_system.run_if(in_state(AppState::Editor)))
        .run();
}