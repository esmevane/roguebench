# Lighting System

Core lighting architecture for dynamic illumination in 2D.

## Core Logic

**Light Types**

| Type | Behavior | Use Case |
|------|----------|----------|
| Point | Radial falloff | Torches, explosions |
| Spot | Cone-shaped | Flashlights, directed |
| Ambient | Global fill | Base visibility |
| Directional | Parallel rays | Sun, moon |

**Light Properties**

| Property | Type | Description |
|----------|------|-------------|
| color | Color | Light tint |
| intensity | f32 | Brightness multiplier |
| radius | f32 | Falloff distance |
| falloff | enum | Linear, Quadratic, None |
| shadows | bool | Cast shadows |

**Operations**

`add_light(position, properties)` - Create light

- Spawn light entity
- Configure properties
- Add to lighting pass

`remove_light(entity)` - Remove light

- Despawn light entity
- Update lighting

`set_intensity(entity, value)` - Adjust brightness

- Modify light intensity
- Used for effects (flicker, pulse)

`set_ambient(color, intensity)` - Set base lighting

- Global illumination level
- Affects entire scene

**Falloff Calculation**

```
Linear:    attenuation = 1 - (distance / radius)
Quadratic: attenuation = 1 / (1 + distance²)
```

**Invariants**

- Lights blend additively
- Ambient is minimum illumination
- Shadows optional per light
- Performance scales with light count

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Ambient | 0.1 | Base darkness |
| Max lights | 64 | Per-frame limit |
| Shadow resolution | 256 | Shadow map size |

---

## Bevy Integration

**Libraries**

- bevy_light_2d - 2D lighting
- bevy_radiance_cascades - advanced GI
- Custom shader approach

**Components**

- PointLight2d { color, intensity, radius, falloff }
- SpotLight2d { color, intensity, radius, angle }
- AmbientLight2d { color, intensity }

**Resources**

- LightingConfig { ambient, max_lights, shadows_enabled }

**Systems**

- Gather visible lights
- Render shadow maps (if enabled)
- Composite lighting pass
- Apply to sprites

**Render Pipeline**

1. Render scene to albedo buffer
2. Render lights to light buffer
3. Multiply albedo × light
4. Output final image

**Performance**

- Frustum cull off-screen lights
- Limit shadow-casting lights
- LOD for distant lights
