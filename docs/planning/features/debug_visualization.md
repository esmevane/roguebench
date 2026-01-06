# Debug Visualization

Runtime debug rendering for collision, AI, and system state inspection.

## Core Logic

**Concept**

- Visual overlays for debugging
- Toggle on/off at runtime
- Development builds only
- No performance impact when disabled

**Visualization Types**

| Type | Shape | Color | Purpose |
|------|-------|-------|---------|
| Collision tiles | Rectangle | Green/Red | Walkable/blocked |
| Entity colliders | Circle/Rect | Cyan | Collision bounds |
| Current tile | Rectangle outline | Yellow | Player grid cell |
| Invalid position | X mark | Red | In unwalkable tile |
| Velocity | Line | Blue | Movement direction |
| Path | Line strip | Orange | AI pathfinding |
| Aggro range | Circle | Purple | Detection radius |
| Attack range | Arc/Circle | Red | Weapon reach |

**Operations**

`toggle_debug()` - Enable/disable

- Flip debug rendering state
- Typically bound to F3

`draw_rect(pos, size, color)` - Draw rectangle

- One-frame gizmo
- Outline or filled

`draw_circle(pos, radius, color)` - Draw circle

- One-frame gizmo
- For colliders, ranges

`draw_line(start, end, color)` - Draw line

- One-frame gizmo
- For velocity, paths

**Debug Panels**

| Panel | Information |
|-------|-------------|
| FPS | Frame time, frame rate |
| Entities | Count by type |
| Physics | Collision checks, active bodies |
| Memory | Heap usage |
| Network | Ping, packet loss |

**Invariants**

- Only in debug builds (cfg)
- Zero overhead when disabled
- Gizmos clear each frame
- Colors consistent across systems

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Toggle key | F3 | Debug mode |
| Default state | Off | Start disabled |

---

## Bevy Integration

**Resources**

- DebugConfig { enabled, show_colliders, show_paths, ... }

**Gizmos**

```rust
fn draw_debug_colliders(
    debug: Res<DebugConfig>,
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Collider)>,
) {
    if !debug.enabled { return; }

    for (transform, collider) in &query {
        gizmos.circle_2d(
            transform.translation.truncate(),
            collider.radius,
            Color::srgb(0.0, 1.0, 1.0),
        );
    }
}
```

**Conditional Compilation**

```rust
#[cfg(debug_assertions)]
fn setup_debug(mut commands: Commands) {
    commands.insert_resource(DebugConfig::default());
}

#[cfg(debug_assertions)]
app.add_systems(Update, draw_debug_colliders);
```

**Toggle System**

```rust
fn toggle_debug(
    input: Res<ButtonInput<KeyCode>>,
    mut debug: ResMut<DebugConfig>,
) {
    if input.just_pressed(KeyCode::F3) {
        debug.enabled = !debug.enabled;
    }
}
```

**Debug UI**

- egui or bevy_ui overlay
- Real-time stats display
- Collapsible panels
- Only in debug builds

**Tile Grid Visualization**

```rust
for x in 0..map.width {
    for y in 0..map.height {
        let world_pos = map.grid_to_world(x, y);
        let color = if map.is_walkable(x, y) {
            Color::srgba(0.0, 1.0, 0.0, 0.3) // Green
        } else {
            Color::srgba(1.0, 0.0, 0.0, 0.3) // Red
        };
        gizmos.rect_2d(world_pos, tile_size, color);
    }
}
```
