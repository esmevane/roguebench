//! roguebench-client: Game client with Bevy rendering
//!
//! For the walking skeleton, the client reads directly from the database.
//! Later, this will connect to the server via Lightyear for networking.

use bevy::{asset::AssetMetaCheck, prelude::*};
use roguebench_core::{Database, EnemyDefinition, EnemyId};
use std::collections::HashMap;

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
        .init_resource::<EnemyRegistry>()
        .add_systems(Startup, (setup_camera, load_enemies, spawn_test_enemies).chain())
        .add_systems(Update, check_for_reload)
        .run();
}

/// Camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

/// Registry holding loaded enemy definitions
#[derive(Resource, Default)]
struct EnemyRegistry {
    enemies: HashMap<EnemyId, EnemyDefinition>,
    last_load: Option<std::time::Instant>,
}

impl EnemyRegistry {
    fn load_from_db(db: &Database) -> Self {
        let enemies = db
            .get_all_enemies()
            .unwrap_or_default()
            .into_iter()
            .map(|e| (e.id.clone(), e))
            .collect();

        Self {
            enemies,
            last_load: Some(std::time::Instant::now()),
        }
    }

    fn get(&self, id: &EnemyId) -> Option<&EnemyDefinition> {
        self.enemies.get(id)
    }

    fn iter(&self) -> impl Iterator<Item = &EnemyDefinition> {
        self.enemies.values()
    }
}

/// Component to mark spawned enemies
#[derive(Component)]
struct Enemy {
    definition_id: EnemyId,
}

/// Component for health display
#[derive(Component)]
struct HealthText;

/// Load enemies from database into registry
fn load_enemies(mut registry: ResMut<EnemyRegistry>) {
    let db_path = "roguebench.db";

    match Database::open(db_path) {
        Ok(db) => {
            *registry = EnemyRegistry::load_from_db(&db);
            info!("Loaded {} enemies from database", registry.enemies.len());
        }
        Err(e) => {
            warn!("Failed to open database: {}. Using empty registry.", e);
        }
    }
}

/// Spawn test enemies based on loaded definitions
fn spawn_test_enemies(
    mut commands: Commands,
    registry: Res<EnemyRegistry>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Spawn each enemy definition as a colored rectangle
    for (i, enemy) in registry.iter().enumerate() {
        let x = -200.0 + (i as f32 * 150.0);
        let y = 0.0;

        // Spawn the enemy rectangle
        commands
            .spawn((
                Sprite {
                    color: Color::srgb(0.8, 0.2, 0.2),
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
                Enemy {
                    definition_id: enemy.id.clone(),
                },
            ))
            .with_children(|parent| {
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
                    HealthText,
                ));
            });

        info!("Spawned enemy: {} at ({}, {})", enemy.name, x, y);
    }

    if registry.enemies.is_empty() {
        info!("No enemies to spawn. Create some in the editor at http://localhost:8080/enemies");
    }
}

/// Check for database changes and reload (simple polling for now)
fn check_for_reload(
    mut registry: ResMut<EnemyRegistry>,
    time: Res<Time>,
    mut last_check: Local<f32>,
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    // Check every 2 seconds
    *last_check += time.delta_secs();
    if *last_check < 2.0 {
        return;
    }
    *last_check = 0.0;

    let db_path = "roguebench.db";
    if let Ok(db) = Database::open(db_path) {
        let new_registry = EnemyRegistry::load_from_db(&db);

        // Check if anything changed
        if new_registry.enemies.len() != registry.enemies.len()
            || new_registry
                .enemies
                .iter()
                .any(|(id, def)| registry.get(id).map(|old| old.health != def.health || old.name != def.name).unwrap_or(true))
        {
            info!("Database changed, reloading enemies");

            // Despawn old enemies
            for entity in enemies.iter() {
                commands.entity(entity).despawn();
            }

            // Update registry
            *registry = new_registry;

            // Respawn enemies
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            for (i, enemy) in registry.iter().enumerate() {
                let x = -200.0 + (i as f32 * 150.0);
                let y = 0.0;

                commands
                    .spawn((
                        Sprite {
                            color: Color::srgb(0.8, 0.2, 0.2),
                            custom_size: Some(Vec2::new(100.0, 100.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, y, 0.0),
                        Enemy {
                            definition_id: enemy.id.clone(),
                        },
                    ))
                    .with_children(|parent| {
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

                        parent.spawn((
                            Text2d::new(format!("HP: {}", enemy.health)),
                            TextFont {
                                font: font.clone(),
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                            Transform::from_xyz(0.0, -70.0, 1.0),
                            HealthText,
                        ));
                    });
            }
        }
    }
}
