//! Web editor backend for Roguebench content authoring.
//!
//! Provides an axum-based REST API for managing game content
//! stored in SQLite.

pub mod api;
pub mod db;

pub use api::{router, AppState};
pub use db::EditorDb;

pub mod prelude {
    pub use roguebench_core::prelude::*;
    pub use crate::{router, AppState, EditorDb};
}
