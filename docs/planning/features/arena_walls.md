# Arena Walls

Collision boundaries that contain entities within the arena. Prevents movement outside play area.

## Core Logic

**Types**

- Perimeter walls - visible, around arena edge
- Invisible walls - physics-only barriers

**Properties**

- Static rigid body (immovable)
- Rectangle collider
- Blocks all entity movement
- Blocks projectiles

**Operations**

`spawn_perimeter(width, height, tile_size)` - Create edge walls

- Spawn wall segments along all four edges
- Each segment is one tile wide
- Full arena height/width coverage

`spawn_invisible_barrier(position, size)` - Create physics wall

- Spawn collider without sprite
- Used for additional containment

**Invariants**

- Walls never move
- Walls block all entity types
- Complete enclosure (no gaps)
- Collisions stop movement, don't bounce

---

## Bevy Integration

**Components**

- Wall marker
- RigidBody::Static
- Collider::rectangle
- Visibility::Hidden (for invisible walls)

**Collision Layers**

- Wall layer collides with:
  - Player
  - Enemy
  - PlayerAttack
  - EnemyAttack
  - Cadaver

**Spawning**

- Created during arena generation
- Persist entire gameplay session
