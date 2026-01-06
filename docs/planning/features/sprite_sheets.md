# Sprite Sheets

Organization and usage of sprite atlases for characters, enemies, and effects.

## Core Logic

**Atlas Types**

| Atlas | Content | Size |
|-------|---------|------|
| Characters | Player, NPCs | 512x512 |
| Enemies | All enemy types | 1024x512 |
| Effects | Particles, impacts | 256x256 |
| Tiles | Tileset | 512x512 |
| UI | Icons, frames | 512x256 |

**Sprite Organization**

- Grid-based: uniform cell size
- Packed: variable sizes, atlas coordinates
- Rows per animation, columns per frame

**Animation Layout**

```
Row 0: Idle (4 frames)
Row 1: Walk (6 frames)
Row 2: Attack (5 frames)
Row 3: Hurt (2 frames)
Row 4: Death (6 frames)
```

**Frame Definition**

| Property | Type | Description |
|----------|------|-------------|
| index | u32 | Position in atlas |
| size | Vec2 | Frame dimensions |
| offset | Vec2 | Pivot offset |
| duration | f32 | Frame duration (ms) |

**Operations**

`get_frame(animation, frame_index)` - Get atlas rect

- Return UV coordinates for frame
- Handle animation wrapping

`get_animation(entity_type, state)` - Get animation data

- Return frame list for state
- Include timing information

**Invariants**

- All sprites power-of-two atlas
- Consistent padding between sprites
- Pivot points defined per sprite
- No overlapping regions

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Cell size | 32x32 | Standard sprite size |
| Padding | 1px | Between sprites |
| Format | PNG | With transparency |

---

## Bevy Integration

**Assets**

- TextureAtlasLayout - defines grid/regions
- Handle<Image> - the texture

**Loading**

```rust
let texture = asset_server.load("sprites/characters.png");
let layout = TextureAtlasLayout::from_grid(
    UVec2::new(32, 32),
    columns,
    rows,
    Some(UVec2::new(1, 1)), // padding
    None,
);
```

**Components**

- Sprite with TextureAtlas
- AnimationIndices { first, last }
- AnimationTimer

**Resources**

- SpriteSheets { characters, enemies, effects, ... }
- AnimationLibrary { animations: HashMap }

**Aseprite Integration**

- Export from Aseprite with JSON
- Parse animation data
- Generate TextureAtlasLayout

**Systems**

- Animate sprites based on timer
- Switch animation on state change
- Handle animation events (end, loop)
