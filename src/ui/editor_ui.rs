use crate::project_manager::ProjectManager;
use crate::{AppState, CurrentProject};
use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransformMode {
    #[default]
    Translate,
    Rotate,
    Scale,
}

#[derive(Resource, Default)]
pub struct EditorUiState {
    pub is_exiting: bool,
    pub transform_mode: TransformMode,
}

pub fn editor_ui_system(
    mut contexts: EguiContexts,
    current_project: Res<CurrentProject>,
    mut next_state: ResMut<NextState<AppState>>,
    mut windows: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut ui_state: Local<EditorUiState>,
) {
    let ctx = contexts.ctx_mut();

    if ui_state.is_exiting {
        if let Ok((window_entity, mut window)) = windows.get_single_mut() {
            let path = ProjectManager::get_projects_dir()
                .join(&current_project.name)
                .join("preview.png");

            let _ = screenshot_manager.save_screenshot_to_disk(window_entity, path);

            window.set_maximized(false);
            window.resolution.set(800.0, 550.0);
            window.title = "light3D - Launcher".into();
        }
        ui_state.is_exiting = false;
        next_state.set(AppState::Hub);
    }

    egui::TopBottomPanel::top("editor_top_panel").show(ctx, |ui| {
        if ui_state.is_exiting {
            ui.set_visible(false);
        }
        egui::menu::bar(ui, |ui| {
            if ui.button("⏴ Hub").clicked() {
                // Инициируем выход: скрываем панель на следующий кадр для скриншота без UI
                ui_state.is_exiting = true;
            }

            ui.separator();
            ui.label(format!("Project: {}", current_project.name));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("light3D v0.1");
                ui.separator();

                // Кнопки режимов трансформации (в right_to_left отображаются слева направо: Move | Rotate | Scale)
                ui.selectable_value(
                    &mut ui_state.transform_mode,
                    TransformMode::Scale,
                    "⤢ Scale",
                );
                ui.selectable_value(
                    &mut ui_state.transform_mode,
                    TransformMode::Rotate,
                    "⟲ Rotate",
                );
                ui.selectable_value(
                    &mut ui_state.transform_mode,
                    TransformMode::Translate,
                    "✛ Move",
                );
            });
        });
    });
}
