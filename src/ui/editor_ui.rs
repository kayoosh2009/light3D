use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{AppState, CurrentProject};

pub fn editor_ui_system(
    mut contexts: EguiContexts,
    current_project: Res<CurrentProject>,
    mut next_state: ResMut<NextState<AppState>>,
    mut windows: Query<&mut Window>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("editor_top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            if ui.button("⏴ В меню").clicked() {
                if let Ok(mut window) = windows.get_single_mut() {
                    window.set_maximized(false);
                    window.resolution.set(800.0, 550.0);
                    window.title = "light3D - Launcher".into();
                }
                next_state.set(AppState::Hub);
            }

            ui.separator();
            ui.label(format!("Проект: {}", current_project.name));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("light3D v0.1");
            });
        });
    });
}