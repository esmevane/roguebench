# Player Melee Attack

Close-range attack that damages enemies in an arc in front of player. Has configurable damage, range, and cooldown.

## Core Logic

**Parameters**

- Damage (i32) - damage dealt on hit
- Range (f32) - attack reach distance
- Arc (f32) - attack width in degrees
- Duration (f32) - how long hitbox is active
- Cooldown (f32) - time before next attack
- Knockback (f32) - force applied to hit targets

**Operations**

`attack(direction)` - Perform melee attack

- Spawn hitbox in direction
- Hitbox active for duration
- Start cooldown timer

`is_available()` - Check if can attack

- Return cooldown timer <= 0

**Invariants**

- Cannot attack during cooldown
- Hitbox follows player position
- Each enemy hit only once per attack
- Attack direction locked at start

**Defaults**

| Field     | Value | Description       |
| --------- | ----- | ----------------- |
| Damage    | 25    | Per hit           |
| Range     | 50.0  | Pixels            |
| Arc       | 90.0  | Degrees           |
| Duration  | 0.15  | Seconds           |
| Cooldown  | 0.3   | Seconds           |
| Knockback | 200.0 | Force units       |

---

## Bevy Integration

**Input**

- Left mouse button
- J key
- Gamepad button

**Spawning**

- Spawn ActiveMelee entity with Collider
- Attach DamageSource and HitTracker
- Parent to player or track player position

**Events**

- AttackStarted { attacker, kind: Melee, direction }
- AttackSpawned { entity, kind: Melee, position, direction, team }
- HitEvent on collision with enemy

**Cleanup**

- Despawn when duration expires
