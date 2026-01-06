# Data Loading Framework

Content loading, hot reload, and asset management.

## Core Logic

**Concept**

- Game content defined in data files (RON, JSON)
- Data loaded at startup and on-demand
- Hot reload for development iteration
- Typed data definitions with validation

**Data Categories**

| Category | Examples |
|----------|----------|
| Definitions | Items, enemies, effects, stats |
| Content | Dialogue, quests, tutorials |
| Configuration | Settings, balance, tuning |
| Assets | Sprites, sounds, fonts |

**Loading States**

| State | Description |
|-------|-------------|
| Unloaded | Not yet requested |
| Loading | In progress |
| Loaded | Available for use |
| Failed | Error during load |
| Reloading | Hot reload in progress |

**Operations**

`load<T>(path)` - Load typed data

- Parse file
- Validate schema
- Store in registry

`get<T>(id)` - Retrieve loaded data

- Return reference to data
- Panic if not loaded

`try_get<T>(id)` - Safe retrieval

- Return Option
- None if not loaded

`reload(path)` - Hot reload file

- Re-parse file
- Validate
- Replace in registry
- Emit reload event

`validate<T>(data)` - Schema validation

- Check required fields
- Validate references
- Return errors

**Invariants**

- Data immutable after load
- Hot reload atomic (all or nothing)
- Invalid data rejected with clear errors
- References validated at load time

**Design Notes**

- File format left to implementation (RON recommended)
- Validation rules per data type
- Hot reload optional (dev builds)

---

## Bevy Integration

**Resources**

- DataRegistry<T> { items: HashMap<String, T> }
- LoadingState { pending: HashSet<String>, failed: HashMap<String, Error> }

**Asset Integration**

```rust
#[derive(Asset, TypePath, Deserialize)]
struct ItemDefinition {
    id: String,
    name: LocalizedKey,
    stats: Vec<StatModifier>,
    // ...
}

impl DataAsset for ItemDefinition {
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate references exist
    }
}
```

**Loading Pattern**

```rust
fn load_definitions(
    asset_server: Res<AssetServer>,
    mut registry: ResMut<DataRegistry<ItemDefinition>>,
) {
    // Load all .item.ron files
    // Validate and register
}
```

**Hot Reload**

```rust
fn hot_reload_system(
    mut asset_events: EventReader<AssetEvent<ItemDefinition>>,
    mut registry: ResMut<DataRegistry<ItemDefinition>>,
) {
    for event in asset_events.read() {
        if let AssetEvent::Modified { id } = event {
            // Reload and re-register
        }
    }
}
```

**Events**

- DataLoaded { type_name, id }
- DataReloaded { type_name, id }
- DataLoadFailed { type_name, path, error }

**Scripting Integration**

- Data readable from scripts
- Hot reload triggers script refresh
- Data IDs used in script commands

*See: architecture/data.md, architecture/scripting.md*
