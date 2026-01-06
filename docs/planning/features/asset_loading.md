# Asset Loading

Asynchronous asset loading with loading screen and placeholder handling.

## Core Logic

**Concept**

- Assets load asynchronously
- Show loading progress
- Spawn placeholders immediately
- Populate when ready

**Loading Phases**

| Phase | Description |
|-------|-------------|
| Queue | Request asset load |
| Loading | Asset being loaded |
| Processing | Post-load setup |
| Ready | Asset available |
| Failed | Load error |

**Asset Types**

| Type | Examples | Priority |
|------|----------|----------|
| Critical | Core sprites, fonts | Block gameplay |
| Gameplay | Characters, enemies | Show loading |
| Optional | Music, ambient | Background |

**Operations**

`load_asset(path)` - Request load

- Return handle immediately
- Load happens async

`is_loaded(handle)` - Check status

- Return true when ready
- Used for loading screen

`get_progress()` - Loading percentage

- Count loaded / total
- For progress bar

`on_load(handle, callback)` - Handle ready

- Execute when asset loaded
- Populate components

**Placeholder Pattern**

1. Spawn entity with placeholder sprite
2. Store asset handle in component
3. When asset loads, swap sprite
4. Entity functional throughout

**Invariants**

- Handles valid even before load
- Failed loads don't crash
- Progress accurate
- Critical assets block transition

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Timeout | 30s | Max load time |
| Retry count | 3 | Failed load retries |

---

## Bevy Integration

**Resources**

- LoadingState { assets: Vec<HandleUntyped>, loaded: usize }
- AssetProgress { current: f32, target: f32 }

**States**

```rust
enum GameState {
    Loading,
    Playing,
    Paused,
}
```

**Loading System**

```rust
fn check_loading(
    mut state: ResMut<NextState<GameState>>,
    loading: Res<LoadingState>,
    asset_server: Res<AssetServer>,
) {
    let all_loaded = loading.assets.iter().all(|h| {
        asset_server.is_loaded_with_dependencies(h)
    });

    if all_loaded {
        state.set(GameState::Playing);
    }
}
```

**Loading Screen**

- Display progress bar
- Show loading tips/art
- Animate to feel responsive
- Transition when complete

**Asset Collections**

```rust
#[derive(Resource)]
struct GameAssets {
    player_sprite: Handle<Image>,
    enemy_sprites: Handle<Image>,
    tile_atlas: Handle<Image>,
    // ...
}
```

**bevy_asset_loader Integration**

```rust
#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "sprites/player.png")]
    player: Handle<Image>,

    #[asset(path = "audio", collection(typed))]
    sounds: Vec<Handle<AudioSource>>,
}
```

**Error Handling**

- Log failed loads
- Use fallback assets
- Notify player of missing content
- Continue with available assets

**Hot Reloading (Dev)**

- Watch for file changes
- Reload modified assets
- Update entities using asset
- Development workflow improvement
