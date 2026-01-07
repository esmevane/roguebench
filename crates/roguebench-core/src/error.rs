//! Error types for roguebench-core

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Content not found: {kind} with id '{id}'")]
    NotFound { kind: &'static str, id: String },

    #[error("Migration error: {0}")]
    Migration(#[from] rusqlite_migration::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
