use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::project_manager::{ProjectInfo, ProjectManager};

#[derive(Resource)]
pub struct HubState {
    pub projects: Vec<ProjectInfo>,
    pub add_texture: Option<egui::TextureId>,
    pub show_create_modal: bool,
    pub new_project_name: String,
}

impl Default for HubState {
    fn default() -> Self {
        Self {
            projects: ProjectManager::list_projects(),
            add_texture: None,
            show_create_modal: false,
            new_project_name: String::new(),
        }
    }
}

pub fn hub_ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<HubState>,
    asset_server: Res<AssetServer>,
) {
    // Ленивая загрузка иконки add.png
    if state.add_texture.is_none() {
        let handle: Handle<Image> = asset_server.load("icons/add.png");
        let tex_id = contexts.add_image(handle);
        state.add_texture = Some(tex_id);
    }

    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("light3D");
        ui.label("Выберите проект для открытия или создайте новый:");
        ui.add_space(15.0);

        let card_w = 150.0;
        let card_h = 200.0; // Соотношение сторон 3:4

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                // Карточка "Создать новый проект"
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(card_w, card_h),
                    egui::Sense::click(),
                );

                let bg_color = if response.hovered() {
                    egui::Color32::from_rgb(60, 60, 65)
                } else {
                    egui::Color32::from_rgb(45, 45, 48)
                };

                ui.painter().rect_filled(rect, 8.0, bg_color);
                ui.painter().rect_stroke(
                    rect,
                    8.0,
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 75)),
                );

                if let Some(tex) = state.add_texture {
                    let icon_size = 48.0;
                    let icon_rect = egui::Rect::from_center_size(
                        egui::pos2(rect.center().x, rect.center().y - 15.0),
                        egui::vec2(icon_size, icon_size),
                    );
                    ui.painter().image(
                        tex,
                        icon_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }

                ui.painter().text(
                    egui::pos2(rect.center().x, rect.max.y - 25.0),
                    egui::Align2::CENTER_CENTER,
                    "Новый проект",
                    egui::FontId::proportional(14.0),
                    egui::Color32::LIGHT_GRAY,
                );

                if response.clicked() {
                    state.show_create_modal = true;
                }

                // Карточки существующих проектов
                for proj in &state.projects {
                    let (card_rect, card_resp) = ui.allocate_exact_size(
                        egui::vec2(card_w, card_h),
                        egui::Sense::click(),
                    );

                    let card_bg = if card_resp.hovered() {
                        egui::Color32::from_rgb(55, 55, 60)
                    } else {
                        egui::Color32::from_rgb(40, 40, 42)
                    };

                    ui.painter().rect_filled(card_rect, 8.0, card_bg);
                    ui.painter().rect_stroke(
                        card_rect,
                        8.0,
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 65)),
                    );

                    // Область для превью (верхние 130px)
                    let preview_rect = egui::Rect::from_min_size(
                        card_rect.min + egui::vec2(6.0, 6.0),
                        egui::vec2(card_w - 12.0, 125.0),
                    );
                    ui.painter().rect_filled(
                        preview_rect,
                        4.0,
                        egui::Color32::from_rgb(25, 25, 28),
                    );

                    // Название проекта
                    ui.painter().text(
                        egui::pos2(card_rect.min.x + 10.0, card_rect.min.y + 145.0),
                        egui::Align2::LEFT_TOP,
                        &proj.name,
                        egui::FontId::proportional(15.0),
                        egui::Color32::WHITE,
                    );

                    // Дата изменения
                    ui.painter().text(
                        egui::pos2(card_rect.min.x + 10.0, card_rect.min.y + 172.0),
                        egui::Align2::LEFT_TOP,
                        &proj.modified,
                        egui::FontId::proportional(11.0),
                        egui::Color32::GRAY,
                    );

                    if card_resp.clicked() {
                        println!("Открытие проекта: {}", proj.name);
                    }
                }
            });
        });
    });

    // Модальное окно создания проекта
    if state.show_create_modal {
        egui::Window::new("Создание проекта")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.label("Введите название проекта (оставьте пустым для автоимени):");
                ui.text_edit_singleline(&mut state.new_project_name);
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Создать").clicked() {
                        ProjectManager::create_project(state.new_project_name.clone());
                        state.new_project_name.clear();
                        state.projects = ProjectManager::list_projects();
                        state.show_create_modal = false;
                    }

                    if ui.button("Отмена").clicked() {
                        state.new_project_name.clear();
                        state.show_create_modal = false;
                    }
                });
            });
    }
}