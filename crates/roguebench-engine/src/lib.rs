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

#[cfg(test)]
mod tests {
    use super::*;
    use roguebench_core::EntityDef;
    use roguebench_protocol::EntityName;
    use roguebench_storage::MemoryStore;
    use systems::{ReloadEntities, SpawnedEntity};

    /// Create a minimal test app with storage and editor receiver.
    fn test_app(
        storage: Arc<dyn ContentStore>,
    ) -> (App, mpsc::UnboundedSender<EditorMessage>) {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Add storage
        app.insert_resource(Storage(storage));

        // Add editor channel
        let (tx, rx) = mpsc::unbounded_channel();
        app.insert_resource(EditorReceiver(rx));

        // Register the message type and add systems/observers
        app.add_message::<ReloadEntities>();
        app.add_systems(Update, systems::check_editor_messages);
        app.add_observer(systems::reload_entities);

        (app, tx)
    }

    #[test]
    fn reload_entities_spawns_from_storage() {
        let storage = Arc::new(MemoryStore::new());

        // Pre-populate storage
        let goblin = EntityDef::new("Goblin");
        let orc = EntityDef::new("Orc");
        storage.save_entity(&goblin).unwrap();
        storage.save_entity(&orc).unwrap();

        let (mut app, _tx) = test_app(storage);

        // Trigger reload
        app.world_mut().commands().trigger(ReloadEntities);
        app.update();

        // Verify entities were spawned
        let mut query = app.world_mut().query::<(&SpawnedEntity, &EntityName)>();
        let names: Vec<String> = query
            .iter(app.world())
            .map(|(_, name)| name.0.clone())
            .collect();

        assert_eq!(names.len(), 2);
        assert!(names.contains(&"Goblin".to_string()));
        assert!(names.contains(&"Orc".to_string()));
    }

    #[test]
    fn reload_entities_despawns_existing() {
        let storage = Arc::new(MemoryStore::new());

        // Pre-populate with one entity
        let goblin = EntityDef::new("Goblin");
        storage.save_entity(&goblin).unwrap();

        let (mut app, _tx) = test_app(storage.clone());

        // First reload
        app.world_mut().commands().trigger(ReloadEntities);
        app.update();

        let mut query = app.world_mut().query::<&SpawnedEntity>();
        let count = query.iter(app.world()).count();
        assert_eq!(count, 1);

        // Add another entity to storage
        let orc = EntityDef::new("Orc");
        storage.save_entity(&orc).unwrap();

        // Second reload should despawn old and spawn both
        app.world_mut().commands().trigger(ReloadEntities);
        app.update();

        let mut query = app.world_mut().query::<(&SpawnedEntity, &EntityName)>();
        let names: Vec<String> = query
            .iter(app.world())
            .map(|(_, name)| name.0.clone())
            .collect();

        assert_eq!(names.len(), 2);
        assert!(names.contains(&"Goblin".to_string()));
        assert!(names.contains(&"Orc".to_string()));
    }

    #[test]
    fn check_editor_messages_triggers_reload() {
        let storage = Arc::new(MemoryStore::new());

        // Pre-populate with entity
        let goblin = EntityDef::new("Goblin");
        storage.save_entity(&goblin).unwrap();

        let (mut app, tx) = test_app(storage);

        // Send reload message through channel
        tx.send(EditorMessage::ReloadEntities).unwrap();

        // Run update - should process message and trigger reload
        app.update();

        // Verify entity was spawned (proves the message was processed)
        let mut query = app.world_mut().query::<&SpawnedEntity>();
        let count = query.iter(app.world()).count();
        assert_eq!(count, 1);
    }
}
