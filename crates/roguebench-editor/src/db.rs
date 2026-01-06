//! Database wrapper for the editor API.

use roguebench_core::items::ItemDefinition;
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Item not found: {0}")]
    NotFound(String),
    #[error("Lock error")]
    Lock,
}

/// Thread-safe database handle for the editor API.
#[derive(Clone)]
pub struct EditorDb {
    conn: Arc<Mutex<Connection>>,
}

impl EditorDb {
    /// Open or create a database at the given path.
    pub fn open(path: &str) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Create an in-memory database (for testing).
    pub fn in_memory() -> Result<Self, DbError> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|_| DbError::Lock)?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS items (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                item_type TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (unixepoch()),
                updated_at INTEGER NOT NULL DEFAULT (unixepoch())
            );
            CREATE INDEX IF NOT EXISTS idx_items_type ON items(item_type);
            CREATE INDEX IF NOT EXISTS idx_items_name ON items(name);
            "
        )?;
        Ok(())
    }

    /// Get all items.
    pub fn list_items(&self) -> Result<Vec<ItemDefinition>, DbError> {
        let conn = self.conn.lock().map_err(|_| DbError::Lock)?;
        let mut stmt = conn.prepare("SELECT data FROM items ORDER BY name")?;
        let items = stmt.query_map([], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?;

        let mut result = Vec::new();
        for data in items {
            let item: ItemDefinition = serde_json::from_str(&data?)?;
            result.push(item);
        }
        Ok(result)
    }

    /// Get an item by ID.
    pub fn get_item(&self, id: &str) -> Result<ItemDefinition, DbError> {
        let conn = self.conn.lock().map_err(|_| DbError::Lock)?;
        let data: String = conn.query_row(
            "SELECT data FROM items WHERE id = ?1",
            params![id],
            |row| row.get(0),
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DbError::NotFound(id.to_string()),
            other => DbError::Sqlite(other),
        })?;

        let item: ItemDefinition = serde_json::from_str(&data)?;
        Ok(item)
    }

    /// Create a new item.
    pub fn create_item(&self, item: &ItemDefinition) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|_| DbError::Lock)?;
        let data = serde_json::to_string(item)?;
        let item_type = format!("{:?}", item.item_type).to_lowercase();

        conn.execute(
            "INSERT INTO items (id, name, item_type, data) VALUES (?1, ?2, ?3, ?4)",
            params![item.id.0, item.name, item_type, data],
        )?;
        Ok(())
    }

    /// Update an existing item.
    pub fn update_item(&self, id: &str, item: &ItemDefinition) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|_| DbError::Lock)?;
        let data = serde_json::to_string(item)?;
        let item_type = format!("{:?}", item.item_type).to_lowercase();

        let affected = conn.execute(
            "UPDATE items SET name = ?1, item_type = ?2, data = ?3, updated_at = unixepoch() WHERE id = ?4",
            params![item.name, item_type, data, id],
        )?;

        if affected == 0 {
            return Err(DbError::NotFound(id.to_string()));
        }
        Ok(())
    }

    /// Delete an item.
    pub fn delete_item(&self, id: &str) -> Result<bool, DbError> {
        let conn = self.conn.lock().map_err(|_| DbError::Lock)?;
        let affected = conn.execute("DELETE FROM items WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }
}
