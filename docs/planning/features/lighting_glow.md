# Light Glow

Soft bloom and aura effects around light sources and emissive objects.

## Core Logic

**Concept**

- Soft halo around bright objects
- Simulates light scattering
- Enhances magical/ethereal feel

**Glow Types**

| Type | Appearance | Use Case |
|------|------------|----------|
| Bloom | Bright overflow | Intense lights |
| Aura | Colored halo | Magic, power-ups |
| Pulse | Breathing glow | Ambient magic |
| Corona | Ring effect | Special items |

**Parameters**

| Property | Type | Description |
|----------|------|-------------|
| color | Color | Glow tint |
| intensity | f32 | Glow brightness |
| radius | f32 | Glow spread |
| falloff | f32 | Edge softness |
| threshold | f32 | Bloom threshold |

**Operations**

`set_glow(entity, properties)` - Add glow effect

- Apply glow parameters
- Enable in render pass

`pulse_glow(entity, min, max, speed)` - Animate glow

- Oscillate intensity
- Sine wave or custom curve

`remove_glow(entity)` - Disable glow

- Remove glow effect
- Clean up resources

**Bloom Pipeline**

1. Extract bright pixels (above threshold)
2. Downsample and blur
3. Upsample with blending
4. Composite over scene

**Invariants**

- Glow doesn't affect gameplay
- Performance scales with resolution
- Can be disabled in settings
- Intensity clamped to prevent blowout

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Threshold | 0.8 | Bloom cutoff |
| Radius | 16px | Blur radius |
| Intensity | 0.5 | Bloom strength |

---

## Bevy Integration

**Post-Processing**

- bevy_bloom or custom
- Render to HDR buffer
- Apply bloom pass

**Components**

- Glow { color, intensity, radius }
- GlowPulse { min, max, speed, phase }
- EmissiveSprite - marks sprite for bloom

**Resources**

- BloomSettings { threshold, intensity, radius }

**Systems**

- Update pulse animations
- Configure bloom per-entity (if supported)
- Fallback: emissive sprite overlay

**Sprite-Based Fallback**

If no post-process bloom:
- Additional sprite behind entity
- Additive blend mode
- Larger, tinted, semi-transparent

```rust
commands.spawn((
    Sprite {
        color: Color::srgba(1.0, 0.8, 0.2, 0.3),
        custom_size: Some(Vec2::splat(64.0)),
        ..default()
    },
    GlowSprite { parent: entity },
    RenderLayer::EFFECTS,
));
```

**Pulse System**

```rust
fn update_glow_pulse(
    time: Res<Time>,
    mut query: Query<(&mut Glow, &mut GlowPulse)>,
) {
    for (mut glow, mut pulse) in &mut query {
        pulse.phase += time.delta_secs() * pulse.speed;
        let t = (pulse.phase.sin() + 1.0) / 2.0;
        glow.intensity = pulse.min + (pulse.max - pulse.min) * t;
    }
}
```
