# Dash Trail

Afterimage particles during player dash. Visual feedback for fast movement ability.

## Core Logic

**Parameters**

- Position (vec2) - dash start location

**Behavior**

- Spawns when dash begins
- Lingering particles that fade
- Creates sense of speed and motion blur
- Short to medium lifetime

**Invariants**

- One-shot effect at dash start, or
- Continuous emission during dash
- Particles stay where spawned (no follow)

---

## Bevy Integration

**Trigger**

- On dash start (Dashing component added)
- Position: player location at dash start

**Spawning**

- Create emitter at dash start position
- Or: emit continuously while Dashing component present

**Lifecycle**

- If one-shot: auto-despawns when complete
- If continuous: stops when Dashing removed
