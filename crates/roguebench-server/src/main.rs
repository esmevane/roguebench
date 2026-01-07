//! roguebench-server: Game server with embedded web editor
//!
//! Runs two services:
//! 1. Web editor at localhost:8080 (axum serving HTML forms)
//! 2. Game server (Bevy headless with Lightyear for networking)

mod editor;

use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use lightyear_udp::server::ServerUdpIo;
use roguebench_core::{Database, EnemyDefinition};
use roguebench_protocol::{Enemy, Position, ProtocolPlugin, FIXED_TIMESTEP_HZ, SERVER_PORT};
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Shared application state for axum handlers
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
}

/// Bevy resource wrapping database access
#[derive(Resource)]
struct DatabaseResource(Arc<Mutex<Database>>);

/// Tracks spawned enemies by their template ID
#[derive(Resource, Default)]
struct SpawnedEnemies {
    entities: std::collections::HashMap<String, Entity>,
}

fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("roguebench=debug".parse().unwrap()),
        )
        .init();

    tracing::info!("Starting roguebench-server");

    // Open database
    let db_path = "roguebench.db";
    let db = Database::open(db_path)?;
    tracing::info!("Database opened at {}", db_path);

    let db = Arc::new(Mutex::new(db));

    // Start web editor in background thread
    let editor_db = db.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let state = AppState { db: editor_db };
            if let Err(e) = editor::run_editor(state).await {
                tracing::error!("Editor error: {}", e);
            }
        });
    });

    // Build and run Bevy app
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(ServerPlugins {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        })
        .add_plugins(ProtocolPlugin)
        .insert_resource(DatabaseResource(db))
        .init_resource::<SpawnedEnemies>()
        .add_systems(Startup, setup_server)
        .add_systems(Update, sync_enemies_from_db)
        .run();

    Ok(())
}

/// Initial server setup - spawn the UDP listener
fn setup_server(mut commands: Commands) {
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), SERVER_PORT);

    tracing::info!("Server running. Editor at http://localhost:8080");
    tracing::info!("Game server listening on UDP port {}", SERVER_PORT);

    // Spawn the server's network listener
    commands.spawn((
        Server::default(),
        ServerUdpIo::default(),
        LocalAddr(addr),
    ));
}

/// Sync enemy entities with database definitions
fn sync_enemies_from_db(
    mut commands: Commands,
    db: Res<DatabaseResource>,
    mut spawned: ResMut<SpawnedEnemies>,
    time: Res<Time>,
    mut last_sync: Local<f32>,
) {
    // Sync every 2 seconds
    *last_sync += time.delta_secs();
    if *last_sync < 2.0 {
        return;
    }
    *last_sync = 0.0;

    let definitions = {
        let db = db.0.lock().unwrap();
        db.get_all_enemies().unwrap_or_default()
    };

    // Track which templates we've seen
    let mut seen_ids: std::collections::HashSet<String> = std::collections::HashSet::new();

    for (i, def) in definitions.iter().enumerate() {
        seen_ids.insert(def.id.0.clone());

        if spawned.entities.contains_key(&def.id.0) {
            // Entity exists - skip (updates would need Query access)
        } else {
            // Spawn new enemy
            let x = -200.0 + (i as f32 * 150.0);
            let entity = spawn_enemy(&mut commands, def, x, 0.0);
            spawned.entities.insert(def.id.0.clone(), entity);
            tracing::info!("Spawned enemy: {} at ({}, 0)", def.name, x);
        }
    }

    // Remove entities for deleted definitions
    let to_remove: Vec<String> = spawned
        .entities
        .keys()
        .filter(|id| !seen_ids.contains(*id))
        .cloned()
        .collect();

    for id in to_remove {
        if let Some(entity) = spawned.entities.remove(&id) {
            commands.entity(entity).despawn();
            tracing::info!("Despawned enemy: {}", id);
        }
    }
}

/// Spawn an enemy entity with replication
fn spawn_enemy(commands: &mut Commands, def: &EnemyDefinition, x: f32, y: f32) -> Entity {
    commands
        .spawn((
            Enemy {
                template_id: def.id.0.clone(),
                name: def.name.clone(),
                health: def.health,
            },
            Position(bevy::math::Vec2::new(x, y)),
            // Replicate to all connected clients
            Replicate::to_clients(NetworkTarget::All),
        ))
        .id()
}
