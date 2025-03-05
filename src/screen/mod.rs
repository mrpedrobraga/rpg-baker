//! A Screen is one of the primary concepts for game-making.
//!
//! You may conceptualise it like a power point slide,
//! with objects that can be placed relative to the screen,
//! and can be animated or react to user input.
//!
//! You may use screens for making:
//! - Title Screens;
//! - Inventory Screens;
//! - Setting Menus;
//! - Crafting Menus;
//! - HUD;
//!
//! And anything that requires graphics on a canvas! In fact,
//! even the overworld is rendered using a dedicated screen.
//!
//! As implied by the list of things screens can do,
//! you can invoke screens _on top of_ screens and compose
//! complex-looking menus and HUDs.
use crate::project::{
    resource::{ExternalResource, Handle, Resource},
    room::{ObjectLayerDescriptor, ObjectLayerInstance},
};
use serde::{Deserialize, Serialize};

/// Describes how a scene will look and behave when invoked in-game.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenDescriptor {
    name: String,
    description: Option<String>,
    content: ScreenContentDescriptor,
}

/// Describes the content of a scene for when it's invoked in-game.
#[derive(Debug, Serialize, Deserialize)]
pub enum ScreenContentDescriptor {
    Room(ExternalResource),
    Objects(SceneObjectsDescriptor),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneRoomDescriptor {
    objects: ObjectLayerDescriptor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneObjectsDescriptor {
    objects: ObjectLayerDescriptor,
}

/// A scene running in the game.
pub struct ScreenInstance<'game> {
    pub descriptor: &'game ScreenDescriptor,
    pub content: ScreenContent<'game>,
}

pub enum ScreenContent<'game> {
    Room(Handle<Resource>),
    Objects(ObjectLayerInstance<'game>),
}
