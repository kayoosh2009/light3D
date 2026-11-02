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

pub struct EditorIcons {
    pub translate: Handle<Image>,
    pub rotate: Handle<Image>,
    pub scale: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct EditorUiState {
    pub is_exiting: bool,
    pub transform_mode: TransformMode,
}

pub fn editor_ui_system(
    mut contexts: EguiContexts,
    asset_server: Res<AssetServer>,
    current_project: Res<CurrentProject>,
    mut next_state: ResMut<NextState<AppState>>,
    mut windows: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut ui_state: Local<EditorUiState>,
    mut icons: Local<Option<EditorIcons>>,
) {
    let icons = icons.get_or_insert_with(|| EditorIcons {
        translate: asset_server.load("icons/translate.png"),
        rotate: asset_server.load("icons/rotate.png"),
        scale: asset_server.load("icons/scale.png"),
    });

    let translate_tid = contexts.image_id(&icons.translate);
    let rotate_tid = contexts.image_id(&icons.rotate);
    let scale_tid = contexts.image_id(&icons.scale);
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
            });
        });

        ui.separator();

        // Полоска инструментов в стиле Roblox Studio / Blender
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(6.0, 0.0);

            let mut tool_button = |ui: &mut egui::Ui, mode: TransformMode, tid: Option<egui::TextureId>, label: &str| {
                let is_selected = ui_state.transform_mode == mode;
                let size = egui::vec2(56.0, 42.0); // Пропорция 4:3
                let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
                let visuals = ui.style().interact_selectable(&response, is_selected);

                if ui.is_rect_visible(rect) {
                    if is_selected || response.hovered() {
                        ui.painter().rect(
                            rect,
                            visuals.rounding,
                            visuals.bg_fill,
                            visuals.bg_stroke,
                        );
                    }

                    // Иконка по центру в верхней части
                    let icon_size = 20.0;
                    let icon_rect = egui::Rect::from_center_size(
                        rect.center() - egui::vec2(0.0, 7.0),
                        egui::vec2(icon_size, icon_size),
                    );

                    if let Some(id) = tid {
                        egui::Image::new(egui::load::SizedTexture::new(id, [icon_size, icon_size]))
                            .paint_at(ui, icon_rect);
                    } else {
                        let fallback = match mode {
                            TransformMode::Translate => "✛",
                            TransformMode::Rotate => "⟲",
                            TransformMode::Scale => "⤢",
                        };
                        ui.painter().text(
                            icon_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            fallback,
                            egui::FontId::proportional(15.0),
                            visuals.text_color(),
                        );
                    }

                    // Текст под иконкой
                    let text_pos = egui::pos2(rect.center().x, rect.bottom() - 9.0);
                    ui.painter().text(
                        text_pos,
                        egui::Align2::CENTER_CENTER,
                        label,
                        egui::FontId::proportional(11.0),
                        visuals.text_color(),
                    );
                }

                if response.clicked() {
                    ui_state.transform_mode = mode;
                }
            };

            tool_button(ui, TransformMode::Translate, translate_tid, "Move");
            tool_button(ui, TransformMode::Rotate, rotate_tid, "Rotate");
            tool_button(ui, TransformMode::Scale, scale_tid, "Scale");
        });
    });
}
