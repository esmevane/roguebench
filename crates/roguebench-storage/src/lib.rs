//! Content storage abstraction for roguebench.
//!
//! Provides a [`ContentStore`] trait for persisting and loading game content,
//! with implementations for SQLite (production) and in-memory (testing).

mod memory;
mod sqlite;

pub use memory::MemoryStore;
pub use sqlite::SqliteStore;

use roguebench_core::EntityDef;
use thiserror::Error;

/// Errors that can occur during storage operations.
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Data corruption: invalid UUID '{0}'")]
    InvalidUuid(String),
}

/// Result type for storage operations.
pub type Result<T> = std::result::Result<T, StorageError>;

/// Trait for content storage implementations.
///
/// This is the port in hexagonal architecture terms - both the editor
/// (writing content) and engine (loading content) depend on this abstraction.
pub trait ContentStore: Send + Sync {
    /// Load all entity definitions.
    fn load_entities(&self) -> Result<Vec<EntityDef>>;

    /// Save an entity definition.
    fn save_entity(&self, entity: &EntityDef) -> Result<()>;

    /// Delete an entity by ID.
    fn delete_entity(&self, id: uuid::Uuid) -> Result<()>;
}

pub mod prelude {
    pub use crate::{ContentStore, MemoryStore, Result, SqliteStore, StorageError};
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that exercises the ContentStore contract.
    /// Run against any implementation to verify correctness.
    fn test_roundtrip(store: &dyn ContentStore) {
        // Initially empty
        let entities = store.load_entities().unwrap();
        assert!(entities.is_empty());

        // Save an entity
        let entity = EntityDef::new("Goblin");
        store.save_entity(&entity).unwrap();

        // Load and verify
        let loaded = store.load_entities().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, entity.id);
        assert_eq!(loaded[0].name, entity.name);

        // Save another
        let entity2 = EntityDef::new("Orc");
        store.save_entity(&entity2).unwrap();

        let loaded = store.load_entities().unwrap();
        assert_eq!(loaded.len(), 2);

        // Update existing (same ID, different name)
        let mut updated = entity.clone();
        updated.name = "Goblin King".to_string();
        store.save_entity(&updated).unwrap();

        let loaded = store.load_entities().unwrap();
        assert_eq!(loaded.len(), 2);
        let goblin = loaded.iter().find(|e| e.id == entity.id).unwrap();
        assert_eq!(goblin.name, "Goblin King");

        // Delete
        store.delete_entity(entity.id).unwrap();
        let loaded = store.load_entities().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, entity2.id);
    }

    #[test]
    fn memory_store_roundtrip() {
        let store = MemoryStore::new();
        test_roundtrip(&store);
    }

    #[test]
    fn sqlite_store_roundtrip() {
        let store = SqliteStore::open_in_memory().unwrap();
        test_roundtrip(&store);
    }
}
