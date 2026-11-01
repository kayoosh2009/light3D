use crate::project_manager::{ProjectInfo, ProjectManager};
use crate::{AppState, CurrentProject};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::collections::HashMap;

#[derive(Resource)]
pub struct HubState {
    pub projects: Vec<ProjectInfo>,
    pub add_texture: Option<egui::TextureId>,
    pub project_textures: HashMap<String, egui::TextureId>,
    pub show_create_modal: bool,
    pub new_project_name: String,
}

impl Default for HubState {
    fn default() -> Self {
        Self {
            projects: ProjectManager::list_projects(),
            add_texture: None,
            project_textures: HashMap::new(),
            show_create_modal: false,
            new_project_name: String::new(),
        }
    }
}

pub fn refresh_hub_on_enter(mut state: ResMut<HubState>) {
    state.projects = ProjectManager::list_projects();
    state.project_textures.clear();
}

pub fn hub_ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<HubState>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut current_project: ResMut<CurrentProject>,
    mut windows: Query<&mut Window>,
) {
    // 1. Предзагрузка иконки добавления проекта
    if state.add_texture.is_none() {
        let handle: Handle<Image> = asset_server.load("icons/add.png");
        let tex_id = contexts.add_image(handle);
        state.add_texture = Some(tex_id);
    }

    // 2. Предзагрузка скриншотов проектов ДО вызова UI closure
    let mut pending_textures = Vec::new();
    for proj in &state.projects {
        if !state.project_textures.contains_key(&proj.name) {
            if let Some(ref path) = proj.preview_path {
                let handle: Handle<Image> = asset_server.load(path.clone());
                let tex_id = contexts.add_image(handle);
                pending_textures.push((proj.name.clone(), tex_id));
            }
        }
    }
    for (name, tex_id) in pending_textures {
        state.project_textures.insert(name, tex_id);
    }

    let mut selected_to_open = None;
    let mut trigger_create_modal = false;

    // 3. Отрисовка UI
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("light3D");
        ui.label("Select a project to open or create a new one:");
        ui.add_space(15.0);

        let card_w = 150.0;
        let card_h = 200.0; // 3:4 aspect ratio

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                // Card: "New Project"
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(card_w, card_h), egui::Sense::click());

                let bg_color = if response.hovered() {
                    egui::Color32::from_rgb(60, 60, 65)
                } else {
                    egui::Color32::from_rgb(45, 45, 48)
                };

                ui.painter().rect_filled(rect, 8.0, bg_color);
                ui.painter().rect_stroke(
                    rect,
                    8.0,
                    egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(70, 70, 75)),
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
                    "New Project",
                    egui::FontId::proportional(14.0),
                    egui::Color32::LIGHT_GRAY,
                );

                if response.clicked() {
                    trigger_create_modal = true;
                }

                // Project Cards
                for proj in &state.projects {
                    let (card_rect, card_resp) =
                        ui.allocate_exact_size(egui::vec2(card_w, card_h), egui::Sense::click());

                    let card_bg = if card_resp.hovered() {
                        egui::Color32::from_rgb(55, 55, 60)
                    } else {
                        egui::Color32::from_rgb(40, 40, 42)
                    };

                    ui.painter().rect_filled(card_rect, 8.0, card_bg);
                    ui.painter().rect_stroke(
                        card_rect,
                        8.0,
                        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(60, 60, 65)),
                    );

                    // Preview Area (top 125px)
                    let preview_rect = egui::Rect::from_min_size(
                        card_rect.min + egui::vec2(6.0, 6.0),
                        egui::vec2(card_w - 12.0, 125.0),
                    );

                    if let Some(&tex_id) = state.project_textures.get(&proj.name) {
                        // Center-crop UV coordinates (сохраняет пропорции без растяжения/сплющивания)
                        let target_aspect = preview_rect.width() / preview_rect.height();
                        let src_aspect = 16.0 / 9.0; // Соотношение экрана монитора

                        let uv = if src_aspect > target_aspect {
                            let visible_width = target_aspect / src_aspect;
                            let offset = (1.0 - visible_width) / 2.0;
                            egui::Rect::from_min_max(
                                egui::pos2(offset, 0.0),
                                egui::pos2(1.0 - offset, 1.0),
                            )
                        } else {
                            let visible_height = src_aspect / target_aspect;
                            let offset = (1.0 - visible_height) / 2.0;
                            egui::Rect::from_min_max(
                                egui::pos2(0.0, offset),
                                egui::pos2(1.0, 1.0 - offset),
                            )
                        };

                        ui.painter()
                            .image(tex_id, preview_rect, uv, egui::Color32::WHITE);
                    } else {
                        ui.painter().rect_filled(
                            preview_rect,
                            4.0,
                            egui::Color32::from_rgb(25, 25, 28),
                        );
                    }

                    // Project Name
                    ui.painter().text(
                        egui::pos2(card_rect.min.x + 10.0, card_rect.min.y + 145.0),
                        egui::Align2::LEFT_TOP,
                        &proj.name,
                        egui::FontId::proportional(15.0),
                        egui::Color32::WHITE,
                    );

                    // Modified Date
                    ui.painter().text(
                        egui::pos2(card_rect.min.x + 10.0, card_rect.min.y + 172.0),
                        egui::Align2::LEFT_TOP,
                        &proj.modified,
                        egui::FontId::proportional(11.0),
                        egui::Color32::GRAY,
                    );

                    if card_resp.clicked() {
                        selected_to_open = Some(proj.name.clone());
                    }
                }
            });
        });
    });

    if trigger_create_modal {
        state.show_create_modal = true;
    }

    if let Some(name) = selected_to_open {
        current_project.name = name.clone();
        if let Ok(mut window) = windows.get_single_mut() {
            window.set_maximized(true);
            window.title = format!("light3D — {}", name);
        }
        next_state.set(AppState::Loading);
    }

    // Modal Dialog
    if state.show_create_modal {
        egui::Window::new("Create Project")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.label("Project name (leave empty for auto-generated name):");
                ui.text_edit_singleline(&mut state.new_project_name);
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Create").clicked() {
                        ProjectManager::create_project(state.new_project_name.clone());
                        state.new_project_name.clear();
                        state.projects = ProjectManager::list_projects();
                        state.show_create_modal = false;
                    }

                    if ui.button("Cancel").clicked() {
                        state.new_project_name.clear();
                        state.show_create_modal = false;
                    }
                });
            });
    }
}
