//! roguebench-client: Game client with Bevy rendering
//!
//! Connects to the server via Lightyear and renders replicated entities.
//! The client has no direct database access - all game state comes from the server.

use bevy::{asset::AssetMetaCheck, prelude::*};
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use lightyear_udp::UdpIo;
use roguebench_protocol::{Enemy, Position, ProtocolPlugin, FIXED_TIMESTEP_HZ, SERVER_PORT};
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "roguebench".into(),
                        resolution: (800, 600).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "assets".to_string(),
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins(ClientPlugins {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        })
        .add_plugins(ProtocolPlugin)
        .add_systems(Startup, (setup_camera, connect_to_server))
        .add_systems(Update, render_enemies)
        .run();
}

/// Camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

/// Connect to the game server
fn connect_to_server(mut commands: Commands) {
    let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), SERVER_PORT);

    info!("Connecting to server at {}", server_addr);

    // Spawn the client connection entity
    commands.spawn((
        Client::default(),
        UdpIo::default(),
        // Local address: use any available port
        LocalAddr(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0)),
        // Server address to connect to
        PeerAddr(server_addr),
    ));
}

/// Component to mark that we've created a visual for this enemy
#[derive(Component)]
struct EnemyVisual;

/// Render enemies that have been replicated from the server
fn render_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemies: Query<(Entity, &Enemy, &Position), (With<Replicated>, Without<EnemyVisual>)>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for (entity, enemy, position) in enemies.iter() {
        info!("Rendering enemy: {} at {:?}", enemy.name, position.0);

        // Add visual components to the replicated enemy entity
        commands.entity(entity).insert((
            Sprite {
                color: Color::srgb(0.8, 0.2, 0.2),
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            Transform::from_xyz(position.0.x, position.0.y, 0.0),
            EnemyVisual,
        ));

        // Spawn text as children
        commands.entity(entity).with_children(|parent| {
            // Name label above
            parent.spawn((
                Text2d::new(&enemy.name),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Transform::from_xyz(0.0, 70.0, 1.0),
            ));

            // Health label below
            parent.spawn((
                Text2d::new(format!("HP: {}", enemy.health)),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Transform::from_xyz(0.0, -70.0, 1.0),
            ));
        });
    }
}
