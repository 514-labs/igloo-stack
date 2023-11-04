//! # Project
//!
//! This module contains the `Project` struct, which represents a users project. These projects are data-intensive applications or services. A project is initialized using the `igloo init` command and is stored in the `~/.igloo` directory. The `Project` struct contains the following fields:
//! - `name` - The name of the project
//! - `language` - The language of the project
//!
//! ## Example
//! ```
//! use igloo::framework::Project;
//! use igloo::framework::languages::SupportedLanguages;
//!
//! let project = Project::new("my-project".to_string(), SupportedLanguages::Typescript);
//! ```

use std::path::{Path, PathBuf};

use crate::framework::{
    directories::{get_igloo_directory, get_igloo_directory_from_current},
    languages::SupportedLanguages,
};
use serde::{Deserialize, Serialize};

fn parse_path(path: &str) -> PathBuf {
    match path {
        "." => std::env::current_dir().unwrap(),
        ".." => std::env::current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf(),
        _ => Path::new(path).to_path_buf(),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub language: SupportedLanguages,
    pub location: PathBuf,
    pub app_folder: PathBuf,
}

impl Project {
    pub fn new(name: String, language: SupportedLanguages, path: String) -> Self {
        Self {
            name,
            language,
            location: parse_path(&path),
            app_folder: parse_path(&path).join("app"),
        }
    }

    pub fn from_file() -> Result<Self, std::io::Error> {
        let igloo_dir = get_igloo_directory_from_current()?;
        let project_file = igloo_dir.join("project.toml");
        let project = std::fs::read_to_string(project_file)?;
        let project: Project = toml::from_str(&project)?;
        Ok(project)
    }

    pub fn write_to_file(&self) -> Result<(), std::io::Error> {
        let igloo_dir = get_igloo_directory(self.clone())?;
        let project_file = igloo_dir.join("project.toml");
        let toml_project = toml::to_string(self);
        match toml_project {
            Ok(project) => {
                std::fs::write(project_file, project)?;
                Ok(())
            }
            Err(err) => {
                println!("Failed to serialize project: {}", err);
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to serialize project",
                ))
            }
        }
    }
}
