//! # RPG Baker
//!
//! An RPG engine/editor with an approachable and fun design that allows for
//! extensively personalized games.

pub mod object;
pub mod resource;
pub mod room;
use resource::{ResourceDatabase, ResourceLoadError, ResourceSaveError};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::path::{Path, PathBuf};

/// An RPG Baker project, assumed to be saved on disk to a folder.
///
/// A project is described by a folder containing a `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    #[serde(skip)]
    base_path: PathBuf,
    name: String,
    version: String,
    description: String,
    authors: Vec<String>,
    #[serde(skip)]
    pub resource_database: ResourceDatabase,
}

impl Project {
    /// Creates a new project into a folder.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ResourceSaveError> {
        let mut project = Self {
            name: "New Project".into(),
            version: "0.0".into(),
            description: "A new RPG from a handsome game developer!".into(),
            authors: vec!["You".into()],
            base_path: path.as_ref().into(),
            resource_database: ResourceDatabase::default(),
        };

        project.save_as(path)?;

        Ok(project)
    }

    /// Loads a project from a folder containing a `project.json` file.
    pub fn load(path: PathBuf) -> Result<Self, ResourceLoadError> {
        let file = std::fs::read_to_string(path.join("project.json").as_path())?;
        let mut project: Project = from_str(file.as_str())?;
        project.base_path = path.clone();
        project.resource_database = ResourceDatabase::from_directory(path);
        Ok(project)
    }

    /// Saves a project to disk (this changes the saved path of the project).
    #[inline]
    pub fn save_as<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ResourceSaveError> {
        let project_string = to_string_pretty(self)?;
        self.base_path = path.as_ref().into();
        std::fs::write(path.as_ref(), project_string)?;
        Ok(())
    }

    #[inline]
    pub fn save(&mut self) -> Result<(), ResourceSaveError> {
        let path = self.base_path.clone();
        self.save_as(path)
    }
}
