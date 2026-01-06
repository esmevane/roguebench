//! Platform-agnostic types for Roguebench.
//!
//! This crate contains pure data structures with no Bevy dependency,
//! enabling use in tooling, serialization, and non-game contexts.

pub mod items;

pub mod prelude {
    pub use crate::items::{Effect, ItemDefinition, ItemId, ItemType};
}
