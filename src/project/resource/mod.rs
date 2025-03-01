//! Resources are useful constructs -- they represent a unit of data
//! describing how something in the game will look or behave.
//!
//! There are many built in resources: Rooms, Items, Tilesets, Images,
//! but you can also create your own using [`Type`]s.

use super::{object::ObjectRecipe, room::Room};
use crate::formats::FormatDefinition;
use futures_signals::signal::Mutable;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use thiserror::Error;
use uuid::Uuid;
use walkdir::WalkDir;

/// Error for when a resource fails to load.
#[derive(Debug, Error)]
#[error("Error loading the Resource from disk.")]
pub enum ResourceLoadError {
    DoesNotExist,
    File(#[from] std::io::Error),
    Deserialize(#[from] serde_json::Error),
}

/// Error for when a resource fails to save.
#[derive(Debug, Error)]
#[error("Error saving the Resource to disk.")]
pub enum ResourceSaveError {
    File(#[from] std::io::Error),
    Serialize(#[from] serde_json::Error),
}

/// A project-wide database of [`Resource`]s that reads resources from json files,
/// and keeps a watch on them for hot-reloading purposes.
///
/// The database is *lazy* -- it only loads resources when they are strictly needed.
///
/// The API allows the editor or the game to get handles to the resource that all share the _same_
/// underlying value, observe changes on that value.
///
/// Futurely, there will be a behaviour to unload resources when they are no longer in use.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDatabase {
    resources: HashMap<Uuid, ResourceEntry>,
}

impl ResourceDatabase {
    /// Scans a directory, finds all resources,
    /// and sets up hot reloading.
    pub fn from_directory(base_path: PathBuf) -> Self {
        let mut database = ResourceDatabase {
            resources: HashMap::new(),
        };

        for entry in WalkDir::new(base_path).into_iter().filter_map(|x| x.ok()) {
            let path = entry.path();
            if !(path.is_file() && path.extension().map_or(false, |ext| ext == "json")) {
                continue;
            }
            if path.ends_with("project.json") {
                continue;
            }
            let text = std::fs::read_to_string(path);
            let text = match text {
                Ok(t) => t,
                Err(_) => {
                    // TODO: Use `tracing` here.
                    eprintln!("Failed to open resource file: {}", path.display());
                    continue;
                }
            };

            let sample = serde_json::from_str(text.as_ref());
            let sample: ResourceSample = match sample {
                Ok(t) => t,
                Err(e) => {
                    // TODO: Use `tracing` here.
                    eprintln!("Failure to parse resource file: {}", path.display());
                    eprintln!("{:?}", e);
                    continue;
                }
            };

            database.resources.insert(
                sample.uuid,
                ResourceEntry {
                    path: path.to_path_buf(),
                    data: None,
                },
            );
        }

        database
    }

    /// Loads an [`ExternalResource`] reference in place.
    pub fn load(&mut self, ext_resource: &mut ExternalResource) -> Result<(), ResourceLoadError> {
        ext_resource.handle = Some(match self.resources.get_mut(&ext_resource.uuid) {
            Some(resource) => resource.get_ref()?,
            None => Err(ResourceLoadError::DoesNotExist)?,
        });
        Ok(())
    }
}

/// An entry of a resource as saved on disk (or nested in another resource).
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceEntry {
    path: PathBuf,
    #[serde(skip)]
    data: Option<Mutable<Resource>>,
}

/// A temporary sample of a resource as the project is gathering UUIDs.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSample {
    uuid: Uuid,
}

impl ResourceEntry {
    /// Returns a safe, hot-reloadable reference to some resource.
    pub fn get_ref(&mut self) -> Result<Mutable<Resource>, ResourceLoadError> {
        match &self.data {
            Some(existing_data) => Ok(existing_data.clone()),
            None => {
                let loaded_data = Mutable::new(Resource::load(&self.path)?);
                self.data = Some(loaded_data.clone());
                Ok(loaded_data)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    #[serde(flatten)]
    data: ResourceData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ResourceData {
    Format(FormatDefinition),
    ObjectRecipe(ObjectRecipe),
    Room(Room),
    Custom(CustomResourceData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomResourceData {}

impl Resource {
    /// Loads the resource from disk.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ResourceLoadError>
    where
        Self: Sized,
    {
        let text = std::fs::read_to_string(path)?;
        let res = serde_json::from_str(text.as_ref())?;
        Ok(res)
    }

    /// Saves the resource to disk.
    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ResourceSaveError>
    where
        Self: Sized,
    {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

/// A reference to a resource that is either embedded or stored somewhere else.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "storage")]
pub enum ResourceRef {
    External(ExternalResource),
    Embedded(EmbeddedResource),
}

/// A reference to a resource that is stored somewhere else.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalResource {
    pub uuid: Uuid,
    #[serde(skip)]
    pub handle: Option<Mutable<Resource>>,
}

impl PartialEq for ExternalResource {
    fn eq(&self, other: &Self) -> bool {
        self.uuid.eq(&other.uuid)
    }
}
impl Eq for ExternalResource {}

/// A reference to a resource that is embedded in another resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedResource {
    pub uuid: Uuid,
    pub resource: Resource,
}
