# Light Shadows

Dynamic shadow casting from light sources for depth and atmosphere.

## Core Logic

**Shadow Types**

| Type | Method | Use Case |
|------|--------|----------|
| Hard | Ray casting | Sharp, dramatic |
| Soft | Penumbra blur | Realistic, subtle |
| Blob | Simple circle | Performance, arcade |

**Shadow Casters**

| Entity | Casts Shadow | Notes |
|--------|--------------|-------|
| Walls | Yes | Primary occluders |
| Pillars | Yes | Point occluders |
| Entities | Optional | Performance cost |
| Props | Optional | Decoration |

**Parameters**

| Property | Type | Description |
|----------|------|-------------|
| enabled | bool | Shadow casting on |
| softness | f32 | Penumbra width |
| resolution | int | Shadow map size |
| bias | f32 | Prevent self-shadow |

**Operations**

`cast_shadows(light, occluders)` - Generate shadow

- Trace rays from light
- Build shadow geometry
- Render to shadow map

`sample_shadow(position)` - Query shadow

- Check if point is in shadow
- Return shadow factor (0-1)

**2D Shadow Algorithm**

1. For each light, find visible edges of occluders
2. Project edges away from light to infinity
3. Build shadow polygon from projections
4. Render shadow polygons as dark overlay

**Invariants**

- Shadows update with light movement
- Occluders must have collision geometry
- Performance: limit shadow-casting lights
- Shadows don't affect gameplay logic

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Resolution | 256 | Shadow map size |
| Softness | 0.0 | Hard shadows |
| Max casters | 4 | Lights with shadows |

---

## Bevy Integration

**Libraries**

- bevy_light_2d (includes shadows)
- Custom ray casting

**Components**

- ShadowCaster - marks entity as occluder
- ShadowCaster2d { shape } - occluder geometry
- CastsShadow - on lights that cast shadows

**Resources**

- ShadowSettings { resolution, softness, max_casters }

**Systems**

- Gather shadow casters in light range
- Generate shadow geometry
- Render shadow maps
- Composite with lighting

**Occluder Shapes**

```rust
enum ShadowShape {
    Rectangle { half_size: Vec2 },
    Circle { radius: f32 },
    Polygon { points: Vec<Vec2> },
}
```

**Performance Tiers**

| Tier | Shadows | Method |
|------|---------|--------|
| Low | Off | No shadows |
| Medium | Blob | Simple circles under entities |
| High | Hard | Ray-cast shadows |
| Ultra | Soft | Penumbra blur |

**Blob Shadow Fallback**

Simple sprite-based shadow:
```rust
commands.spawn((
    Sprite {
        color: Color::srgba(0.0, 0.0, 0.0, 0.3),
        custom_size: Some(Vec2::new(24.0, 12.0)),
        ..default()
    },
    BlobShadow { parent: entity },
    RenderLayer::SHADOWS,
));
```
