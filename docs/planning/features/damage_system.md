# Damage System

Processes damage from attacks to health. Handles damage sources, amounts, and effects.

## Core Logic

**Damage Source**

- Damage (i32) - base damage amount
- Source entity (optional) - who dealt the damage
- Pierce (bool) - whether attack continues after hit

**Operations**

`apply(target, source, amount)` - Deal damage

- Check target has Health
- Check target not invincible
- Subtract amount from target health
- Grant invincibility frames
- Emit damage event

**Invariants**

- Damage cannot be negative
- Invincible entities ignore damage
- Dead entities cannot take damage
- Source used for knockback direction

**Damage Flow**

1. Attack collides with target
2. HitEvent emitted
3. DamageEvent created
4. Damage applied to Health
5. Invincibility granted
6. Effects triggered (flash, shake, knockback)

---

## Bevy Integration

**Components**

- DamageSource { damage: i32, pierce: bool }
- HitTracker { hit_entities: HashSet<Entity> }

**Events**

- HitEvent { source, target } - collision detected
- DamageEvent { target, amount, source } - damage to apply

**Systems**

- Collision detection emits HitEvent
- Hit processing emits DamageEvent
- Damage application modifies Health

**Filtering**

- Team system prevents friendly fire
- HitTracker prevents multi-hit on same target

---

## Framework Dependencies

- `framework/command_bus.md` - Damage as command for networking/scripting
- `framework/event_hooks.md` - OnDamage hooks for scripted reactions
- `framework/collision_framework.md` - Hit detection triggering damage
- `features/stat_system.md` - Damage modifiers from stats
- `features/effect_framework.md` - Status effects that modify damage

*See: architecture/scripting.md, architecture/protocol.md*
