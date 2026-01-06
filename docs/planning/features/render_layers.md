# Render Layers

Z-ordering and layer system for visual depth and draw order.

## Core Logic

**Layer Order** (low to high)

| Layer | Z | Content |
|-------|---|---------|
| Floor | 0 | Tilemap floor layer |
| Floor Details | 1 | Decals, cracks, stains |
| Shadows | 2 | Entity shadows |
| Ground Effects | 3 | Blood pools, scorch marks |
| Items | 4 | Pickups, drops |
| Corpses | 5 | Dead bodies (cadavers) |
| Entities | 10 | Players, enemies |
| Entity Effects | 11 | Hit flashes, auras |
| Projectiles | 12 | Bullets, arrows |
| Particles | 20 | Particle effects |
| Weather | 25 | Rain, fog overlay |
| UI World | 30 | Health bars, floating text |
| Screen Effects | 40 | Screen flash, vignette |
| UI | 50 | HUD, menus |

**Y-Sorting**

- Entities on same layer sorted by Y position
- Lower Y (higher on screen) draws first
- Creates depth illusion

**Operations**

`get_z(layer)` - Get base Z value

- Return Z for layer
- Used when spawning entities

`get_render_z(layer, y_position)` - Get sorted Z

- Combine layer Z with Y-sort offset
- For entities that need Y-sorting

**Y-Sort Calculation**

```
render_z = layer_z + (1.0 - normalize(y, min_y, max_y)) * 0.9
```

**Invariants**

- Higher layers always draw over lower
- Y-sorting only within same layer
- Z values don't overlap between layers
- UI always on top

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Layer gap | 10 | Z space between layers |
| Y-sort range | 0.9 | Within-layer Z range |

---

## Bevy Integration

**Constants**

```rust
pub mod RenderLayer {
    pub const FLOOR: f32 = 0.0;
    pub const SHADOWS: f32 = 2.0;
    pub const ENTITIES: f32 = 10.0;
    pub const PARTICLES: f32 = 20.0;
    pub const UI: f32 = 50.0;
}
```

**Components**

- RenderLayer(f32) - base layer
- YSorted - marker for Y-sorting

**Systems**

- Update Transform.translation.z based on layer
- Apply Y-sorting for marked entities
- Run in PostUpdate before rendering

**Y-Sort System**

```rust
fn y_sort(mut query: Query<&mut Transform, With<YSorted>>) {
    for mut transform in &mut query {
        let y_offset = /* calculate from y position */;
        transform.translation.z = ENTITIES + y_offset;
    }
}
```

**Camera Layers**

- RenderLayers component for selective rendering
- Main camera sees gameplay layers
- UI camera sees UI layer only
- Separate cameras for post-processing
