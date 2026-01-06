//! Platform-agnostic types for Roguebench.
//!
//! This crate contains pure data structures with no Bevy dependency,
//! enabling use in tooling, serialization, and non-game contexts.

pub mod commands;
pub mod items;

pub mod prelude {
    pub use crate::commands::{Command, CommandId, CommandMeta, CommandResult, Envelope, ValidationError};
    pub use crate::items::{Effect, ItemDefinition, ItemId, ItemType};
}
