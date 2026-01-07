//! SQLite-backed content storage.

use std::path::Path;
use std::sync::Mutex;

use rusqlite::Connection;

use crate::{ContentStore, Result, StorageError};
use roguebench_core::EntityDef;

/// SQLite-backed content store.
///
/// Thread-safe via internal mutex. Suitable for use from both
/// async (web) and sync (Bevy) contexts.
pub struct SqliteStore {
    conn: Mutex<Connection>,
}

impl SqliteStore {
    /// Open a SQLite database at the given path.
    ///
    /// Creates the database and tables if they don't exist.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path)?;
        let store = Self {
            conn: Mutex::new(conn),
        };
        store.init_schema()?;
        Ok(store)
    }

    /// Open an in-memory SQLite database.
    ///
    /// Useful for testing without touching the filesystem.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let store = Self {
            conn: Mutex::new(conn),
        };
        store.init_schema()?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS entities (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }
}

impl ContentStore for SqliteStore {
    fn load_entities(&self) -> Result<Vec<EntityDef>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name FROM entities")?;

        let mut entities = Vec::new();
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            Ok((id, name))
        })?;

        for row_result in rows {
            let (id_str, name) = row_result?;
            let id = id_str
                .parse()
                .map_err(|_| StorageError::InvalidUuid(id_str.clone()))?;
            entities.push(EntityDef { id, name });
        }

        Ok(entities)
    }

    fn save_entity(&self, entity: &EntityDef) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO entities (id, name) VALUES (?1, ?2)",
            [&entity.id.to_string(), &entity.name],
        )?;
        Ok(())
    }

    fn delete_entity(&self, id: uuid::Uuid) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute("DELETE FROM entities WHERE id = ?1", [&id.to_string()])?;
        if rows == 0 {
            return Err(StorageError::NotFound(id.to_string()));
        }
        Ok(())
    }
}
