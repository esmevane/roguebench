# Landing Smoke

Dust cloud when entity lands on ground or stops moving suddenly. Conveys weight and impact with the environment.

## Core Logic

**Parameters**

- Position (vec2) - ground contact point
- Intensity (f32) - size/amount based on fall speed or stop force

**Behavior**

- Spawns at ground level beneath entity
- Particles spread horizontally outward
- Brown/gray dust color
- Quick fade (< 0.5s)
- Size scales with intensity

**Invariants**

- One-shot effect
- Horizontal spread, not vertical
- Intensity affects particle count and spread radius
- Ground-aligned (spawns at feet, not center)

---

## Bevy Integration

**Trigger**

- On ground contact after airborne, or
- On sudden velocity change (dash end, knockback stop)
- Position: entity feet position
- Intensity: based on vertical velocity or speed delta

**Spawning**

- Create emitter at ground position
- Scale particle count/spread by intensity

**Lifecycle**

- Emitter auto-despawns when complete
