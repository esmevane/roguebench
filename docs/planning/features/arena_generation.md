# Arena Generation

Generation of playable arena rooms using WFC and other procedural techniques.

## Core Logic

**Relationship to Other Features**

- Arenas are a type of *room* (see `room_system.md`)
- Uses *WFC generation* as canonical approach (see `wfc_generation.md`)
- Respects *tile system* for rendering (see `tilemap.md`, `tile_types.md`)
- Provides *spawn points* for enemies (see `spawn_points.md`)

**Concept**

- "Arena" is a room category (combat-focused)
- Generated using WFC for coherent terrain
- May have layered generation (terrain + props + spawns)
- Design defines arena variants

**Generation Layers**

| Layer | Technique | Content |
|-------|-----------|---------|
| Base | WFC | Floor tiles, walls |
| Features | Rules | Obstacles, cover |
| Props | Weighted random | Decorations |
| Spawns | Algorithm | Enemy spawn points |

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| size | Vec2 | Arena dimensions |
| variant | string | Arena variant type |
| difficulty | float | Affects complexity |
| seed | option | For determinism |

**Operations**

`generate_arena(params)` - Create arena

- Run WFC for base layout
- Apply variant rules
- Place features and props
- Calculate spawn points

`get_spawn_points(arena)` - Query spawns

- Return enemy spawn locations
- Excludes player area

**Invariants**

- All arenas are playable (no blocked paths)
- Player spawn area always clear
- Spawn points on walkable tiles
- Matches room system interface

**Design Notes**

- Specific variants left to design
- WFC tile sets defined in data
- Arena = room category, not separate concept

---

## Bevy Integration

**Resources**

- ArenaConfig { default_size, variants }
- GeneratedArena { tiles, spawns, features }

**Integration**

- Uses `wfc_generation.md` for tile placement
- Outputs to `tilemap.md` for rendering
- Provides data to `spawn_points.md`
- Registered as room type in `room_system.md`

**Messages/Commands**

- GenerateArena { params }
- RegenerateArena { seed }

**Events**

- ArenaGenerated { arena_id, size, variant }

**Scripting Compatibility**

- Arena variants definable in data
- Generation hookable for custom rules
- Spawn point queries available

*See: wfc_generation.md, room_system.md, tile_sockets.md*
