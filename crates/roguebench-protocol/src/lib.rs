//! Network protocol and shared components for roguebench.
//!
//! This crate contains replicated components and messages shared between client and server.

use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub use roguebench_core::prelude::*;

/// Fixed timestep for network synchronization (60 Hz).
pub const FIXED_TIMESTEP_HZ: f64 = 60.0;

/// Tick duration derived from the fixed timestep.
pub fn tick_duration() -> Duration {
    Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ)
}

/// Replicated component for entity identity.
///
/// Links a runtime entity back to its authored definition.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EntityName(pub String);

/// Channel for reliable ordered messages.
pub struct ReliableChannel;

/// Messages from the editor to the engine.
///
/// These are sent via an in-process channel when the editor
/// modifies content that the engine should reload.
#[derive(Debug, Clone)]
pub enum EditorMessage {
    /// Entities have been modified; reload from storage.
    ReloadEntities,
}

/// Plugin that registers the network protocol.
pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Register replicated components
        app.register_component::<EntityName>();

        // Register channels
        app.add_channel::<ReliableChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}

pub mod prelude {
    pub use crate::{
        tick_duration, EditorMessage, EntityName, ProtocolPlugin, ReliableChannel,
        FIXED_TIMESTEP_HZ,
    };
    pub use roguebench_core::prelude::*;
}
