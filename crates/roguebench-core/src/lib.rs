//! roguebench-core: Platform-agnostic types and database layer
//!
//! This crate contains no Bevy dependencies. It provides:
//! - Content type definitions (EnemyDefinition, etc.)
//! - Database access layer (SQLite via rusqlite)
//! - Content loading traits

mod content;
mod database;
mod enemy;
mod error;

pub use content::ContentType;
pub use database::Database;
pub use enemy::{EnemyDefinition, EnemyId};
pub use error::{Error, Result};
