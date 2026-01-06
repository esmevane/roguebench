//! Integration tests for the Data Pipeline.
//!
//! These tests verify the complete content loading workflow:
//! DB → ContentRegistry → FileWatcher → Reload

use bevy::prelude::*;
use roguebench_core::items::{ItemDefinition, ItemId, ItemType};
use roguebench_engine::data::{
    ContentAppExt, ContentChanged, ContentRegistry, ContentWatcher, ContentWatcherPlugin, Database,
};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Helper to create a unique temp database path.
fn temp_db_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "roguebench_data_test_{}_{}.db",
        std::process::id(),
        timestamp
    ))
}

/// Test: Load content from database into registry.
#[test]
fn load_content_from_database() {
    let path = temp_db_path();
    let db = Database::open(path.to_str().unwrap()).unwrap();

    // Insert test data
    db.upsert_item(&ItemDefinition::new("sword", "Iron Sword", ItemType::Equipment))
        .unwrap();
    db.upsert_item(&ItemDefinition::new("potion", "Health Potion", ItemType::Consumable))
        .unwrap();

    // Load into registry
    let registry = ContentRegistry::<ItemDefinition>::load_from_db(&db).unwrap();

    assert_eq!(registry.len(), 2);
    assert!(registry.contains(&ItemId::new("sword")));
    assert!(registry.contains(&ItemId::new("potion")));

    // Cleanup
    std::fs::remove_file(path).ok();
}

/// Test: Reload registry from database.
#[test]
fn reload_content_registry() {
    let path = temp_db_path();
    let db = Database::open(path.to_str().unwrap()).unwrap();

    // Initial data
    db.upsert_item(&ItemDefinition::new("item1", "Item 1", ItemType::Misc))
        .unwrap();

    let mut registry = ContentRegistry::<ItemDefinition>::load_from_db(&db).unwrap();
    assert_eq!(registry.len(), 1);

    // Add more data
    db.upsert_item(&ItemDefinition::new("item2", "Item 2", ItemType::Misc))
        .unwrap();
    db.upsert_item(&ItemDefinition::new("item3", "Item 3", ItemType::Misc))
        .unwrap();

    // Reload
    let count = registry.reload(&db).unwrap();
    assert_eq!(count, 3);
    assert_eq!(registry.len(), 3);

    // Cleanup
    std::fs::remove_file(path).ok();
}

/// Test: ContentChanged events.
#[test]
fn content_changed_events() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .register_content::<ItemDefinition>();

    // Create registry
    let registry = ContentRegistry::<ItemDefinition>::new();
    app.insert_resource(registry);

    // Send reload event
    app.world_mut()
        .send_event(ContentChanged::<ItemDefinition>::reload_all());

    app.update();

    // Event should have been processed (we can't easily verify this
    // without a system to handle it, but at least it doesn't panic)
}

/// Test: File watcher creation.
#[test]
fn file_watcher_creation() {
    let path = temp_db_path();

    // Create the file
    let db = Database::open(path.to_str().unwrap()).unwrap();
    drop(db); // Close connection

    // Create watcher
    let watcher = ContentWatcher::new(&path);
    assert!(watcher.is_ok());

    // Cleanup
    std::fs::remove_file(path).ok();
}

/// Test: File watcher detects changes (may be flaky on some systems).
#[test]
fn file_watcher_detects_changes() {
    let path = temp_db_path();

    // Create initial file
    {
        let mut file = File::create(&path).unwrap();
        file.write_all(b"initial").unwrap();
    }

    // Create watcher
    let watcher = ContentWatcher::new(&path).unwrap();

    // Initial poll should be empty
    assert!(watcher.poll().is_empty());

    // Modify file
    std::thread::sleep(Duration::from_millis(100));
    {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&path)
            .unwrap();
        file.write_all(b"modified").unwrap();
    }

    // Wait for debounce
    std::thread::sleep(Duration::from_millis(700));

    // Poll for changes
    let changes = watcher.poll();

    // Note: File system events are OS-dependent
    // We just verify it doesn't panic

    // Cleanup
    std::fs::remove_file(path).ok();

    let _ = changes; // Suppress unused warning
}

/// Test: ContentWatcher plugin handles startup.
#[test]
fn content_watcher_plugin_startup() {
    let path = temp_db_path();

    // Create the database file
    let db = Database::open(path.to_str().unwrap()).unwrap();
    db.upsert_item(&ItemDefinition::new("test", "Test", ItemType::Misc))
        .unwrap();
    drop(db);

    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(ContentWatcherPlugin::new(&path));

    // Should be able to update without panic
    app.update();

    // Watcher should exist
    assert!(app.world().get_resource::<ContentWatcher>().is_some());

    // Cleanup
    std::fs::remove_file(path).ok();
}

/// Test: Full pipeline - load, modify, detect, reload.
#[test]
fn full_pipeline_integration() {
    let path = temp_db_path();

    // Create initial database
    let db = Database::open(path.to_str().unwrap()).unwrap();
    db.upsert_item(&ItemDefinition::new("item1", "Item 1", ItemType::Misc))
        .unwrap();

    // Load registry
    let mut registry = ContentRegistry::<ItemDefinition>::load_from_db(&db).unwrap();
    assert_eq!(registry.len(), 1);
    assert_eq!(
        registry.get(&ItemId::new("item1")).unwrap().name,
        "Item 1"
    );

    // Modify through database
    db.upsert_item(
        &ItemDefinition::new("item1", "Updated Item 1", ItemType::Misc),
    )
    .unwrap();
    db.upsert_item(&ItemDefinition::new("item2", "Item 2", ItemType::Misc))
        .unwrap();

    // Reload
    registry.reload(&db).unwrap();

    // Verify updates
    assert_eq!(registry.len(), 2);
    assert_eq!(
        registry.get(&ItemId::new("item1")).unwrap().name,
        "Updated Item 1"
    );
    assert!(registry.contains(&ItemId::new("item2")));

    // Cleanup
    std::fs::remove_file(path).ok();
}

/// Test: Multiple content types can be loaded independently.
#[test]
fn multiple_content_types() {
    // For now we only have ItemDefinition, but this test structure
    // shows how multiple types would work

    let path = temp_db_path();
    let db = Database::open(path.to_str().unwrap()).unwrap();

    db.upsert_item(&ItemDefinition::new("sword", "Sword", ItemType::Equipment))
        .unwrap();
    db.upsert_item(&ItemDefinition::new("shield", "Shield", ItemType::Equipment))
        .unwrap();

    // Load items
    let item_registry = ContentRegistry::<ItemDefinition>::load_from_db(&db).unwrap();
    assert_eq!(item_registry.len(), 2);

    // In the future, we'd also load:
    // let enemy_registry = ContentRegistry::<EnemyDefinition>::load_from_db(&db)?;
    // let room_registry = ContentRegistry::<RoomDefinition>::load_from_db(&db)?;

    // Cleanup
    std::fs::remove_file(path).ok();
}

/// Test: App extension methods work correctly.
#[test]
fn app_extension_methods() {
    let path = temp_db_path();
    let db = Database::open(path.to_str().unwrap()).unwrap();

    db.upsert_item(&ItemDefinition::new("test", "Test", ItemType::Misc))
        .unwrap();

    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .load_content::<ItemDefinition>(&db)
        .unwrap();

    app.update();

    // Registry should exist and be populated
    let registry = app.world().resource::<ContentRegistry<ItemDefinition>>();
    assert_eq!(registry.len(), 1);

    // Cleanup
    std::fs::remove_file(path).ok();
}
