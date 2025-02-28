pub mod object;
pub mod resource;
pub mod room;
use resource::{ResourceDatabase, ResourceLoadError, ResourceSaveError, SaveLoad};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// A new RPG Baker project, with info and all.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    #[serde(skip)]
    base_path: PathBuf,
    name: String,
    version: String,
    description: String,
    authors: Vec<String>,
    #[serde(skip)]
    resource_database: ResourceDatabase,
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

        project.save(path)?;

        Ok(project)
    }
}

impl SaveLoad for Project {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, ResourceLoadError> {
        let file = std::fs::read_to_string(path.as_ref())?;
        let mut project: Project = from_str(file.as_str())?;
        project.base_path = path.as_ref().into();
        Ok(project)
    }

    fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ResourceSaveError> {
        let project_string = to_string_pretty(self)?;
        self.base_path = path.as_ref().into();
        std::fs::write(path.as_ref(), project_string)?;
        Ok(())
    }
}
