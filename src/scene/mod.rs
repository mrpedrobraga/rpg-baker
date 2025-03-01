use serde::{Deserialize, Serialize};

use crate::project::{resource::ExternalResource, room::ObjectLayer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    name: String,
    description: Option<String>,
    content: SceneContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SceneContent {
    Room(ExternalResource),
    Objects(ObjectLayer),
}
