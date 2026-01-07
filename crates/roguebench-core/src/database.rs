//! Database connection and migrations

use crate::Result;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use std::path::Path;

/// Define migrations as SQL strings.
/// Each M::up creates a migration step.
fn migrations() -> Migrations<'static> {
    Migrations::new(vec![
        // V1: Create enemies table
        M::up(
            r#"
            CREATE TABLE enemies (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                health INTEGER NOT NULL DEFAULT 100,
                created_at INTEGER NOT NULL DEFAULT (unixepoch()),
                updated_at INTEGER NOT NULL DEFAULT (unixepoch())
            );
            CREATE INDEX idx_enemies_name ON enemies(name);
            "#,
        ),
    ])
}

/// Database connection wrapper.
///
/// Provides access to SQLite storage for all content types.
/// The database file is the source of truth for authored content.
pub struct Database {
    pub(crate) conn: Connection,
}

impl Database {
    /// Open a database connection and run any pending migrations.
    ///
    /// Creates the database file if it doesn't exist.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut conn = Connection::open(path)?;

        // Enable foreign keys
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        // Run migrations
        migrations().to_latest(&mut conn)?;

        Ok(Self { conn })
    }

    /// Open an in-memory database (for testing)
    pub fn open_in_memory() -> Result<Self> {
        let mut conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        // Run migrations
        migrations().to_latest(&mut conn)?;

        Ok(Self { conn })
    }

    /// Get the path to the database file (if file-based)
    pub fn path(&self) -> Option<&str> {
        self.conn.path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_in_memory() {
        let db = Database::open_in_memory().expect("Failed to open in-memory database");
        // In-memory databases have an empty path or None
        let path = db.path();
        assert!(path.is_none() || path == Some(""));
    }

    #[test]
    fn test_migrations_run() {
        let db = Database::open_in_memory().expect("Failed to open database");
        // Verify the enemies table exists by querying it
        let count: i64 = db
            .conn
            .query_row("SELECT COUNT(*) FROM enemies", [], |row| row.get(0))
            .expect("Failed to query enemies table");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_enemy_crud() {
        use crate::{EnemyDefinition, EnemyId};

        let db = Database::open_in_memory().expect("Failed to open database");

        // Create
        let enemy = EnemyDefinition::new("grunt", "Grunt", 50);
        db.upsert_enemy(&enemy).expect("Failed to create enemy");

        // Read
        let loaded = db
            .get_enemy(&EnemyId::new("grunt"))
            .expect("Failed to load enemy");
        assert_eq!(loaded.name, "Grunt");
        assert_eq!(loaded.health, 50);

        // Update
        let updated = EnemyDefinition::new("grunt", "Elite Grunt", 100);
        db.upsert_enemy(&updated).expect("Failed to update enemy");
        let loaded = db.get_enemy(&EnemyId::new("grunt")).unwrap();
        assert_eq!(loaded.name, "Elite Grunt");
        assert_eq!(loaded.health, 100);

        // List
        let all = db.get_all_enemies().expect("Failed to list enemies");
        assert_eq!(all.len(), 1);

        // Delete
        db.delete_enemy(&EnemyId::new("grunt"))
            .expect("Failed to delete enemy");
        let all = db.get_all_enemies().expect("Failed to list enemies");
        assert_eq!(all.len(), 0);
    }
}
