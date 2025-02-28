use super::object::ObjectInstance;
use serde::{Deserialize, Serialize};

/// A Room in the project, it is a place where
/// a player can walk around, interact with stuff, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    name: String,
    description: String,
    layers: Vec<RoomLayer>,
}

/// A Layer of a Room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomLayer {
    name: String,
    data: RoomLayerData,
    scroll_factors: (f32, f32),
}

/// Data for a [`RoomLayer`], contains the editable content for the layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoomLayerData {
    /// A layer containing a single image;
    ImageLayer,
    /// A layer containing tiles chosen from a [`TileMap`];
    TileLayer,
    /// A layer containing many object instances;
    ObjectLayer(ObjectLayer),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectLayer {
    objects: Vec<ObjectInstance>,
}
