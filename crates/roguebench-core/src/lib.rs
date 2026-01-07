//! Platform-agnostic types for roguebench.
//!
//! This crate contains pure data structures with no Bevy dependency.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Definition of an entity as stored in the content database.
///
/// This is the "template" that gets authored via the web editor.
/// The runtime spawns game entities based on these definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDef {
    pub id: Uuid,
    pub name: String,
}

impl EntityDef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
        }
    }
}

pub mod prelude {
    pub use crate::EntityDef;
}
