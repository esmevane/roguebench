//! SQLite schema and database operations.

use refinery::embed_migrations;
use roguebench_core::items::{ItemDefinition, ItemId};
use rusqlite::{Connection, params};
use thiserror::Error;

// Embed migrations from the migrations folder
embed_migrations!("migrations");

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Item not found: {0}")]
    ItemNotFound(String),
    #[error("Migration error: {0}")]
    Migration(#[from] refinery::Error),
}

/// Game content database backed by SQLite.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open or create a database at the given path.
    pub fn open(path: &str) -> Result<Self, DatabaseError> {
        let mut conn = Connection::open(path)?;
        migrations::runner().run(&mut conn)?;
        Ok(Self { conn })
    }

    /// Create an in-memory database (for testing).
    pub fn in_memory() -> Result<Self, DatabaseError> {
        let mut conn = Connection::open_in_memory()?;
        migrations::runner().run(&mut conn)?;
        Ok(Self { conn })
    }

    /// Insert or update an item definition.
    pub fn upsert_item(&self, item: &ItemDefinition) -> Result<(), DatabaseError> {
        let data = serde_json::to_string(item)?;
        let item_type = format!("{:?}", item.item_type).to_lowercase();

        self.conn.execute(
            "INSERT INTO items (id, name, item_type, data, updated_at)
             VALUES (?1, ?2, ?3, ?4, unixepoch())
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                item_type = excluded.item_type,
                data = excluded.data,
                updated_at = unixepoch()",
            params![item.id.0, item.name, item_type, data],
        )?;
        Ok(())
    }

    /// Get an item definition by ID.
    pub fn get_item(&self, id: &ItemId) -> Result<ItemDefinition, DatabaseError> {
        let data: String = self.conn.query_row(
            "SELECT data FROM items WHERE id = ?1",
            params![id.0],
            |row| row.get(0),
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DatabaseError::ItemNotFound(id.0.clone()),
            other => DatabaseError::Sqlite(other),
        })?;

        let item: ItemDefinition = serde_json::from_str(&data)?;
        Ok(item)
    }

    /// Get all item definitions.
    pub fn get_all_items(&self) -> Result<Vec<ItemDefinition>, DatabaseError> {
        let mut stmt = self.conn.prepare("SELECT data FROM items")?;
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

    /// Delete an item by ID.
    pub fn delete_item(&self, id: &ItemId) -> Result<bool, DatabaseError> {
        let affected = self.conn.execute(
            "DELETE FROM items WHERE id = ?1",
            params![id.0],
        )?;
        Ok(affected > 0)
    }

    /// Count total items in database.
    pub fn count_items(&self) -> Result<usize, DatabaseError> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM items",
            [],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roguebench_core::items::{Effect, ItemType};

    #[test]
    fn database_crud_operations() {
        let db = Database::in_memory().unwrap();

        // Create
        let potion = ItemDefinition::new("health_potion", "Health Potion", ItemType::Consumable)
            .with_description("Restores 50 health")
            .with_effect(Effect::ModifyStat {
                stat: "health".into(),
                amount: 50,
            })
            .stackable(99);

        db.upsert_item(&potion).unwrap();
        assert_eq!(db.count_items().unwrap(), 1);

        // Read
        let loaded = db.get_item(&ItemId::new("health_potion")).unwrap();
        assert_eq!(loaded.name, "Health Potion");
        assert_eq!(loaded.effects.len(), 1);

        // Update
        let updated = ItemDefinition::new("health_potion", "Greater Health Potion", ItemType::Consumable)
            .with_effect(Effect::ModifyStat {
                stat: "health".into(),
                amount: 100,
            });
        db.upsert_item(&updated).unwrap();
        assert_eq!(db.count_items().unwrap(), 1);

        let reloaded = db.get_item(&ItemId::new("health_potion")).unwrap();
        assert_eq!(reloaded.name, "Greater Health Potion");

        // Delete
        assert!(db.delete_item(&ItemId::new("health_potion")).unwrap());
        assert_eq!(db.count_items().unwrap(), 0);
    }

    #[test]
    fn get_all_items() {
        let db = Database::in_memory().unwrap();

        let items = vec![
            ItemDefinition::new("sword", "Iron Sword", ItemType::Equipment),
            ItemDefinition::new("potion", "Potion", ItemType::Consumable),
            ItemDefinition::new("key", "Dungeon Key", ItemType::Key),
        ];

        for item in &items {
            db.upsert_item(item).unwrap();
        }

        let all = db.get_all_items().unwrap();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn item_not_found() {
        let db = Database::in_memory().unwrap();
        let result = db.get_item(&ItemId::new("nonexistent"));
        assert!(matches!(result, Err(DatabaseError::ItemNotFound(_))));
    }
}
