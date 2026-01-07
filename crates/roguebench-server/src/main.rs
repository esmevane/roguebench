//! roguebench-server: Game server with embedded web editor
//!
//! This binary runs two services:
//! 1. Web editor at localhost:8080 (axum serving HTML forms)
//! 2. Game server (Bevy headless, will add Lightyear later)

mod editor;

use roguebench_core::Database;
use std::sync::{Arc, Mutex};

/// Shared application state accessible by axum handlers
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("roguebench=debug".parse().unwrap()),
        )
        .init();

    tracing::info!("Starting roguebench-server");

    // Open database (creates if doesn't exist)
    let db_path = "roguebench.db";
    let db = Database::open(db_path)?;
    tracing::info!("Database opened at {}", db_path);

    // Create shared state
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    // Run the web editor
    tracing::info!("Server running. Editor at http://localhost:8080");
    editor::run_editor(state).await?;

    Ok(())
}
