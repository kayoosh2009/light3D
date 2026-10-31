use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub const PROJECTS_DIR: &str = "Light3D_Projects";

#[derive(Clone, Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub modified: String,
    pub preview_path: Option<PathBuf>,
}

pub struct ProjectManager;

impl ProjectManager {
    /// Создает базовую директорию для проектов, если она не существует
    pub fn ensure_project_dir() -> PathBuf {
        let path = Path::new(PROJECTS_DIR);
        if !path.exists() {
            fs::create_dir_all(path).expect("Не удалось создать папку проектов");
        }
        path.to_path_buf()
    }

    /// Получает список всех папок-проектов с метаданными
    pub fn list_projects() -> Vec<ProjectInfo> {
        Self::ensure_project_dir();
        let mut list = Vec::new();

        if let Ok(entries) = fs::read_dir(PROJECTS_DIR) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        if let Ok(name) = entry.file_name().into_string() {
                            let modified = entry
                                .metadata()
                                .and_then(|m| m.modified())
                                .unwrap_or(SystemTime::now());
                            
                            let datetime: chrono::DateTime<chrono::Local> = modified.into();
                            let formatted_date = datetime.format("%d.%m.%Y %H:%M").to_string();

                            let preview = entry.path().join("preview.png");
                            let preview_path = if preview.exists() {
                                Some(preview)
                            } else {
                                None
                            };

                            list.push(ProjectInfo {
                                name,
                                modified: formatted_date,
                                preview_path,
                            });
                        }
                    }
                }
            }
        }
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }

    /// Генерирует имя проекта по умолчанию (например: 000, 001)
    pub fn next_default_name() -> String {
        let projects = Self::list_projects();
        for i in 0..1000 {
            let candidate = format!("{:03}", i);
            if !projects.iter().any(|p| p.name == candidate) {
                return candidate;
            }
        }
        "999".into()
    }

    /// Создает новую папку проекта
    pub fn create_project(mut name: String) -> String {
        Self::ensure_project_dir();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            name = Self::next_default_name();
        } else {
            name = trimmed.to_string();
        }

        let project_path = Path::new(PROJECTS_DIR).join(&name);
        if !project_path.exists() {
            fs::create_dir_all(&project_path).expect("Не удалось создать проект");
        }

        name
    }
}