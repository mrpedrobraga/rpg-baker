use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, Weak},
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDatabase {
    #[serde(skip)]
    base_path: PathBuf,
    resources: HashMap<Uuid, ResourceFile>,
}

impl ResourceDatabase {
    fn from_directory(base_path: PathBuf) -> Self {
        let database = ResourceDatabase {
            base_path,
            resources: HashMap::new(),
        };

        database
    }

    fn get_resource(uuid: Uuid) {}
}

type Ref<T> = Arc<Mutex<T>>;
type WeakRef<T> = Weak<Mutex<T>>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceFile {
    path: PathBuf,
    #[serde(skip)]
    data: Option<WeakRef<Box<dyn Resource>>>,
}

pub trait Resource {}

pub trait SaveLoad {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, ResourceLoadError>
    where
        Self: Sized;
    fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ResourceSaveError>;
}

#[derive(Debug, Error)]
#[error("Error loading the Resource from disk.")]
pub enum ResourceLoadError {
    File(#[from] std::io::Error),
    Deserialize(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
#[error("Error saving the Resource to disk.")]
pub enum ResourceSaveError {
    File(#[from] std::io::Error),
    Serialize(#[from] serde_json::Error),
}
