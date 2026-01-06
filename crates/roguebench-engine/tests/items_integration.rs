//! Integration tests for the Items vertical slice.
//!
//! These tests verify the complete authoring workflow:
//! DB -> Registry -> Spawn -> Entities

use bevy::prelude::*;
use roguebench_core::items::{Effect, ItemDefinition, ItemType};
use roguebench_engine::data::Database;
use roguebench_engine::items::{Item, ItemRegistry, Pickup, SpawnItem};

/// Helper to create a test database with sample items.
fn create_test_db() -> (Database, String) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir()
        .join(format!("roguebench_test_{}_{}.db", std::process::id(), timestamp))
        .to_string_lossy()
        .to_string();
    // Remove any existing file
    std::fs::remove_file(&path).ok();
    let db = Database::open(&path).expect("Failed to create test database");

    // Insert sample items
    let potion = ItemDefinition::new("health_potion", "Health Potion", ItemType::Consumable)
        .with_description("Restores health")
        .with_effect(Effect::ModifyStat {
            stat: "health".into(),
            amount: 50,
        })
        .stackable(99)
        .with_value(25);

    let sword = ItemDefinition::new("iron_sword", "Iron Sword", ItemType::Equipment)
        .with_description("A sturdy iron sword")
        .with_value(100);

    db.upsert_item(&potion).expect("Failed to insert potion");
    db.upsert_item(&sword).expect("Failed to insert sword");

    (db, path)
}

/// Test: Items can be loaded from database into registry.
#[test]
fn load_items_from_db_to_registry() {
    let (db, path) = create_test_db();

    // Load items from database
    let items = db.get_all_items().expect("Failed to load items");
    let registry = ItemRegistry::from_items(items);

    // Verify items are in registry
    assert_eq!(registry.len(), 2);
    assert!(registry.get_by_str("health_potion").is_some());
    assert!(registry.get_by_str("iron_sword").is_some());

    // Verify item properties
    let potion = registry.get_by_str("health_potion").unwrap();
    assert_eq!(potion.name, "Health Potion");
    assert_eq!(potion.item_type, ItemType::Consumable);
    assert!(potion.stackable);
    assert_eq!(potion.effects.len(), 1);

    // Cleanup
    std::fs::remove_file(path).ok();
}

/// Test: SpawnItem event creates entity with correct components.
#[test]
fn spawn_item_creates_entity() {
    // Create a minimal Bevy app
    let mut app = App::new();

    // Add minimal plugins (no rendering)
    app.add_plugins(MinimalPlugins);

    // Set up registry with test item
    let items = vec![
        ItemDefinition::new("test_item", "Test Item", ItemType::Misc),
    ];
    let registry = ItemRegistry::from_items(items);
    app.insert_resource(registry);

    // Add item systems
    app.add_event::<SpawnItem>();
    app.add_systems(Update, roguebench_engine::items::spawn::spawn_item_system);

    // Send spawn event
    app.world_mut().send_event(SpawnItem::new("test_item", Vec3::new(10.0, 20.0, 0.0)));

    // Run one update
    app.update();

    // Verify entity was spawned with correct components
    let mut query = app.world_mut().query::<(&Item, &Pickup, &Transform)>();
    let results: Vec<_> = query.iter(app.world()).collect();

    assert_eq!(results.len(), 1, "Expected exactly one spawned item entity");

    let (item, pickup, transform) = results[0];
    assert_eq!(item.definition_id.0, "test_item");
    assert!(!pickup.picked_up);
    assert_eq!(transform.translation, Vec3::new(10.0, 20.0, 0.0));
}

/// Test: Multiple items can be spawned independently.
#[test]
fn spawn_multiple_items() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    let items = vec![
        ItemDefinition::new("item_a", "Item A", ItemType::Consumable),
        ItemDefinition::new("item_b", "Item B", ItemType::Equipment),
    ];
    let registry = ItemRegistry::from_items(items);
    app.insert_resource(registry);

    app.add_event::<SpawnItem>();
    app.add_systems(Update, roguebench_engine::items::spawn::spawn_item_system);

    // Spawn multiple items
    app.world_mut().send_event(SpawnItem::new("item_a", Vec3::ZERO));
    app.world_mut().send_event(SpawnItem::new("item_b", Vec3::new(5.0, 0.0, 0.0)));
    app.world_mut().send_event(SpawnItem::new("item_a", Vec3::new(10.0, 0.0, 0.0)));

    app.update();

    let mut query = app.world_mut().query::<&Item>();
    let count = query.iter(app.world()).count();
    assert_eq!(count, 3, "Expected 3 spawned items");
}

/// Test: Registry filtering by type works correctly.
#[test]
fn filter_items_by_type() {
    let items = vec![
        ItemDefinition::new("potion1", "Potion 1", ItemType::Consumable),
        ItemDefinition::new("potion2", "Potion 2", ItemType::Consumable),
        ItemDefinition::new("sword", "Sword", ItemType::Equipment),
        ItemDefinition::new("key", "Key", ItemType::Key),
    ];
    let registry = ItemRegistry::from_items(items);

    let consumables: Vec<_> = registry.by_type(ItemType::Consumable).collect();
    assert_eq!(consumables.len(), 2);

    let equipment: Vec<_> = registry.by_type(ItemType::Equipment).collect();
    assert_eq!(equipment.len(), 1);

    let keys: Vec<_> = registry.by_type(ItemType::Key).collect();
    assert_eq!(keys.len(), 1);
}

/// Test: Item form validation catches errors.
#[test]
fn item_form_validation() {
    use roguebench_engine::editor::item_editor::ItemForm;

    // Empty ID should fail
    let mut form = ItemForm::default();
    let errors = form.validate();
    assert!(errors.iter().any(|e: &String| e.contains("ID")));
    assert!(errors.iter().any(|e: &String| e.contains("Name")));

    // Valid form should pass
    form.id = "valid_item".to_string();
    form.name = "Valid Item".to_string();
    let errors = form.validate();
    assert!(errors.is_empty());

    // Invalid ID characters should fail
    form.id = "invalid-id!".to_string();
    let errors = form.validate();
    assert!(errors.iter().any(|e: &String| e.contains("alphanumeric")));
}

/// Test: Full CRUD cycle through database.
#[test]
fn database_crud_cycle() {
    let (db, path) = create_test_db();

    // Create
    let new_item = ItemDefinition::new("new_item", "New Item", ItemType::Currency)
        .with_value(500);
    db.upsert_item(&new_item).expect("Failed to create item");

    // Read
    let loaded = db.get_item(&roguebench_core::items::ItemId::new("new_item"))
        .expect("Failed to load item");
    assert_eq!(loaded.name, "New Item");
    assert_eq!(loaded.value, 500);

    // Update
    let updated = ItemDefinition::new("new_item", "Updated Item", ItemType::Currency)
        .with_value(1000);
    db.upsert_item(&updated).expect("Failed to update item");

    let reloaded = db.get_item(&roguebench_core::items::ItemId::new("new_item"))
        .expect("Failed to reload item");
    assert_eq!(reloaded.name, "Updated Item");
    assert_eq!(reloaded.value, 1000);

    // Delete
    let deleted = db.delete_item(&roguebench_core::items::ItemId::new("new_item"))
        .expect("Failed to delete item");
    assert!(deleted);

    // Verify deleted
    let result = db.get_item(&roguebench_core::items::ItemId::new("new_item"));
    assert!(result.is_err());

    // Cleanup
    std::fs::remove_file(path).ok();
}
