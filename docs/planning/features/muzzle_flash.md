# Muzzle Flash

Brief bright burst at projectile origin. Visual feedback that an attack was fired.

## Core Logic

**Parameters**

- Position (vec2) - spawn location
- Rotation (f32) - direction facing (radians)

**Behavior**

- Spawns at attack origin
- Oriented in attack direction
- Brief lifetime (< 0.1s typical)
- Expanding bright particles
- Self-destructs when complete

**Invariants**

- One-shot effect (not continuous)
- Does not follow the projectile
- Rotation affects emission direction

---

## Bevy Integration

**Trigger**

- On projectile attack spawned
- Position: attack origin
- Rotation: attack direction angle

**Spawning**

- Create emitter entity at position with rotation
- Particle engine handles simulation and cleanup

**Lifecycle**

- Emitter entity auto-despawns when effect completes
- No manual cleanup required
