# Tile Sockets

Connection rules defining which tiles can be adjacent. Core constraint system for WFC generation.

## Core Logic

**Concept**

- Each tile edge has a "socket" identifier
- Tiles connect if sockets match
- Enables coherent terrain transitions

**Socket Edges**

| Edge | Direction | Example |
|------|-----------|---------|
| x_pos | Right (+X) | East edge |
| x_neg | Left (-X) | West edge |
| y_pos | Up (+Y) | North edge |
| y_neg | Down (-Y) | South edge |
| z_pos | Above (+Z) | Top layer |
| z_neg | Below (-Z) | Bottom layer |

**Socket Types**

| Type | Connects To | Use Case |
|------|-------------|----------|
| grass | grass | Same terrain |
| grass_water | grass_water | Transition edge |
| water | water | Same terrain |
| empty | empty | Layer stacking |
| tree_base | tree_top | Multi-tile object |

**Connection Rules**

- grass ↔ grass (same material)
- grass_water ↔ water_grass (transition pair)
- tree_base ↔ tree_top (vertical object)
- empty ↔ any (layer above)

**Operations**

`can_connect(socket_a, socket_b)` - Check compatibility

- Return true if sockets can be adjacent
- Symmetric check

`get_sockets(tile)` - Get tile edges

- Return all 4-6 socket identifiers
- Used for constraint propagation

`add_rule(socket_a, socket_b)` - Define connection

- Register valid pairing
- Update constraint table

**Invariants**

- Connection rules are symmetric
- Every tile has all edges defined
- No undefined socket connections
- Transition tiles bridge materials

**Transition Tiles**

```
[grass][grass→water][water]
         ^
    Transition tile with:
    - x_neg: grass
    - x_pos: water
```

---

## Bevy Integration

**Data Structure**

```rust
struct TileDefinition {
    id: TileId,
    sockets: Sockets,
    weight: f32,
    sprite_index: u32,
}

struct Sockets {
    x_pos: SocketId,
    x_neg: SocketId,
    y_pos: SocketId,
    y_neg: SocketId,
}
```

**Resources**

- SocketRules { connections: HashSet<(SocketId, SocketId)> }
- TileDefinitions { tiles: Vec<TileDefinition> }

**Loading**

- Define in RON/JSON
- Or procedurally from tileset analysis
- Validate on load (no orphan sockets)

**Weight System**

- Common tiles: weight 5.0 (grass)
- Rare tiles: weight 0.1 (special features)
- Transitions: weight 1.0 (edges)

**Multi-Tile Objects**

```rust
// Tree spanning 2 vertical tiles
TileDefinition {
    id: TREE_BOTTOM,
    sockets: Sockets {
        z_pos: socket("tree_base"),
        ..default_ground()
    },
}
TileDefinition {
    id: TREE_TOP,
    sockets: Sockets {
        z_neg: socket("tree_top"), // matches tree_base
        ..default_empty()
    },
}
```
