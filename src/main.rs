use bevy::prelude::*;
use bevy_egui::{EguiPlugin};

mod project_manager;
mod ui;

use ui::hub::{hub_ui_system, HubState};

fn main() {
    // Гарантируем создание папки Light3D_Projects при запуске
    project_manager::ProjectManager::ensure_project_dir();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "light3D - Launcher".into(),
                resolution: (600.0, 400.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .init_resource::<HubState>()
        .add_systems(Update, hub_ui_system)
        .run();
}