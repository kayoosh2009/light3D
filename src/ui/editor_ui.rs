use std::path::Path;
use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use crate::project_manager::PROJECTS_DIR;
use crate::{AppState, CurrentProject};

pub fn editor_ui_system(
    mut contexts: EguiContexts,
    current_project: Res<CurrentProject>,
    mut next_state: ResMut<NextState<AppState>>,
    mut windows: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("editor_top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            if ui.button("⏴ Hub").clicked() {
                if let Ok((window_entity, mut window)) = windows.get_single_mut() {
                    // Сохраняем скриншот последнего кадра проекта
                    let path = Path::new(PROJECTS_DIR)
                        .join(&current_project.name)
                        .join("preview.png");
                    
                    let _ = screenshot_manager.save_screenshot_to_disk(window_entity, path);

                    window.set_maximized(false);
                    window.resolution.set(800.0, 550.0);
                    window.title = "light3D - Launcher".into();
                }
                next_state.set(AppState::Hub);
            }

            ui.separator();
            ui.label(format!("Project: {}", current_project.name));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("light3D v0.1");
            });
        });
    });
}