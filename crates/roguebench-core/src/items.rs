//! Item definitions and types.
//!
//! Items are the first vertical slice content type, validating the full
//! authoring pipeline from editor to runtime.

use serde::{Deserialize, Serialize};

/// Unique identifier for an item definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub String);

impl ItemId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Classification of item types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    /// Consumable items (potions, food, scrolls)
    Consumable,
    /// Equipment items (weapons, armor, accessories)
    Equipment,
    /// Key items (quest items, keys, special objects)
    Key,
    /// Currency or stackable resources
    Currency,
    /// Miscellaneous items
    Misc,
}

/// An effect that can be applied when an item is used.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Effect {
    /// Modify a stat by an amount
    ModifyStat { stat: String, amount: i32 },
    /// Grant temporary status effect
    ApplyStatus { status: String, duration_secs: f32 },
    /// Trigger a game event
    TriggerEvent { event: String },
}

/// Definition of an item template.
///
/// This is the data stored in SQLite and loaded at runtime.
/// Individual item instances in the game world reference this by ID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemDefinition {
    /// Unique identifier
    pub id: ItemId,
    /// Display name
    pub name: String,
    /// Item classification
    pub item_type: ItemType,
    /// Optional description text
    pub description: Option<String>,
    /// Effects when item is used
    pub effects: Vec<Effect>,
    /// Whether item stacks in inventory
    pub stackable: bool,
    /// Maximum stack size (if stackable)
    pub max_stack: Option<u32>,
    /// Base value for trading
    pub value: u32,
}

impl ItemDefinition {
    /// Create a new item definition with minimal required fields.
    pub fn new(id: impl Into<String>, name: impl Into<String>, item_type: ItemType) -> Self {
        Self {
            id: ItemId::new(id),
            name: name.into(),
            item_type,
            description: None,
            effects: Vec::new(),
            stackable: false,
            max_stack: None,
            value: 0,
        }
    }

    /// Builder method to set description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Builder method to add an effect.
    pub fn with_effect(mut self, effect: Effect) -> Self {
        self.effects.push(effect);
        self
    }

    /// Builder method to make item stackable.
    pub fn stackable(mut self, max_stack: u32) -> Self {
        self.stackable = true;
        self.max_stack = Some(max_stack);
        self
    }

    /// Builder method to set value.
    pub fn with_value(mut self, value: u32) -> Self {
        self.value = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_definition_builder() {
        let potion = ItemDefinition::new("health_potion", "Health Potion", ItemType::Consumable)
            .with_description("Restores 50 health")
            .with_effect(Effect::ModifyStat {
                stat: "health".into(),
                amount: 50,
            })
            .stackable(99)
            .with_value(25);

        assert_eq!(potion.id.0, "health_potion");
        assert_eq!(potion.name, "Health Potion");
        assert_eq!(potion.item_type, ItemType::Consumable);
        assert!(potion.stackable);
        assert_eq!(potion.max_stack, Some(99));
        assert_eq!(potion.effects.len(), 1);
    }

    #[test]
    fn item_serialization_roundtrip() {
        let item = ItemDefinition::new("sword", "Iron Sword", ItemType::Equipment)
            .with_description("A sturdy iron sword")
            .with_value(100);

        let json = serde_json::to_string(&item).unwrap();
        let parsed: ItemDefinition = serde_json::from_str(&json).unwrap();

        assert_eq!(item, parsed);
    }
}
