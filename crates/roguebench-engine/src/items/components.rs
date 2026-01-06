//! Item-related Bevy components.

use bevy::prelude::*;
use roguebench_core::items::ItemId;

/// Marker component for entities that are items in the world.
#[derive(Component, Debug)]
pub struct Item {
    /// Reference to the item definition.
    pub definition_id: ItemId,
}

impl Item {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            definition_id: ItemId::new(id),
        }
    }
}

/// Component for items that can be picked up.
#[derive(Component, Debug, Default)]
pub struct Pickup {
    /// Whether the item has been picked up.
    pub picked_up: bool,
}

/// Event fired when an item is picked up.
#[derive(Event, Debug)]
pub struct ItemPickedUp {
    /// The item entity that was picked up.
    pub item_entity: Entity,
    /// The entity that picked it up.
    pub picker_entity: Entity,
    /// The item definition ID.
    pub item_id: ItemId,
}

/// Event fired when an item is used.
#[derive(Event, Debug)]
pub struct ItemUsed {
    /// The item definition ID.
    pub item_id: ItemId,
    /// The entity using the item.
    pub user_entity: Entity,
}

/// Bundle for spawning a world item.
#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub pickup: Pickup,
    pub transform: Transform,
}

impl ItemBundle {
    pub fn new(definition_id: impl Into<String>, position: Vec3) -> Self {
        Self {
            item: Item::new(definition_id),
            pickup: Pickup::default(),
            transform: Transform::from_translation(position),
        }
    }
}
