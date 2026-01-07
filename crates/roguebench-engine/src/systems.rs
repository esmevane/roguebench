//! Bevy systems for the engine.

use bevy::prelude::*;
use lightyear::prelude::server::{
    NetcodeConfig, NetcodeServer, ServerUdpIo, Start as LightyearStart,
};
use lightyear::prelude::{Link, LocalAddr, Replicate};
use roguebench_protocol::{EditorMessage, EntityName};

use crate::resources::{EditorReceiver, ServerAddr, Storage};

/// Event triggered when entities should be reloaded from storage.
#[derive(Event, Message)]
pub struct ReloadEntities;

/// Marker component for entities spawned from definitions.
#[derive(Component)]
pub struct SpawnedEntity;

/// Spawn the Lightyear server.
pub fn spawn_server(mut commands: Commands, server_addr: Res<ServerAddr>) {
    tracing::info!("Spawning Lightyear server on {}", server_addr.0);

    let server = commands
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr(server_addr.0),
            ServerUdpIo::default(),
        ))
        .id();

    commands.trigger(LightyearStart { entity: server });
}

/// Trigger initial entity load at startup.
pub fn initial_load(mut commands: Commands) {
    commands.trigger(ReloadEntities);
}

/// Check for messages from the editor and dispatch events.
pub fn check_editor_messages(mut editor_rx: ResMut<EditorReceiver>, mut commands: Commands) {
    while let Ok(message) = editor_rx.0.try_recv() {
        match message {
            EditorMessage::ReloadEntities => {
                commands.trigger(ReloadEntities);
            }
        }
    }
}

/// Reload entities from storage when triggered.
pub fn reload_entities(
    _trigger: On<ReloadEntities>,
    mut commands: Commands,
    storage: Res<Storage>,
    existing: Query<Entity, With<SpawnedEntity>>,
) {
    tracing::info!("Reloading entities from storage");

    // Despawn existing entities
    for entity in existing.iter() {
        commands.entity(entity).despawn();
    }

    // Load and spawn new entities
    match storage.0.load_entities() {
        Ok(entities) => {
            for entity_def in entities {
                tracing::info!("Spawning entity: {}", entity_def.name);
                commands.spawn((SpawnedEntity, EntityName(entity_def.name), Replicate::default()));
            }
        }
        Err(e) => {
            tracing::error!("Failed to load entities: {}", e);
        }
    }
}

/// Log new connections.
pub fn log_connections(trigger: On<Add, Link>) {
    tracing::info!("New link added: {:?}", trigger.entity);
}
