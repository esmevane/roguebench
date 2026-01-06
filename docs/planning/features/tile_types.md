# Tile Types

Definitions for different tile varieties and their properties.

## Core Logic

**Categories**

| Category | Collision | Examples |
|----------|-----------|----------|
| Floor | None | Stone, dirt, grass |
| Wall | Solid | Brick, stone wall |
| Hazard | Trigger | Spikes, lava, poison |
| Decoration | None | Rubble, bones, plants |
| Interactive | Trigger | Pressure plate, door |

**Tile Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | u32 | Unique identifier |
| name | string | Human-readable name |
| category | enum | Floor, Wall, Hazard, etc. |
| sprite_index | u32 | Index in tileset |
| collision | enum | None, Solid, Trigger |
| variants | list | Alternative sprites |
| animation | option | Animated tile frames |

**Floor Tiles**

| Tile | Variants | Notes |
|------|----------|-------|
| Stone | 4 | Basic dungeon floor |
| Dirt | 3 | Cave/outdoor |
| Wood | 2 | Indoor/ship |
| Grass | 4 | Outdoor |

**Wall Tiles**

| Tile | Autotile | Notes |
|------|----------|-------|
| Brick | Yes | 47-tile autotile |
| Stone | Yes | Dungeon walls |
| Wood | Yes | Building walls |

**Hazard Tiles**

| Tile | Damage | Effect |
|------|--------|--------|
| Spikes | 10 | Instant damage |
| Lava | 5/s | Continuous damage |
| Poison | 2/s | DOT + slow |
| Pit | Instant | Fall death |

**Operations**

`get_tile_def(id)` - Get tile definition

- Return properties for tile ID
- Used for collision, rendering

`is_walkable(id)` - Check traversability

- True for Floor, Decoration
- False for Wall
- Conditional for Hazard

**Invariants**

- Every placed tile has valid definition
- Collision matches category
- Variants visually similar
- Autotile rules consistent

---

## Bevy Integration

**Resources**

- TileDefinitions { tiles: HashMap<TileId, TileDef> }

**Data Loading**

- Load from RON/JSON file
- Or define in code
- Validate on load

**Tile Definition**

```rust
struct TileDef {
    name: String,
    category: TileCategory,
    sprite_index: u32,
    collision: CollisionType,
    variants: Vec<u32>,
    damage: Option<f32>,
}
```

**Autotiling**

- Check neighbors when placing
- Select correct sprite variant
- Update neighbors when tile changes

**Hazard Systems**

- Detect entity on hazard tile
- Apply damage or effect
- Trigger particles/sounds
