//! Bevy plugins and systems for Roguebench.
//!
//! This is the main game logic crate, containing all ECS systems,
//! plugins, and runtime behavior.

use bevy::prelude::*;

pub mod commands;
pub mod data;
pub mod editor;
pub mod items;

pub mod prelude {
    pub use roguebench_core::prelude::*;

    pub use crate::commands::{
        CommandBus, CommandBusAppExt, CommandBusPlugin, CommandEventAppExt, CommandExecuted,
        CommandLog, CommandLogAppExt, ExecutionTimer, FrameCount, LogEntry, Validator,
        ValidatorAppExt, Validators,
    };
    pub use crate::data::Database;
    pub use crate::editor::{EditorPlugin, ItemEditorPlugin};
    pub use crate::items::{
        Item, ItemBundle, ItemPickedUp, ItemRegistry, ItemUsed, ItemsPlugin, Pickup, SpawnItem,
    };
}

/// Main plugin that registers all Roguebench systems.
pub struct RoguebenchPlugin;

impl Plugin for RoguebenchPlugin {
    fn build(&self, _app: &mut App) {
        // Plugins will be registered here as they're created
    }
}
