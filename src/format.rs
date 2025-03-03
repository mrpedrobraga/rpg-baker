use std::any::TypeId;

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
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum Format {
    Tuple(Vec<(String, Format)>),
    Either(Vec<(String, Format)>),
    BaseType(BaseType),
    External(ExternalResource),
}

/// A base type from the engine; a primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaseType {
    Void,
    Int,
    Float,
    Text,
}

impl BaseType {
    pub fn type_id(&self) -> TypeId {
        match self {
            BaseType::Void => TypeId::of::<()>(),
            BaseType::Int => TypeId::of::<i32>(),
            BaseType::Float => TypeId::of::<f32>(),
            BaseType::Text => TypeId::of::<String>(),
        }
    }
}

/// Describes a value.
#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum VariantValue {
    /// A value that carries no information.
    Void,
    /// An integer number (with 32 bits).
    Int(i32),
}

impl VariantValue {
    pub fn base_type(&self) -> BaseType {
        match self {
            VariantValue::Int(_) => BaseType::Int,
            VariantValue::Void => BaseType::Void,
        }
    }
}
