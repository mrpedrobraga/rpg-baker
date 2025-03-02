use serde::{Deserialize, Serialize};

/// A Recipe specifying a runtime behaviour (a script).
#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
pub struct ScriptRecipe {}

impl ScriptRecipe {
    /// Returns a new, empty recipe.
    pub fn new() -> Self {
        ScriptRecipe {}
    }
}

pub trait Block {}
