//! # RPG Baker
//!
//! An RPG engine/editor with an approachable and fun design that allows for
//! extensively personalized games.

pub mod object;
pub mod resource;
pub mod room;
use resource::{ResourceDatabase, ResourceLoadError, ResourceSaveError};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::path::PathBuf;

use crate::{
    format::{BaseType, Format},
    plugin::PluginDatabase,
    scripting::ScriptRecipe,
};

/// An RPG Baker project, assumed to be saved on disk to a folder.
///
/// A project is described by a folder containing a `project.json`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(skip)]
    pub base_path: PathBuf,
    pub name: String,
    pub version: semver::Version,
    pub description: String,
    pub authors: Vec<String>,
    #[serde(skip)]
    pub resource_database: ResourceDatabase,
    #[serde(skip)]
    pub plugin_database: PluginDatabase,

    /* Game Stuff */
    story_definition: Format,
    startup_routine: ScriptRecipe,
}

impl Project {
    /// Creates a new project into a folder.
    pub fn new(path: PathBuf) -> Result<Self, ResourceSaveError> {
        let mut project = Self {
            name: "New Project".into(),
            version: Version::new(0, 0, 0),
            description: "A new RPG from a handsome game developer!".into(),
            authors: vec!["You".into()],
            base_path: path.clone(),
            resource_database: ResourceDatabase::default(),
            plugin_database: PluginDatabase::default(),
            story_definition: Format::BaseType(BaseType::Void),
            startup_routine: ScriptRecipe::new(),
        };

        project._save_as(path)?;

        Ok(project)
    }

    /// Loads a project from a directory containing a `project.json` file.
    pub fn load(path: PathBuf) -> Result<Self, ResourceLoadError> {
        let file = std::fs::read_to_string(path.join("project.json").as_path())?;
        let mut project: Project = from_str(file.as_str())?;
        project.base_path = path.clone();
        project.resource_database = ResourceDatabase::from_directory(path);
        Ok(project)
    }

    /// Saves a project to a directory (this changes the saved path of the project).
    #[inline]
    pub fn save_as(&mut self, path: PathBuf) -> Result<(), ResourceSaveError> {
        self.base_path = path.clone();
        self._save_as(path)
    }

    #[inline]
    fn _save_as(&mut self, path: PathBuf) -> Result<(), ResourceSaveError> {
        let project_string = to_string_pretty(self)?;
        std::fs::write(path.join("project.json"), project_string)?;
        Ok(())
    }

    #[inline]
    pub fn save(&mut self) -> Result<(), ResourceSaveError> {
        let path = self.base_path.clone();
        self.save_as(path)
    }
}
