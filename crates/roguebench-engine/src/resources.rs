//! Bevy resources for the engine.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use roguebench_protocol::EditorMessage;
use roguebench_storage::ContentStore;
use tokio::sync::mpsc;

/// Configuration for the engine plugin.
///
/// Note: Uses Mutex for the receiver since Plugin::build takes &self
/// but we need to take ownership of the non-Clone receiver, and Plugin
/// requires Send + Sync.
pub struct EngineConfig {
    /// Content storage backend.
    pub storage: Arc<dyn ContentStore>,
    /// Receiver for editor messages (wrapped for thread-safe interior mutability).
    pub(crate) editor_receiver: Mutex<Option<mpsc::UnboundedReceiver<EditorMessage>>>,
    /// Address for the Lightyear server.
    pub server_addr: SocketAddr,
}

/// Resource holding the content store.
#[derive(Resource)]
pub struct Storage(pub Arc<dyn ContentStore>);

/// Resource holding the server address.
#[derive(Resource)]
pub struct ServerAddr(pub SocketAddr);

/// Resource for receiving messages from the web editor.
#[derive(Resource)]
pub struct EditorReceiver(pub mpsc::UnboundedReceiver<EditorMessage>);
