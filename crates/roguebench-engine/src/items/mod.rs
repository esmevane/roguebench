//! Item loading, registry, and runtime systems.

mod components;
mod registry;
pub mod spawn;

pub use components::{Item, ItemBundle, ItemPickedUp, ItemUsed, Pickup};
pub use registry::ItemRegistry;
pub use spawn::SpawnItem;

use bevy::prelude::*;
use crate::data::Database;

/// Plugin for item content loading and management.
pub struct ItemsPlugin {
    /// Path to the content database.
    pub db_path: String,
}

impl ItemsPlugin {
    pub fn new(db_path: impl Into<String>) -> Self {
        Self {
            db_path: db_path.into(),
        }
    }
}

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        // Open database and load items
        let db = Database::open(&self.db_path)
            .expect("Failed to open content database");

        let items = db.get_all_items()
            .expect("Failed to load items from database");

        let registry = ItemRegistry::from_items(items);
        let item_count = registry.len();

        app.insert_resource(registry)
            .add_event::<SpawnItem>()
            .add_event::<ItemPickedUp>()
            .add_event::<ItemUsed>()
            .add_event::<ReloadItems>()
            .add_systems(Update, (
                spawn::spawn_item_system,
                spawn::despawn_picked_up_items,
            ));

        info!("Loaded {} items from database", item_count);
    }
}

/// Event to trigger item reload.
#[derive(Event)]
pub struct ReloadItems;

/// System to reload items when requested.
pub fn reload_items_system(
    mut events: EventReader<ReloadItems>,
    mut registry: ResMut<ItemRegistry>,
) {
    for _ in events.read() {
        // TODO: Implement reload when hot-reload is added
        info!("Item reload requested (not yet implemented)");
        let _ = registry.as_mut(); // Suppress unused warning
    }
}
