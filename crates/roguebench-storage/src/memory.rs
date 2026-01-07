//! In-memory content storage for testing.

use std::collections::HashMap;
use std::sync::Mutex;

use uuid::Uuid;

use crate::{ContentStore, Result, StorageError};
use roguebench_core::EntityDef;

/// In-memory content store.
///
/// Useful for testing without database overhead.
/// Not suitable for production - data is lost when dropped.
pub struct MemoryStore {
    entities: Mutex<HashMap<Uuid, EntityDef>>,
}

impl MemoryStore {
    /// Create a new empty in-memory store.
    pub fn new() -> Self {
        Self {
            entities: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentStore for MemoryStore {
    fn load_entities(&self) -> Result<Vec<EntityDef>> {
        let entities = self.entities.lock().unwrap();
        Ok(entities.values().cloned().collect())
    }

    fn save_entity(&self, entity: &EntityDef) -> Result<()> {
        let mut entities = self.entities.lock().unwrap();
        entities.insert(entity.id, entity.clone());
        Ok(())
    }

    fn delete_entity(&self, id: Uuid) -> Result<()> {
        let mut entities = self.entities.lock().unwrap();
        if entities.remove(&id).is_none() {
            return Err(StorageError::NotFound(id.to_string()));
        }
        Ok(())
    }
}
