//! Roguebench game client.
//!
//! Connects to the game server via Lightyear and renders entities.

use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use bevy::prelude::*;
use lightyear::netcode::Key;
use lightyear::prelude::client::{
    ClientPlugins, Connect as LightyearConnect, NetcodeClient, NetcodeConfig,
};
use lightyear::prelude::{Authentication, LocalAddr, PeerAddr, UdpIo, *};
use roguebench_protocol::prelude::*;

/// Server address to connect to.
const SERVER_ADDR: SocketAddr = SocketAddr::new(
    std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    5000,
);

/// Client local address (use any available port).
const CLIENT_ADDR: SocketAddr = SocketAddr::new(
    std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
    0,
);

// ============================================================================
// UI
// ============================================================================

#[derive(Component)]
struct EntityLabel;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_labels_for_entities(
    mut commands: Commands,
    new_entities: Query<(Entity, &EntityName), Added<EntityName>>,
) {
    for (entity, name) in new_entities.iter() {
        tracing::info!("Received replicated entity: {}", name.0);

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                EntityLabel,
                Text2d::new(name.0.clone()),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    }
}

fn update_entity_positions(
    mut labels: Query<&mut Transform, With<EntityLabel>>,
    time: Res<Time>,
) {
    // Simple animation to show entities are alive
    for mut transform in labels.iter_mut() {
        transform.translation.y = (time.elapsed_secs() * 2.0).sin() * 5.0;
    }
}

// ============================================================================
// Networking
// ============================================================================

fn spawn_client(mut commands: Commands) {
    tracing::info!("Connecting to server at {}", SERVER_ADDR);

    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: rand::random(),
        private_key: Key::default(),
        protocol_id: 0,
    };

    let netcode_client = NetcodeClient::new(auth, NetcodeConfig::default())
        .expect("Failed to create netcode client");

    let client = commands
        .spawn((
            netcode_client,
            LocalAddr(CLIENT_ADDR),
            PeerAddr(SERVER_ADDR),
            UdpIo::default(),
            ReplicationReceiver::default(),
        ))
        .id();

    commands.trigger(LightyearConnect { entity: client });
}

fn log_connection_status(trigger: On<Add, Link>) {
    tracing::info!("Connected to server: {:?}", trigger.entity);
}

// ============================================================================
// Main
// ============================================================================

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("roguebench_client=info,lightyear=warn")
        .init();

    tracing::info!("Starting client...");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Roguebench Client".into(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ClientPlugins {
            tick_duration: tick_duration(),
        })
        .add_plugins(ProtocolPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_client)
        .add_systems(Update, spawn_labels_for_entities)
        .add_systems(Update, update_entity_positions)
        .add_observer(log_connection_status)
        .run();

    Ok(())
}
