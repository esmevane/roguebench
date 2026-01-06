# Wall Hit

Particle burst when projectile or entity collides with wall. Distinct from entity hit - shows environmental impact.

## Core Logic

**Parameters**

- Position (vec2) - collision point
- Normal (vec2) - wall surface direction
- Intensity (f32) - impact force

**Behavior**

- Spawns at collision point
- Particles reflect off wall (biased along normal)
- Spark/debris appearance
- May include brief flash at impact point
- Short lifetime

**Invariants**

- One-shot effect
- Particles move away from wall (along normal)
- Visually distinct from entity hit (different color/shape)
- Intensity affects particle count and spread

**Variants**

| Surface  | Color        | Character           |
| -------- | ------------ | ------------------- |
| Stone    | Gray/orange  | Sparks, dust        |
| Wood     | Brown        | Splinters           |
| Metal    | White/yellow | Bright sparks       |

---

## Bevy Integration

**Trigger**

- On projectile collision with wall/tilemap
- On entity collision with wall (optional, for dashing into walls)
- Position: collision point
- Normal: from collision data

**Spawning**

- Create emitter at collision point
- Orient particles along wall normal

**Lifecycle**

- Emitter auto-despawns when complete
