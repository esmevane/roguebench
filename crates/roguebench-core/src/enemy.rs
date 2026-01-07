//! Enemy definition content type

use crate::{ContentType, Database, Error, Result};
use serde::{Deserialize, Serialize};

/// Unique identifier for an enemy definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnemyId(pub String);

impl EnemyId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for EnemyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Definition of an enemy type that can be authored via the editor.
///
/// This is the template from which enemy instances are spawned.
/// For the walking skeleton, we keep it minimal: name and health.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyDefinition {
    /// Unique identifier for this enemy type
    pub id: EnemyId,
    /// Display name
    pub name: String,
    /// Starting health points
    pub health: i32,
}

impl EnemyDefinition {
    pub fn new(id: impl Into<String>, name: impl Into<String>, health: i32) -> Self {
        Self {
            id: EnemyId::new(id),
            name: name.into(),
            health,
        }
    }
}

impl ContentType for EnemyDefinition {
    type Id = EnemyId;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn load_all(db: &Database) -> Result<Vec<Self>> {
        db.get_all_enemies()
    }

    fn type_name() -> &'static str {
        "enemy"
    }
}

// Database operations for EnemyDefinition
impl Database {
    /// Get all enemy definitions from the database
    pub fn get_all_enemies(&self) -> Result<Vec<EnemyDefinition>> {
        let mut stmt = self.conn.prepare("SELECT id, name, health FROM enemies")?;
        let enemies = stmt
            .query_map([], |row| {
                Ok(EnemyDefinition {
                    id: EnemyId(row.get(0)?),
                    name: row.get(1)?,
                    health: row.get(2)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(enemies)
    }

    /// Get a single enemy definition by ID
    pub fn get_enemy(&self, id: &EnemyId) -> Result<EnemyDefinition> {
        self.conn
            .query_row(
                "SELECT id, name, health FROM enemies WHERE id = ?1",
                [&id.0],
                |row| {
                    Ok(EnemyDefinition {
                        id: EnemyId(row.get(0)?),
                        name: row.get(1)?,
                        health: row.get(2)?,
                    })
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => Error::NotFound {
                    kind: "enemy",
                    id: id.0.clone(),
                },
                other => Error::Database(other),
            })
    }

    /// Create or update an enemy definition
    pub fn upsert_enemy(&self, enemy: &EnemyDefinition) -> Result<()> {
        self.conn.execute(
            "INSERT INTO enemies (id, name, health) VALUES (?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET name = ?2, health = ?3, updated_at = unixepoch()",
            (&enemy.id.0, &enemy.name, enemy.health),
        )?;
        Ok(())
    }

    /// Delete an enemy definition
    pub fn delete_enemy(&self, id: &EnemyId) -> Result<()> {
        self.conn
            .execute("DELETE FROM enemies WHERE id = ?1", [&id.0])?;
        Ok(())
    }
}
