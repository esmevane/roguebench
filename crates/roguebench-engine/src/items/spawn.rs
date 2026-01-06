//! Item spawning systems.

use bevy::prelude::*;
use roguebench_core::items::ItemId;
use super::components::{ItemBundle, Pickup};
use super::registry::ItemRegistry;

/// Command to spawn an item in the world.
#[derive(Event)]
pub struct SpawnItem {
    /// The item definition ID to spawn.
    pub item_id: ItemId,
    /// World position to spawn at.
    pub position: Vec3,
}

impl SpawnItem {
    pub fn new(id: impl Into<String>, position: Vec3) -> Self {
        Self {
            item_id: ItemId::new(id),
            position,
        }
    }
}

/// System that processes SpawnItem events and creates item entities.
pub fn spawn_item_system(
    mut commands: Commands,
    mut events: EventReader<SpawnItem>,
    registry: Res<ItemRegistry>,
) {
    for event in events.read() {
        // Verify the item exists in the registry
        if registry.get(&event.item_id).is_none() {
            warn!("Attempted to spawn unknown item: {}", event.item_id);
            continue;
        }

        commands.spawn(ItemBundle::new(event.item_id.0.clone(), event.position));

        debug!("Spawned item {} at {:?}", event.item_id, event.position);
    }
}

/// System that handles item despawning when picked up.
pub fn despawn_picked_up_items(
    mut commands: Commands,
    query: Query<(Entity, &Pickup), Changed<Pickup>>,
) {
    for (entity, pickup) in &query {
        if pickup.picked_up {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_item_event() {
        let event = SpawnItem::new("health_potion", Vec3::new(10.0, 20.0, 0.0));
        assert_eq!(event.item_id.0, "health_potion");
        assert_eq!(event.position, Vec3::new(10.0, 20.0, 0.0));
    }
}
