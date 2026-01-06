# Entity Collision

Collision response behavior when entities contact each other.

## Core Logic

**Collision Types**

| Type | Behavior | Example |
|------|----------|---------|
| Block | Stop movement | Wall, solid obstacle |
| Push | Displace other | Player vs enemy |
| Overlap | Pass through, trigger | Pickup, attack hitbox |
| Sensor | Detect only | Aggro range, trigger zone |

**Entity Collision Matrix**

| A vs B | Response |
|--------|----------|
| Player vs Wall | Block |
| Player vs Enemy | Push (mutual) |
| Player vs Cadaver | Push (cadaver moves) |
| Player vs Pickup | Overlap (collect) |
| Enemy vs Wall | Block |
| Enemy vs Enemy | Push (mutual) |
| Enemy vs Cadaver | Push (cadaver moves) |
| Projectile vs Wall | Block (destroy) |
| Projectile vs Enemy | Overlap (damage) |

**Push Behavior**

- Mass determines push ratio
- Heavier entities pushed less
- Equal mass = equal push
- Immovable entities (walls) never pushed

**Mass Values**

| Entity | Mass | Pushability |
|--------|------|-------------|
| Player | 1.0 | Normal |
| Grunt | 0.8 | Easier to push |
| Brute | 2.0 | Harder to push |
| Cadaver | 0.5 | Very pushable |
| Wall | ∞ | Immovable |

**Operations**

`resolve_collision(a, b)` - Handle collision

- Determine collision type
- Apply appropriate response
- Separate overlapping entities

`calculate_push(a_mass, b_mass, overlap)` - Push resolution

- Distribute separation by inverse mass
- Return displacement for each entity

**Invariants**

- No entity overlap after resolution
- Walls never move
- Push direction away from collision
- Mass affects push ratio, not collision detection

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Separation iterations | 4 | Physics solver iterations |
| Push damping | 0.8 | Reduce bounce |

---

## Bevy Integration

**Physics Engine**

- Avian2D handles collision detection
- Collision response via RigidBody types
- Custom sensors via CollisionStarted events

**Components**

- RigidBody::Dynamic - pushable
- RigidBody::Static - immovable (walls)
- RigidBody::Kinematic - script-controlled
- Sensor - overlap detection only
- ColliderMassProperties - mass for pushing

**Events**

- CollisionStarted { a, b }
- CollisionEnded { a, b }

**Systems**

- Listen for collision events
- Trigger gameplay responses (damage, pickup)
- Custom push logic if needed

**Collision Callbacks**

```rust
fn on_collision(
    mut events: EventReader<CollisionStarted>,
    players: Query<&Player>,
    pickups: Query<&Pickup>,
) {
    for CollisionStarted(a, b) in events.read() {
        if players.contains(*a) && pickups.contains(*b) {
            // Collect pickup
        }
    }
}
```

**Sensor Usage**

- Aggro detection radius
- Trigger zones
- Attack hitboxes (no physical push)
