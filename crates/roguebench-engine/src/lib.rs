//! Bevy game engine plugin for roguebench.
//!
//! Provides the core game systems for entity spawning, reloading, and
//! integration with the content storage layer.

mod resources;
mod systems;

pub use resources::{EditorReceiver, EngineConfig, Storage};

use std::net::SocketAddr;
use std::sync::Arc;

use bevy::prelude::*;
use roguebench_protocol::EditorMessage;
use roguebench_storage::ContentStore;
use tokio::sync::mpsc;

/// Main engine plugin for roguebench server.
///
/// Handles game world initialization and synchronization from content storage.
pub struct EnginePlugin {
    /// Configuration for the engine.
    pub config: EngineConfig,
}

impl EnginePlugin {
    /// Create a new engine plugin with the given configuration.
    pub fn new(
        storage: Arc<dyn ContentStore>,
        editor_receiver: mpsc::UnboundedReceiver<EditorMessage>,
        server_addr: SocketAddr,
    ) -> Self {
        use std::sync::Mutex;
        Self {
            config: EngineConfig {
                storage,
                editor_receiver: Mutex::new(Some(editor_receiver)),
                server_addr,
            },
        }
    }
}

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        // Insert resources from config
        app.insert_resource(Storage(self.config.storage.clone()));
        app.insert_resource(resources::ServerAddr(self.config.server_addr));

        // Take ownership of the receiver (uses interior mutability)
        let receiver = self
            .config
            .editor_receiver
            .lock()
            .unwrap()
            .take()
            .expect("EnginePlugin can only be added once");
        app.insert_resource(EditorReceiver(receiver));

        // Register messages (events)
        app.add_message::<systems::ReloadEntities>();

        // Add systems
        app.add_systems(Startup, systems::spawn_server);
        app.add_systems(Startup, systems::initial_load);
        app.add_systems(Update, systems::check_editor_messages);

        // Add observers
        app.add_observer(systems::reload_entities);
        app.add_observer(systems::log_connections);
    }
}

pub mod prelude {
    pub use crate::{EngineConfig, EnginePlugin};
}
