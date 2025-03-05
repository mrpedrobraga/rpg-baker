//! # [Engine Name Here]
//!
//! A data-driven game engine with an approachable and fun design that allows for making games without the stress.
//! The engine uses "toyish" constructs -- such as Scratch-like code blocks -- to be easily understood by begginer developers,
//! but also to take advantage of "syntactic sugar" possibilities only visual scripting provides.
//!
//! You may logically conclude it only makes sense to interact with an [Engine Name Here] project via its editor -- and you would be right,
//! but this engine uses open formats (all available here in this crate) that are serialized to JSON and can be easily handled by
//! your own external software.
//!
//! ### Pros
//! - Light, fast enough.
//! - Uses JSON for its text resources, a format that is easy to understand and plays nice with version control.
//! - Resource system that uses UUIDs and thus is resistant to renames and moves.
//! - Ability to use dependencies such as third-party assets, behaviours and tools.
//!
//! ### Cons
//! - Never as fast as an _ad hoc_ engine made for a game very specific needs.
//! - Does not support non-visual scripting (as of the moment).
//! - Does not support 3D games (as of the moment).

pub mod behaviour;
pub mod format;
pub mod plugin;
pub mod project;
pub mod screen;
pub mod view;
