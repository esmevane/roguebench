# Bullet Trail

Continuous particle stream following a projectile. Creates visual trace of projectile path.

## Core Logic

**Parameters**

- Entity - projectile to follow
- Emission rate (f32) - particles per second

**Behavior**

- Emits particles at projectile position each frame
- Particles have short lifetime, fade quickly
- Trail length determined by particle lifetime + projectile speed
- Stops emitting when projectile despawns

**Invariants**

- Continuous effect (not one-shot)
- Follows entity position
- Particles inherit no velocity (stationary once spawned)

---

## Bevy Integration

**Trigger**

- On projectile spawn
- Attach emitter to projectile entity

**Spawning**

- Create emitter as child of projectile, or
- Create emitter that samples projectile position each frame

**Lifecycle**

- Emitter despawns with projectile, or
- Emitter stops when projectile gone, remaining particles fade
