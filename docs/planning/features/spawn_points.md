# Spawn Points

Predetermined locations where enemies can spawn. Distributed around arena avoiding player.

## Core Logic

**Generation**

- Select N random walkable tiles
- Exclude center area (player spawn)
- Minimum distance from each other (optional)
- Store as world positions

**Operations**

`generate_spawn_points(arena, count)` - Create spawn points

- Get all walkable tiles
- Filter out center region
- Randomly select count positions
- Convert to world coordinates

`get_random_spawn()` - Get one spawn point

- Return random position from list

`get_spawn_away_from(position, min_distance)` - Get distant spawn

- Filter spawn points by distance
- Return random from filtered list

**Invariants**

- Never spawn on walls
- Never spawn at arena center
- Spawn points reusable across waves
- World positions, not tile coordinates

**Defaults**

| Field          | Value | Description        |
| -------------- | ----- | ------------------ |
| Count          | 10    | Pre-generated points |
| Center exclusion | 3   | Tiles from center  |

---

## Bevy Integration

**Resource**

- SpawnPoints { positions: Vec<Vec2> }

**Generation**

- Created during arena generation
- Stored for entire gameplay session

**Usage**

- Wave system queries spawn points
- Spawns enemies at random selections
- Can spawn multiple at same point if needed
