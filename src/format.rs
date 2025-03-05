//! # Formats & Types
//!
//! A format describes the data structure of a custom resource, like an object's property,
//! a custom recipe, or game data.
//!
//! ## Base Formats
//!
//! The engine comes with a few base formats:
//! - `Int`: whole numbers, without a fractional part;
//! - `Float`: numbers that might have a fractional part;
//! - `Text`: some text, of any length;
//! - `Truth`: a truth about some statement, like the response to a yes/no question.
//!
//! ## Composite Formats
//!
//! You can compose base types using so-called "algebraic data-structures." Since you're reading
//! this documentation written in Rust, you probably know what they are.
//!
//! ### Structs
//!
//! Structs can compose formats linearly, aggregating two formats side by side to create a bigger format:
//!
//! ```
//! --- struct ---
//!  name: Text
//!  age: Int
//! --------------
//! ```
//!
//! ### Eithers
//!
//! Eithers can compose formats alternatively: it describes a choice between one of many cases.
//!
//! ```
//! --- either ---
//! chapter_1
//! chapter_2
//! chapter_3
//! --------------
//! ```
//!
//! Each either variant can also have its own associated data: this makes sense if you are in `chapter_1`,
//! there is no `chapter_2` data available and vice-versa.
//!
//! ## Resources & Recipes
//!
//! Any resource format is also a valid type, like, for example:
//! Item, Attack, Enemy, Character and even Recipe are valid types.
//!
//! Also, any _recipe_ is a valid type - and describes objects that
//! were baked from that recipe.
//!
//! ## Parametric Formats
//!
//! Parametric formats are formats which are "incomplete" on their own:
//! they need a _format parameter_ to fully describe the data structure.
//!
//! ### Lists
//!
//! A list is a collection of items that can grow and shrink as needed. A list
//! can contain items which all have a particular format.
//!
//! ### Maybe
//!
//! A maybe is a collection that contains one item - possibly. You can do different
//! things depending on whether an item is present or not.

use crate::project::resource::ExternalResource;
use serde::{Deserialize, Serialize};
use std::any::TypeId;

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
