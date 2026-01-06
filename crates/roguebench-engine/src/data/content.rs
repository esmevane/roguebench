//! Generic content loading infrastructure.
//!
//! Provides traits and types for loading typed content from SQLite.

use bevy::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use super::Database;

/// Error type for content loading operations.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("Database error: {0}")]
    Database(#[from] super::schema::DatabaseError),

    #[error("Content not found: {0}")]
    NotFound(String),

    #[error("Invalid content: {0}")]
    Invalid(String),
}

/// Trait for content types that can be loaded from the database.
///
/// Implement this for each content type (items, enemies, etc.) to enable
/// generic loading and registry management.
///
/// # Example
///
/// ```ignore
/// impl ContentType for ItemDefinition {
///     type Id = ItemId;
///
///     fn id(&self) -> &Self::Id {
///         &self.id
///     }
///
///     fn load_all(db: &Database) -> Result<Vec<Self>, LoadError> {
///         db.get_all_items().map_err(LoadError::from)
///     }
/// }
/// ```
pub trait ContentType: Clone + Send + Sync + Debug + 'static {
    /// The ID type for this content.
    type Id: Clone + Eq + Hash + Debug + Send + Sync + 'static;

    /// Get the ID of this content instance.
    fn id(&self) -> &Self::Id;

    /// Load all instances of this content type from the database.
    fn load_all(db: &Database) -> Result<Vec<Self>, LoadError>;

    /// The name of this content type (for logging).
    fn type_name() -> &'static str;
}

/// Generic registry for any content type.
///
/// This is a type-safe container for loaded content, providing
/// lookup by ID and iteration.
#[derive(Resource)]
pub struct ContentRegistry<C: ContentType> {
    items: HashMap<C::Id, C>,
}

impl<C: ContentType> Default for ContentRegistry<C> {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl<C: ContentType> ContentRegistry<C> {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a registry from a list of content items.
    pub fn from_items(items: Vec<C>) -> Self {
        let mut registry = Self::new();
        for item in items {
            registry.insert(item);
        }
        registry
    }

    /// Load all content from the database.
    pub fn load_from_db(db: &Database) -> Result<Self, LoadError> {
        let items = C::load_all(db)?;
        Ok(Self::from_items(items))
    }

    /// Insert or update content in the registry.
    pub fn insert(&mut self, item: C) {
        self.items.insert(item.id().clone(), item);
    }

    /// Remove content by ID.
    pub fn remove(&mut self, id: &C::Id) -> Option<C> {
        self.items.remove(id)
    }

    /// Get content by ID.
    pub fn get(&self, id: &C::Id) -> Option<&C> {
        self.items.get(id)
    }

    /// Check if content exists.
    pub fn contains(&self, id: &C::Id) -> bool {
        self.items.contains_key(id)
    }

    /// Get all content.
    pub fn all(&self) -> impl Iterator<Item = &C> {
        self.items.values()
    }

    /// Get all content IDs.
    pub fn ids(&self) -> impl Iterator<Item = &C::Id> {
        self.items.keys()
    }

    /// Get the number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear all content.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Reload all content from the database.
    ///
    /// This clears the registry and reloads everything.
    pub fn reload(&mut self, db: &Database) -> Result<usize, LoadError> {
        self.clear();
        let items = C::load_all(db)?;
        let count = items.len();
        for item in items {
            self.insert(item);
        }
        Ok(count)
    }

    /// Reload a single item from the database.
    ///
    /// Returns true if the item was found and reloaded.
    pub fn reload_one(&mut self, db: &Database, id: &C::Id) -> Result<bool, LoadError> {
        let all = C::load_all(db)?;
        for item in all {
            if item.id() == id {
                self.insert(item);
                return Ok(true);
            }
        }
        Ok(false)
    }
}

/// Event emitted when content changes.
#[derive(Event, Debug, Clone)]
pub struct ContentChanged<C: ContentType> {
    /// The type of change.
    pub change_type: ChangeType,
    /// The ID of the changed content (if applicable).
    pub id: Option<C::Id>,
    _marker: std::marker::PhantomData<C>,
}

impl<C: ContentType> ContentChanged<C> {
    /// Create an event for a full reload.
    pub fn reload_all() -> Self {
        Self {
            change_type: ChangeType::ReloadAll,
            id: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// Create an event for a specific item change.
    pub fn item_changed(id: C::Id) -> Self {
        Self {
            change_type: ChangeType::ItemChanged,
            id: Some(id),
            _marker: std::marker::PhantomData,
        }
    }

    /// Create an event for an item addition.
    pub fn item_added(id: C::Id) -> Self {
        Self {
            change_type: ChangeType::ItemAdded,
            id: Some(id),
            _marker: std::marker::PhantomData,
        }
    }

    /// Create an event for an item removal.
    pub fn item_removed(id: C::Id) -> Self {
        Self {
            change_type: ChangeType::ItemRemoved,
            id: Some(id),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Type of content change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeType {
    /// Full reload of all content.
    ReloadAll,
    /// A specific item was changed.
    ItemChanged,
    /// A new item was added.
    ItemAdded,
    /// An item was removed.
    ItemRemoved,
}

/// Extension trait for registering content types.
pub trait ContentAppExt {
    /// Register a content type's registry and events.
    fn register_content<C: ContentType>(&mut self) -> &mut Self;

    /// Register and load content from a database path.
    fn load_content<C: ContentType>(&mut self, db: &Database) -> Result<&mut Self, LoadError>;
}

impl ContentAppExt for App {
    fn register_content<C: ContentType>(&mut self) -> &mut Self {
        self.init_resource::<ContentRegistry<C>>()
            .add_event::<ContentChanged<C>>();
        self
    }

    fn load_content<C: ContentType>(&mut self, db: &Database) -> Result<&mut Self, LoadError> {
        let registry = ContentRegistry::<C>::load_from_db(db)?;
        let count = registry.len();
        self.insert_resource(registry)
            .add_event::<ContentChanged<C>>();
        info!("Loaded {} {} from database", count, C::type_name());
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roguebench_core::items::{ItemDefinition, ItemId, ItemType};

    // ContentType impl for ItemDefinition is in impls.rs

    #[test]
    fn registry_crud() {
        let mut registry = ContentRegistry::<ItemDefinition>::new();
        assert!(registry.is_empty());

        let sword = ItemDefinition::new("sword", "Iron Sword", ItemType::Equipment);
        registry.insert(sword);

        assert_eq!(registry.len(), 1);
        assert!(registry.contains(&ItemId::new("sword")));

        let loaded = registry.get(&ItemId::new("sword")).unwrap();
        assert_eq!(loaded.name, "Iron Sword");
    }

    #[test]
    fn registry_from_items() {
        let items = vec![
            ItemDefinition::new("a", "A", ItemType::Misc),
            ItemDefinition::new("b", "B", ItemType::Misc),
            ItemDefinition::new("c", "C", ItemType::Misc),
        ];

        let registry = ContentRegistry::<ItemDefinition>::from_items(items);
        assert_eq!(registry.len(), 3);
    }

    #[test]
    fn registry_remove() {
        let mut registry = ContentRegistry::<ItemDefinition>::new();
        registry.insert(ItemDefinition::new("item", "Item", ItemType::Misc));

        assert!(registry.contains(&ItemId::new("item")));
        let removed = registry.remove(&ItemId::new("item"));
        assert!(removed.is_some());
        assert!(!registry.contains(&ItemId::new("item")));
    }

    #[test]
    fn load_from_database() {
        let db = Database::in_memory().unwrap();

        // Insert test items
        let item = ItemDefinition::new("test", "Test", ItemType::Misc);
        db.upsert_item(&item).unwrap();

        // Load into registry
        let registry = ContentRegistry::<ItemDefinition>::load_from_db(&db).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.contains(&ItemId::new("test")));
    }

    #[test]
    fn reload_registry() {
        let db = Database::in_memory().unwrap();
        db.upsert_item(&ItemDefinition::new("a", "A", ItemType::Misc))
            .unwrap();

        let mut registry = ContentRegistry::<ItemDefinition>::load_from_db(&db).unwrap();
        assert_eq!(registry.len(), 1);

        // Add another item to DB
        db.upsert_item(&ItemDefinition::new("b", "B", ItemType::Misc))
            .unwrap();

        // Reload
        let count = registry.reload(&db).unwrap();
        assert_eq!(count, 2);
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn content_changed_events() {
        let event: ContentChanged<ItemDefinition> = ContentChanged::reload_all();
        assert_eq!(event.change_type, ChangeType::ReloadAll);
        assert!(event.id.is_none());

        let event: ContentChanged<ItemDefinition> =
            ContentChanged::item_changed(ItemId::new("test"));
        assert_eq!(event.change_type, ChangeType::ItemChanged);
        assert_eq!(event.id, Some(ItemId::new("test")));
    }
}
