use super::object::ObjectInstance;
use serde::{Deserialize, Serialize};

/// A Room in the project, it is a place where
/// a player can walk around, interact with stuff, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
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
    Objects(ObjectLayer),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectLayer {
    objects: Vec<ObjectInstance>,
}
