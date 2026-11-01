use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::AppState;

#[derive(Resource)]
pub struct LoadingState {
    pub progress: f32,
    pub speed: f32,
}

impl Default for LoadingState {
    fn default() -> Self {
        Self {
            progress: 0.0,
            speed: 1.2, // Скорость заполнения прогресс-бара
        }
    }
}

pub fn loading_ui_system(
    time: Res<Time>,
    mut contexts: EguiContexts,
    mut state: ResMut<LoadingState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    state.progress += state.speed * time.delta_seconds();

    if state.progress >= 1.0 {
        next_state.set(AppState::Hub);
        return;
    }

    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(30, 30, 33)))
        .show(ctx, |ui| {
            let screen_rect = ui.max_rect();
            let center = screen_rect.center();

            // Логотип light3D
            ui.painter().text(
                egui::pos2(center.x, center.y - 40.0),
                egui::Align2::CENTER_CENTER,
                "light3D",
                egui::FontId::proportional(32.0),
                egui::Color32::WHITE,
            );

            // Подзаголовок
            ui.painter().text(
                egui::pos2(center.x, center.y - 10.0),
                egui::Align2::CENTER_CENTER,
                "Loading workspace...",
                egui::FontId::proportional(14.0),
                egui::Color32::GRAY,
            );

            // Полоса прогресса
            let bar_w = 260.0;
            let bar_h = 6.0;
            let bar_bg_rect = egui::Rect::from_center_size(
                egui::pos2(center.x, center.y + 25.0),
                egui::vec2(bar_w, bar_h),
            );

            // Фон полосы
            ui.painter().rect_filled(bar_bg_rect, 3.0, egui::Color32::from_rgb(45, 45, 50));

            // Заполняемая часть
            let fill_w = bar_w * state.progress.clamp(0.0, 1.0);
            let bar_fill_rect = egui::Rect::from_min_size(
                bar_bg_rect.min,
                egui::vec2(fill_w, bar_h),
            );
            ui.painter().rect_filled(bar_fill_rect, 3.0, egui::Color32::from_rgb(80, 140, 255));
        });
}