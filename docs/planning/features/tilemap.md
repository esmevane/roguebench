# Tilemap

Tile-based map rendering system. Efficiently draws large tile grids with batching.

## Core Logic

**Structure**

- Grid of tiles (2D array)
- Multiple layers (floor, walls, decorations)
- Tile size in pixels

**Layers**

| Layer | Z-Order | Content |
|-------|---------|---------|
| Floor | 0 | Base ground tiles |
| Floor Detail | 1 | Cracks, stains, variations |
| Walls | 2 | Wall tiles, obstacles |
| Wall Top | 3 | Wall tops (for depth) |

**Operations**

`set_tile(layer, x, y, tile_id)` - Place tile

- Update tile at grid position
- Mark chunk dirty for re-render

`get_tile(layer, x, y)` - Query tile

- Return tile ID at position
- Return None if out of bounds

`clear_layer(layer)` - Remove all tiles

- Set all positions to empty
- Used for regeneration

`world_to_tile(position)` - Convert coordinates

- World position to grid coordinates
- Accounts for tile size

`tile_to_world(x, y)` - Convert coordinates

- Grid coordinates to world position
- Returns tile center

**Invariants**

- Tiles render in layer order
- Chunk-based batching for performance
- Tile coordinates are integers
- World coordinates are floats

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Tile size | 16x16 | Pixels per tile |
| Chunk size | 16x16 | Tiles per batch |

---

## Bevy Integration

**Libraries**

- bevy_ecs_tilemap - efficient tile rendering
- Or custom sprite batching

**Components**

- Tilemap { size, tile_size, layers }
- TilemapLayer { tiles: Vec<Option<TileId>> }
- TileId(u32) - index into tileset

**Resources**

- TilesetAtlas - sprite sheet for tiles
- TileDefinitions - tile type data

**Systems**

- Render tilemap layers in order
- Update dirty chunks
- Handle tilemap spawn/despawn

**Spawning**

```rust
commands.spawn(TilemapBundle {
    tilemap: Tilemap::new(width, height, tile_size),
    transform: Transform::default(),
    ..default()
});
```

**Performance**

- Batch tiles into chunks
- Only re-render dirty chunks
- Frustum culling for large maps
