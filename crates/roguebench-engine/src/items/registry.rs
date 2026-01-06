//! Item registry - runtime storage for item definitions.

use bevy::prelude::*;
use roguebench_core::items::{ItemDefinition, ItemId, ItemType};
use std::collections::HashMap;

/// Runtime registry of all loaded item definitions.
#[derive(Resource, Default)]
pub struct ItemRegistry {
    items: HashMap<ItemId, ItemDefinition>,
}

impl ItemRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a registry from a list of items.
    pub fn from_items(items: Vec<ItemDefinition>) -> Self {
        let mut registry = Self::new();
        for item in items {
            registry.insert(item);
        }
        registry
    }

    /// Insert an item into the registry.
    pub fn insert(&mut self, item: ItemDefinition) {
        self.items.insert(item.id.clone(), item);
    }

    /// Get an item by ID.
    pub fn get(&self, id: &ItemId) -> Option<&ItemDefinition> {
        self.items.get(id)
    }

    /// Get an item by string ID.
    pub fn get_by_str(&self, id: &str) -> Option<&ItemDefinition> {
        self.items.get(&ItemId::new(id))
    }

    /// Check if an item exists.
    pub fn contains(&self, id: &ItemId) -> bool {
        self.items.contains_key(id)
    }

    /// Get all items.
    pub fn all(&self) -> impl Iterator<Item = &ItemDefinition> {
        self.items.values()
    }

    /// Get all items of a specific type.
    pub fn by_type(&self, item_type: ItemType) -> impl Iterator<Item = &ItemDefinition> {
        self.items.values().filter(move |item| item.item_type == item_type)
    }

    /// Get the number of items in the registry.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear all items from the registry.
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roguebench_core::items::Effect;

    #[test]
    fn registry_crud() {
        let mut registry = ItemRegistry::new();
        assert!(registry.is_empty());

        let sword = ItemDefinition::new("sword", "Iron Sword", ItemType::Equipment);
        registry.insert(sword);

        assert_eq!(registry.len(), 1);
        assert!(registry.contains(&ItemId::new("sword")));

        let loaded = registry.get_by_str("sword").unwrap();
        assert_eq!(loaded.name, "Iron Sword");
    }

    #[test]
    fn registry_from_items() {
        let items = vec![
            ItemDefinition::new("sword", "Sword", ItemType::Equipment),
            ItemDefinition::new("potion", "Potion", ItemType::Consumable)
                .with_effect(Effect::ModifyStat { stat: "health".into(), amount: 50 }),
            ItemDefinition::new("key", "Key", ItemType::Key),
        ];

        let registry = ItemRegistry::from_items(items);
        assert_eq!(registry.len(), 3);
    }

    #[test]
    fn filter_by_type() {
        let items = vec![
            ItemDefinition::new("sword", "Sword", ItemType::Equipment),
            ItemDefinition::new("shield", "Shield", ItemType::Equipment),
            ItemDefinition::new("potion", "Potion", ItemType::Consumable),
        ];

        let registry = ItemRegistry::from_items(items);

        let equipment: Vec<_> = registry.by_type(ItemType::Equipment).collect();
        assert_eq!(equipment.len(), 2);

        let consumables: Vec<_> = registry.by_type(ItemType::Consumable).collect();
        assert_eq!(consumables.len(), 1);
    }
}
