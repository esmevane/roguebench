//! Roguebench game server.
//!
//! Runs both the web editor API (axum) and the game server (Bevy + Lightyear).

use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use anyhow::Result;
use bevy::prelude::*;
use lightyear::prelude::server::ServerPlugins;
use roguebench_editor::prelude::*;
use roguebench_engine::prelude::*;
use roguebench_protocol::prelude::*;
use roguebench_storage::SqliteStore;
use tokio::sync::mpsc;

/// Port for the web editor.
const WEB_PORT: u16 = 8080;

/// Port for the game server (Lightyear).
const GAME_PORT: u16 = 5000;

/// Server address for Lightyear.
const SERVER_ADDR: SocketAddr = SocketAddr::new(
    std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    GAME_PORT,
);

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("roguebench_server=info,roguebench_engine=info,roguebench_editor=info,lightyear=warn")
        .init();

    // Initialize storage
    let store = Arc::new(SqliteStore::open("entities.db")?);

    // Channel for editor -> engine messages
    let (message_tx, message_rx) = mpsc::unbounded_channel();

    // Start web editor in background
    let editor_config = EditorConfig {
        storage: store.clone(),
        message_tx,
        listen_addr: SocketAddr::from((Ipv4Addr::LOCALHOST, WEB_PORT)),
    };
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(roguebench_editor::run(editor_config));
    });

    tracing::info!("Web editor at http://localhost:{}", WEB_PORT);
    tracing::info!("Game server on UDP port {}", GAME_PORT);

    // Run Bevy app with engine plugin
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(ServerPlugins {
            tick_duration: tick_duration(),
        })
        .add_plugins(ProtocolPlugin)
        .add_plugins(EnginePlugin::new(store, message_rx, SERVER_ADDR))
        .run();

    Ok(())
}
