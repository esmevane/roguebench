# Weapons

Configurable attack definitions that determine damage, range, and behavior. Entities have weapon inventories.

## Core Logic

**Weapon Types**

- Melee - close range arc attack
- Ranged - projectile attack

**Melee Config**

- Damage (i32)
- Range (f32) - attack reach
- Arc (f32) - attack width in degrees
- Cooldown (f32)
- Knockback (f32)
- Duration (f32) - hitbox active time

**Ranged Config**

- Damage (i32)
- Speed (f32) - projectile speed
- Lifetime (f32) - max travel time
- Cooldown (f32)
- Size (f32) - projectile collision size

**Operations**

`get_melee()` - Get melee weapon config
`get_ranged()` - Get ranged weapon config
`can_attack_melee()` - Check melee available
`can_attack_ranged()` - Check ranged available

**Invariants**

- Entity can have melee, ranged, or both
- Each weapon has independent cooldown
- Weapon stats defined in config files

---

## Bevy Integration

**Components**

- WeaponInventory { melee: Option<MeleeWeapon>, ranged: Option<RangedWeapon> }
- MeleeWeapon { damage, range, arc, cooldown, knockback, duration }
- RangedWeapon { damage, speed, lifetime, cooldown, size }

**Config**

- Player weapons in player.ron
- Enemy weapons in enemies.ron per enemy type

**Spawning**

- Attack systems read weapon config
- Spawn appropriate attack entity with weapon stats
