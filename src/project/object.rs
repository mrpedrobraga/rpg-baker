use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInstance {
    name: String,
    description: Option<String>,
}

/// An object's recipe, it describes how an object will be
/// created when it's loaded in game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRecipe {
    name: String,
    description: Option<String>,
    parts: Vec<Part>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {}
