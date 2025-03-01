use serde::{Deserialize, Serialize};

use crate::project::resource::ExternalResource;

/// A Resource that describes how to interpret a bit of custom data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormatDefinition {
    name: String,
    description: Option<String>,
    #[serde(flatten)]
    expression: Format,
}

/// A tree that describes the type thoroughly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum Format {
    Tuple(Vec<(String, Format)>),
    Either(Vec<(String, Format)>),
    BaseType(BaseType),
    External(ExternalResource),
}

/// A base type from the engine; a primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaseType {
    Never,
    Void,
    Int,
    Float,
    Text,
}
