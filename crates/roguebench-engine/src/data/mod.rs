//! Data pipeline for loading and managing game content.
//!
//! Content is stored in SQLite and loaded at runtime.

mod content;
mod impls;
pub mod schema;
mod watcher;

pub use content::{
    ChangeType, ContentAppExt, ContentChanged, ContentRegistry, ContentType, LoadError,
};
pub use schema::Database;
pub use watcher::{
    ContentWatcher, ContentWatcherAppExt, ContentWatcherPlugin, FileChanged, WatchError,
};
