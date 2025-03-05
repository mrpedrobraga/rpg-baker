use super::object::{ObjectInstance, ObjectInstanceDescriptor};
use serde::{Deserialize, Serialize};

/// Describes a Room in the project, a container for
/// layers that themselves contain tiles, objects,
/// events, colliders, hazards and more.
///
/// Most games will use rooms in some regard,
/// from RPGs to platformers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomDescriptor {
    name: String,
    description: Option<String>,
    layers: Vec<RoomLayer>,
}

/// A Layer of a Room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomLayer {
    name: String,
    scroll_factor: (f32, f32),
    #[serde(flatten)]
    content: RoomLayerContent,
}

/// Data for a [`RoomLayer`], contains the editable content for the layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum RoomLayerContent {
    /// This layer contains a single image;
    Image,
    /// This layer contains tiles chosen from a TileMap;
    Tiles,
    /// This layer contains many object instances;
    Objects(ObjectLayerDescriptor),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectLayerDescriptor {
    objects: Vec<ObjectInstanceDescriptor>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoomInstance<'game> {
    descriptor: &'game RoomDescriptor,
    layers: Vec<RoomLayer>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ObjectLayerInstance<'game> {
    objects: Vec<ObjectInstance<'game>>,
}
