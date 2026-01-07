//! Shared protocol for client-server communication
//!
//! Defines replicated components and Lightyear protocol registration.

use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

/// Network configuration constants
pub const FIXED_TIMESTEP_HZ: f64 = 64.0;
pub const SERVER_PORT: u16 = 5000;

/// A spawned enemy in the game world.
///
/// This component is replicated from server to clients.
/// The server spawns enemies based on EnemyDefinition templates.
#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Enemy {
    /// Template ID from the database (e.g., "goblin")
    pub template_id: String,
    /// Display name
    pub name: String,
    /// Current health
    pub health: i32,
}

/// Position component for entities.
///
/// Replicated and interpolated on clients.
#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Position(pub Vec2);

/// Protocol plugin that registers all replicated components.
pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Register components for replication
        // Enemy data changes infrequently, just needs replication
        app.register_component::<Enemy>().add_prediction();

        // Position changes frequently and benefits from interpolation
        app.register_component::<Position>().add_prediction();
    }
}

/// Shared plugin with common configuration.
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProtocolPlugin);
    }
}
