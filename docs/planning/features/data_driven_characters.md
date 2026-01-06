# Data-Driven Characters

External configuration files defining character attributes, separating data from behavior logic.

## Core Logic

**Concept**

- Character properties in external files
- Same systems work with any character
- Edit without recompiling
- Support multiple character types

**Character Definition**

| Property | Type | Description |
|----------|------|-------------|
| name | string | Character identifier |
| sprite_sheet | path | Texture atlas path |
| frame_size | Vec2 | Sprite dimensions |
| speed_walk | f32 | Walking speed |
| speed_run | f32 | Running speed |
| health | f32 | Starting health |
| animations | map | Animation definitions |

**Animation Definition**

| Property | Type | Description |
|----------|------|-------------|
| type | enum | Walk, Run, Jump, Idle |
| row | int | Spritesheet row |
| frames | int | Frame count |
| fps | f32 | Animation speed |
| directions | list | Supported directions |

**Operations**

`load_character(path)` - Load definition

- Parse RON/JSON file
- Validate required fields
- Return character data

`spawn_character(definition)` - Create entity

- Spawn with loaded attributes
- Configure animation controller
- Apply stats

`switch_character(entity, definition)` - Swap character

- Replace character data
- Update sprite/animations
- Preserve position/state

**Invariants**

- All required fields present
- Animation references valid frames
- Sprite sheet exists
- Speeds are positive

**File Format**

```ron
Character(
    name: "knight",
    sprite_sheet: "sprites/knight.png",
    frame_size: (64, 64),
    speed_walk: 100.0,
    speed_run: 200.0,
    health: 100.0,
    animations: {
        Walk: AnimationClip(row: 1, frames: 6, fps: 10.0),
        Run: AnimationClip(row: 2, frames: 8, fps: 15.0),
        Idle: AnimationClip(row: 0, frames: 4, fps: 5.0),
    },
)
```

---

## Bevy Integration

**Assets**

- Custom asset type for character definitions
- AssetLoader implementation for .ron files

**Components**

- CharacterDefinition - loaded data
- CharacterStats - runtime stats from definition

**Resources**

- CharacterLibrary { characters: HashMap<String, Handle<CharacterDef>> }

**Loading Flow**

1. Request character asset load
2. Spawn placeholder entity
3. On asset ready, populate components
4. Character fully functional

**Systems**

- Load character definitions on startup
- Spawn characters from definitions
- Handle character switching
- Hot-reload on file change (dev)

**RON Benefits**

- Comments in config files
- Rust-native types (enums, tuples)
- More readable than JSON
- Strong typing

**Runtime Swapping**

```rust
fn switch_character(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CharacterDefinition, With<Player>>,
    library: Res<CharacterLibrary>,
) {
    if input.just_pressed(KeyCode::Digit1) {
        for mut def in &mut query {
            *def = library.get("knight").clone();
        }
    }
}
```
