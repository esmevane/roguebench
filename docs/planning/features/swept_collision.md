# Swept Collision

Continuous collision detection preventing fast objects from tunneling through obstacles.

## Core Logic

**Problem**

- Discrete collision checks position each frame
- Fast objects can skip over thin walls
- "Tunneling" or "bullet through paper"

**Solution**

- Check entire movement path, not just endpoints
- Sweep collider along velocity
- Detect first collision along path

**Algorithm**

1. Get start position and velocity
2. Calculate end position (start + velocity × dt)
3. Sweep collider from start to end
4. Find first intersection point
5. Stop at collision (or adjust)

**Sweep Types**

| Type | Shape | Use Case |
|------|-------|----------|
| Ray | Point | Bullets, thin projectiles |
| Circle | Circle | Characters, enemies |
| Box | Rectangle | Vehicles, large objects |

**Operations**

`sweep_circle(start, end, radius)` - Sweep circle

- Return first collision along path
- Include collision normal and time

`sweep_ray(start, direction, length)` - Sweep point

- Return hit position and distance
- Fastest, for thin projectiles

`validate_movement(start, velocity, collider)` - Check move

- Sweep against collision map
- Return safe end position

**Collision Result**

| Field | Type | Description |
|-------|------|-------------|
| hit | bool | Did collision occur |
| point | Vec2 | Collision position |
| normal | Vec2 | Surface normal |
| time | f32 | 0-1 along path |
| tile | TileType | What was hit |

**Invariants**

- No tunneling regardless of speed
- Collision time in range [0, 1]
- Normal points away from surface
- Handles corner cases (literally)

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Max iterations | 4 | Collision solving steps |
| Epsilon | 0.001 | Separation distance |

---

## Bevy Integration

**Components**

- SweptCollider { shape: ColliderShape }
- Velocity(Vec2) - used for sweep path

**Systems**

- Run after input, before transform update
- Sweep collider along velocity
- Adjust velocity or position on hit

**Implementation**

```rust
fn validate_movement(
    mut query: Query<(&Transform, &mut Velocity, &SweptCollider)>,
    collision_map: Res<CollisionMap>,
) {
    for (transform, mut velocity, collider) in &mut query {
        let start = transform.translation.truncate();
        let end = start + velocity.0;

        if let Some(hit) = sweep_circle(
            start, end, collider.radius, &collision_map
        ) {
            // Stop at collision point
            velocity.0 = (hit.point - start) * 0.99;
        }
    }
}
```

**Axis Sliding**

- On collision, try movement along each axis separately
- Allows sliding along walls
- Prevents getting stuck on corners

```rust
if hit_x && !hit_y {
    velocity.x = 0.0; // Block X, keep Y
} else if hit_y && !hit_x {
    velocity.y = 0.0; // Block Y, keep X
}
```

**Circle-Rectangle Sweep**

- Find closest point on rectangle to circle path
- Check if circle passes within radius
- Standard algorithm for 2D tile collision
