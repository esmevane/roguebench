# Tile Collision Map

Grid-based collision data for tile walkability and obstacle detection.

## Core Logic

**Concept**

- Flat array storing tile walkability
- Separate from visual tilemap
- Efficient spatial queries
- Updated when tiles change

**Tile Classification**

| Type | Walkable | Examples |
|------|----------|----------|
| Floor | Yes | Grass, dirt, stone |
| Wall | No | Brick, rock |
| Water | No | Lake, river |
| Shore | Yes | Water edge (converted) |
| Hazard | Yes* | Spikes, lava (*with damage) |
| Prop | No | Trees, boulders |

**Data Structure**

- Row-major flat Vec<TileType>
- Width × Height size
- Origin offset for centered maps
- O(1) lookup by grid coordinate

**Operations**

`is_walkable(grid_x, grid_y)` - Check passability

- Return true if tile allows movement
- Bounds check included

`world_to_grid(position)` - Convert coordinates

- World position to grid cell
- Accounts for tile size and origin

`grid_to_world(x, y)` - Convert coordinates

- Grid cell to world center
- For entity positioning

`get_tile_type(x, y)` - Query tile

- Return tile type at position
- Used for effects (water splash, etc.)

`set_tile(x, y, type)` - Update tile

- Modify collision map
- Called when terrain changes

**Shore Conversion**

- Water tiles adjacent to land → Shore
- Shore is walkable for natural movement
- Allows characters to approach water edge

**Invariants**

- Map size matches tilemap
- All cells have valid type
- Origin correctly offsets world
- Updates sync with tilemap

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Tile size | 16 | Pixels per tile |
| Default type | Floor | Fallback |

---

## Bevy Integration

**Resources**

```rust
struct CollisionMap {
    tiles: Vec<TileType>,
    width: usize,
    height: usize,
    origin: Vec2,
    tile_size: f32,
}

impl CollisionMap {
    fn is_walkable(&self, x: i32, y: i32) -> bool { ... }
    fn world_to_grid(&self, pos: Vec2) -> (i32, i32) { ... }
}
```

**Generation**

- Built during tilemap generation
- Or loaded from level data
- Shore conversion post-process

**Systems**

- Build collision map on level load
- Query for movement validation
- Update on terrain changes

**Integration with Physics**

- Use for tile-based collision
- Complements entity-entity physics
- Faster than full physics for static geometry

**Coordinate Conversion**

```rust
fn world_to_grid(&self, pos: Vec2) -> (i32, i32) {
    let local = pos - self.origin;
    let x = (local.x / self.tile_size).floor() as i32;
    let y = (local.y / self.tile_size).floor() as i32;
    (x, y)
}
```

**Multi-Layer Support**

- Optional: separate maps per layer
- Or combined walkability
- Props layer blocks ground layer
