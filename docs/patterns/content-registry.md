# Pattern: Content Registry

A pattern for loading, caching, and hot-reloading authored content from SQLite into runtime-accessible registries.

---

## Problem

Authored content (items, enemies, rooms) lives in SQLite. The runtime needs to:

- Load content at startup
- Access content by ID during gameplay
- Reload content when the editor makes changes
- Do this generically for all content types

## Solution

A `ContentRegistry<T>` that:
1. Loads all entries of type T from SQLite
2. Stores them in a lookup table by ID
3. Exposes them as a Bevy resource
4. Reloads when notified of changes

## Implementation

### ContentType Trait

```rust
pub trait ContentType: Clone + Send + Sync + 'static {
    type Id: Hash + Eq + Clone;

    fn id(&self) -> &Self::Id;
    fn load_all(db: &Database) -> Result<Vec<Self>, LoadError>;
    fn type_name() -> &'static str;
}
```

### ContentRegistry

```rust
#[derive(Resource)]
pub struct ContentRegistry<T: ContentType> {
    entries: HashMap<T::Id, T>,
}

impl<T: ContentType> ContentRegistry<T> {
    pub fn load_from_db(db: &Database) -> Result<Self, LoadError> {
        let entries = T::load_all(db)?
            .into_iter()
            .map(|e| (e.id().clone(), e))
            .collect();
        Ok(Self { entries })
    }

    pub fn get(&self, id: &T::Id) -> Option<&T> {
        self.entries.get(id)
    }

    pub fn reload(&mut self, db: &Database) -> Result<(), LoadError> {
        *self = Self::load_from_db(db)?;
        Ok(())
    }
}
```

### Example: ItemDefinition

```rust
impl ContentType for ItemDefinition {
    type Id = ItemId;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn load_all(db: &Database) -> Result<Vec<Self>, LoadError> {
        db.get_all_items().map_err(LoadError::from)
    }

    fn type_name() -> &'static str {
        "item"
    }
}

// Usage
fn spawn_item(
    registry: Res<ContentRegistry<ItemDefinition>>,
    // ...
) {
    let item_def = registry.get(&ItemId::new("health_potion")).unwrap();
    // Spawn entity using item_def
}
```

### Hot Reload Integration

```rust
// ContentWatcher detects database changes
fn reload_on_change(
    mut registry: ResMut<ContentRegistry<ItemDefinition>>,
    db: Res<Database>,
    mut events: EventReader<ContentChanged<ItemDefinition>>,
) {
    for _ in events.read() {
        registry.reload(&db).unwrap();
    }
}
```

### App Extension

```rust
pub trait ContentAppExt {
    fn register_content_type<T: ContentType>(&mut self) -> &mut Self;
}

impl ContentAppExt for App {
    fn register_content_type<T: ContentType>(&mut self) -> &mut Self {
        self.add_event::<ContentChanged<T>>()
            .init_resource::<ContentRegistry<T>>()
            // Add reload system
    }
}

// Usage
app.register_content_type::<ItemDefinition>()
   .register_content_type::<EnemyDefinition>()
   .register_content_type::<RoomDefinition>();
```

## When to Use

- **Authored content** that lives in SQLite and is used at runtime
- **Content that should hot-reload** when changed in the editor
- **Lookup tables** where you access by ID

## When Not to Use

- **Runtime-only data** that isn't authored (player session state)
- **Singleton resources** (game settings, not a registry of many items)

## Benefits

| Benefit | How |
|---------|-----|
| Generic | One registry pattern for all content types |
| Hot reload | Reload method + change events |
| Type-safe | `ContentRegistry<ItemDefinition>` vs `ContentRegistry<EnemyDefinition>` |
| Testable | In-memory database for tests |

## Related Patterns

- **Repository Pattern:** Similar data access abstraction
- **Unit of Work:** ContentRegistry is a read-only view; writes go through Database
