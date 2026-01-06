# Blood Spray

Red particle burst when damage is applied. Visceral feedback for damage taken.

## Core Logic

**Parameters**

- Position (vec2) - damaged entity location

**Behavior**

- Spawns at damaged entity location
- Red/dark particles
- Short travel distance (droplets)
- Omnidirectional or biased away from damage source
- Quick lifetime

**Invariants**

- One-shot effect
- Triggered on damage, not just hit
- Distinct from hit impact (damage vs connection)

---

## Bevy Integration

**Trigger**

- On damage event (health reduced)
- Position: damaged entity location

**Spawning**

- Create emitter entity at damage position
- Optional: bias direction away from damage source

**Lifecycle**

- Emitter auto-despawns when complete
