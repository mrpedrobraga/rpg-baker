use crate::behaviour::BehaviourDescriptor;
use serde::{Deserialize, Serialize};

/// An object's recipe, it describes how an object will be
/// created when it's loaded in game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRecipe {
    name: String,
    description: Option<String>,
    parts: Vec<Part>,
    script: BehaviourDescriptor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {}

/// Describes an object, to be baked from an [`ObjectRecipe`] into an [`super::room::ObjectLayer`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInstanceDescriptor {
    name: String,
    description: Option<String>,
}

/// An actual runtime object, containing references to the descriptor
/// that created it plus runtime data.
#[derive(Debug, Clone, Serialize)]
pub struct ObjectInstance<'game> {
    descriptor: &'game ObjectInstanceDescriptor,
}
