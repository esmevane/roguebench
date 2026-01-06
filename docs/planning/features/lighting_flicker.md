# Light Flicker

Organic flickering effect for torches, candles, and unstable light sources.

## Core Logic

**Concept**

- Intensity varies over time
- Random but organic feel
- Simulates flame movement

**Flicker Types**

| Type | Pattern | Use Case |
|------|---------|----------|
| Candle | Subtle, slow | Small flames |
| Torch | Medium, irregular | Wall sconces |
| Fire | Large, chaotic | Bonfires |
| Dying | Decreasing, sputtering | Low fuel |
| Electric | Sharp, digital | Broken lights |

**Parameters**

| Property | Type | Description |
|----------|------|-------------|
| base_intensity | f32 | Center intensity |
| variance | f32 | Max deviation (±%) |
| speed | f32 | Flicker rate |
| noise_scale | f32 | Randomness granularity |

**Operations**

`update_flicker(dt)` - Advance flicker state

- Sample noise at current time
- Calculate intensity offset
- Apply to light

`set_flicker_type(type)` - Apply preset

- Configure parameters for type
- Reset phase

**Noise Function**

- Perlin or simplex noise for smooth variation
- Multiple octaves for complexity
- Seeded per light for uniqueness

**Invariants**

- Intensity stays within bounds
- Flicker is deterministic per seed
- No sudden jumps (smooth transitions)
- Can be disabled per light

**Defaults**

| Type | Variance | Speed |
|------|----------|-------|
| Candle | 10% | 3.0 |
| Torch | 20% | 5.0 |
| Fire | 30% | 8.0 |
| Electric | 50% | 20.0 |

---

## Bevy Integration

**Components**

- LightFlicker { variance, speed, noise_scale, phase }
- FlickerType enum

**Systems**

- Update flicker phase with time
- Sample noise function
- Modify light intensity

**Implementation**

```rust
fn update_flickers(
    time: Res<Time>,
    mut query: Query<(&mut PointLight2d, &mut LightFlicker)>,
) {
    for (mut light, mut flicker) in &mut query {
        flicker.phase += time.delta_secs() * flicker.speed;
        let noise = simplex_noise(flicker.phase, flicker.seed);
        let offset = noise * flicker.variance;
        light.intensity = flicker.base_intensity * (1.0 + offset);
    }
}
```

**Noise Options**

- fastnoise-lite crate
- Custom simplex implementation
- Precomputed noise table

**Spawning**

```rust
commands.spawn((
    PointLight2d { intensity: 1.0, .. },
    LightFlicker::torch(),
));
```
