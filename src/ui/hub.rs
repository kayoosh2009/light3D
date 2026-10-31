use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::project_manager::ProjectManager;

#[derive(Resource)]
pub struct HubState {
    pub new_project_name: String,
    pub projects: Vec<String>,
    pub selected_project: Option<String>,
}

impl Default for HubState {
    fn default() -> Self {
        Self {
            new_project_name: String::new(),
            projects: ProjectManager::list_projects(),
            selected_project: None,
        }
    }
}

pub fn hub_ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<HubState>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.heading("light3D — Менеджер проектов");
        ui.add_space(10.0);

        // Панель создания нового проекта
        ui.horizontal(|ui| {
            ui.label("Имя проекта:");
            ui.text_edit_singleline(&mut state.new_project_name);

            if ui.button("Создать проект").clicked() {
                let name = ProjectManager::create_project(state.new_project_name.clone());
                state.new_project_name.clear();
                state.projects = ProjectManager::list_projects();
                state.selected_project = Some(name);
            }
        });

        ui.separator();
        ui.add_space(10.0);
        ui.label("Существующие проекты:");

        // Список проектов
        let mut clicked_project = None;
        egui::ScrollArea::vertical().show(ui, |ui| {
            for project in &state.projects {
                let is_selected = state.selected_project.as_ref() == Some(project);
                if ui.selectable_label(is_selected, project).clicked() {
                    clicked_project = Some(project.clone());
                }
            }
        });

        if let Some(project) = clicked_project {
            state.selected_project = Some(project);
        }

        ui.add_space(10.0);
        if let Some(ref selected) = state.selected_project {
            ui.separator();
            ui.label(format!("Выбран проект: {}", selected));
            if ui.button("Открыть проект").clicked() {
                println!("Запуск редактора для проекта: {}", selected);
                // Сюда мы привяжем переход в окно 3D-редактора
            }
        }
    });
}