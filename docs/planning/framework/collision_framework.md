# Collision Framework

Integration with Avian 2D physics for collision detection and game logic.

## Relationship to Avian 2D

**Avian 2D handles physics.** We don't implement custom collision detection. This framework describes:

1. How to configure Avian colliders for our game
2. How Avian collision events flow into game logic (command bus)
3. Collision layer conventions
4. Server authority model

```
Avian 2D (physics engine)
    ↓ collision events
Collision Framework (this doc)
    ↓ game events
Command Bus (damage, triggers, etc.)
```

## Server Authority

Physics simulation runs on **server only** in `FixedMain` schedule.

- Server: Runs Avian simulation, detects collisions, generates game events
- Client: Receives replicated Avian components (Transform, LinearVelocity) via Lightyear
- Client: May run local Avian for prediction, but server is authoritative

Collision-triggered game events (damage, pickups) only happen on server.

---

## Avian 2D Components

We use Avian's built-in components. Here's how we configure them:

**RigidBody types:**

| Type | Use Case |
|------|----------|
| `RigidBody::Dynamic` | Player, enemies, projectiles |
| `RigidBody::Static` | Walls, terrain, obstacles |
| `RigidBody::Kinematic` | Platforms, scripted movement |

**Collider shapes:**

| Shape | Use Case |
|-------|----------|
| `Collider::circle(radius)` | Characters, projectiles |
| `Collider::rectangle(w, h)` | Tiles, large objects |
| `Collider::capsule(h, r)` | Tall characters |

**Sensors (triggers):**

```rust
// Sensor colliders detect overlap but don't block movement
commands.spawn((
    Collider::circle(pickup_radius),
    Sensor, // Avian marker component
    Pickup { item_id },
));
```

---

## Collision Layers

Use Avian's `CollisionLayers` for filtering:

```rust
#[derive(PhysicsLayer)]
enum GameLayer {
    Player,
    Enemy,
    Terrain,
    PlayerProjectile,
    EnemyProjectile,
    Pickup,
    Trigger,
}

// Player collides with terrain, enemies, enemy projectiles, pickups
fn player_layers() -> CollisionLayers {
    CollisionLayers::new(
        GameLayer::Player,
        [GameLayer::Terrain, GameLayer::Enemy, GameLayer::EnemyProjectile, GameLayer::Pickup]
    )
}

// Enemy projectile only hits player
fn enemy_projectile_layers() -> CollisionLayers {
    CollisionLayers::new(
        GameLayer::EnemyProjectile,
        [GameLayer::Player, GameLayer::Terrain]
    )
}
```

---

## Handling Collision Events

Avian fires collision events. We observe them and trigger game logic:

```rust
fn handle_collisions(
    mut collision_events: EventReader<CollisionStarted>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut command_bus: ResMut<CommandBus>,
) {
    for CollisionStarted(entity_a, entity_b) in collision_events.read() {
        // Player hit enemy
        if let (Ok(player), Ok(enemy)) = (
            player_query.get(*entity_a).or(player_query.get(*entity_b)),
            enemy_query.get(*entity_a).or(enemy_query.get(*entity_b))
        ) {
            command_bus.send(ApplyDamage {
                target: player,
                amount: 10,
                source: Some(enemy),
            });
        }
    }
}
```

**Collision event types (from Avian):**

- `CollisionStarted(Entity, Entity)` — first frame of contact
- `CollisionEnded(Entity, Entity)` — contact ended
- `Collision { entity1, entity2, ... }` — ongoing contact with details

---

## Tile-Based Collision

For room walls/floors, spawn static colliders:

```rust
fn spawn_room_colliders(
    room: &RoomData,
    commands: &mut Commands,
) {
    for (x, y, tile) in room.tiles() {
        if tile == TileKind::Wall {
            commands.spawn((
                RigidBody::Static,
                Collider::rectangle(TILE_SIZE, TILE_SIZE),
                Transform::from_xyz(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    0.0
                ),
                CollisionLayers::new(GameLayer::Terrain, [GameLayer::Player, GameLayer::Enemy]),
            ));
        }
    }
}
```

For large rooms, consider composite colliders or spatial optimization.

---

## Triggers and Sensors

Sensors detect overlap without physics response:

```rust
// Pickup detection
fn handle_pickups(
    mut collision_events: EventReader<CollisionStarted>,
    sensor_query: Query<(Entity, &Pickup), With<Sensor>>,
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
    mut command_bus: ResMut<CommandBus>,
) {
    for CollisionStarted(a, b) in collision_events.read() {
        // Check if one is player, other is pickup sensor
        let (player, pickup_entity, pickup) = /* resolve entities */;
        
        command_bus.send(CollectPickup {
            player,
            item_id: pickup.item_id.clone(),
        });
        
        commands.entity(pickup_entity).despawn();
    }
}
```

---

## Queries

Use Avian's spatial queries:

```rust
fn check_line_of_sight(
    spatial_query: SpatialQuery,
    origin: Vec2,
    target: Vec2,
) -> bool {
    let direction = (target - origin).normalize();
    let distance = origin.distance(target);
    
    // Cast ray, check if it hits terrain before reaching target
    if let Some(hit) = spatial_query.cast_ray(
        origin,
        direction,
        distance,
        true, // solid only
        SpatialQueryFilter::from_mask(GameLayer::Terrain),
    ) {
        hit.time_of_impact >= distance - 0.1 // reached target
    } else {
        true // no obstacle
    }
}
```

**Available queries:**

- `cast_ray` — line trace, first hit
- `cast_shape` — swept shape query
- `point_intersections` — entities at point
- `shape_intersections` — entities overlapping shape

---

## Replication

Avian components replicated via lightyear_avian2d:

- `Transform` (position)
- `LinearVelocity`
- `AngularVelocity`
- `RigidBody` type

Collider shapes and layers are typically static (don't change at runtime), so they're spawned identically on server and client from archetype definitions.

---

## Debug Visualization

Use Avian's debug rendering:

```rust
app.add_plugins(PhysicsDebugPlugin::default());
```

This draws collider outlines, contact points, and ray casts. Enable only in dev builds.

---

## What's Not Here

- Custom collision algorithms (Avian handles this)
- Continuous collision detection settings (use Avian's CCD if needed)
- Complex polygon colliders (add when needed)

*See: framework/movement_framework.md (movement uses Avian velocity)*
