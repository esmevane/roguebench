# Player Ranged Attack

Projectile attack that travels in a direction and damages the first enemy hit. Has configurable damage, speed, and cooldown.

## Core Logic

**Parameters**

- Damage (i32) - damage dealt on hit
- Speed (f32) - projectile travel speed
- Lifetime (f32) - max time before despawn
- Size (f32) - projectile collision radius
- Cooldown (f32) - time before next attack

**Operations**

`attack(position, direction)` - Fire projectile

- Spawn projectile at position
- Set velocity to direction * speed
- Start cooldown timer

`is_available()` - Check if can attack

- Return cooldown timer <= 0

**Invariants**

- Cannot attack during cooldown
- Projectile travels in straight line
- Despawns on first enemy hit (no pierce by default)
- Despawns on wall collision
- Despawns when lifetime expires

**Defaults**

| Field    | Value | Description       |
| -------- | ----- | ----------------- |
| Damage   | 15    | Per hit           |
| Speed    | 600.0 | Pixels per second |
| Lifetime | 2.0   | Seconds           |
| Size     | 8.0   | Pixels radius     |
| Cooldown | 0.2   | Seconds           |

---

## Bevy Integration

**Input**

- Right mouse button
- K key
- Gamepad button

**Spawning**

- Spawn Projectile entity with Collider
- Attach DamageSource component
- Set LinearVelocity for movement

**Events**

- AttackStarted { attacker, kind: Projectile, direction }
- AttackSpawned { entity, kind: Projectile, position, direction, team }
- HitEvent on collision with enemy

**Cleanup**

- Despawn on hit, wall collision, or lifetime expiry
