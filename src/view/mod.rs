//! Contains traits and structures for viewing/interacting with data
//! through a graphical interface.
//!
//! ## The Editor Pattern
//!
//! The editor pattern is a pattern containing three related parts:
//!
//! - The Data or The Model: contains data and internal behaviours of objects and structures;
//! - The View: Visualises the data, by rendering it to the user. It does effort to stay up to date by using reactive data structures such as [`Mutable`].
//! - The Editor: Edits data directly or calls functions. The editor never directly changes a View, it relies on the reactiveness of the data to propagate the change.
//!
//! This approach can be combined with the inherent structure of data to create a well organised and coherent user interface
//! for data that has already been proven to be itself well-modeled.
//!
//! This approach might not feel that revolutionary: is that not what we always do when creating UI?
//!
//! Kind of, but having specific traits for that helps visualise the pattern and enforce it in practice, as the pattern sometimes
//! might be hard to mentally conceive for data types that are too complex.
//!
//! Consider a type `Person`:
//!
//! ```rust
//! struct Person {
//!     name: String,
//!     age: i32,
//! }
//! ```
//!
//! Creating UI for a `Person` does not require that you create all components from scratch. For a product type such as a struct,
//! you might use a linear container that aggregates two smaller UI elements: one for editing/viewing the name, one for editing/viewing the age.
//!
//! Think of it like `serde` does serialization, but for consumption and interaction with data.
//!
//! ### RPG Engine
//!
//! This is no different for video games, either for the level/resources editor and for the running game.
//! A game's logic and behaviour is mostly an internal endeavour, and then graphics are assigned to each element:
//! Screens, Maps, Tiles, Objects, the Player, Dialogue, even audio exists and reacts to the internal logic.
//!
//! Input, too, exists to change internal data / call events neither which directly influence graphics,
//! but change data that then changes what's rendered on screen.

use futures_signals::signal::Mutable;

/// Describes data that can be viewed reactively by an UI consumer downstream.
pub trait View {
    type TView;

    /// Creates UI for a consumer to _view_ this data.
    fn view(&self) -> Mutable<Self::TView>;
}

/// Describes data that can be edited (propagating changes) by an UI consumer downstream.
pub trait Edit {
    type TEdit;

    /// Creates UI for a consumer to _edit_ this data.
    fn edit(&mut self) -> Mutable<Self::TEdit>;
}
