# Effect Framework

System for applying timed effects, stat modifiers, and triggered behaviors to entities.

## Core Logic

**Relationship to Stat System**

- `stat_system.md`: Defines stats and modifier calculation
- `effect_framework.md` (this): Applies timed effects that include stat modifiers

Effects can include stat modifiers (using stat_system) AND triggered behaviors. This framework manages timing, stacking, and application; specific effects are design-defined.

**Concept**

- Effects are temporary modifications
- Can modify stats, trigger behaviors, or both
- Duration, stacking, and removal managed
- Framework enables designer-defined effects

**Effect Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Effect identifier |
| duration | enum | Timed, Permanent, Triggered |
| stacking | enum | None, Intensity, Duration, Independent |
| modifiers | list | Stat modifications |
| behaviors | list | Triggered behaviors |
| tags | list | Effect categories |
| icon | asset | Visual indicator |

**Duration Types**

| Type | Behavior |
|------|----------|
| Timed | Expires after duration |
| Permanent | Until explicitly removed |
| Triggered | Removed by condition |
| Charges | Removed after N triggers |

**Stacking Modes**

| Mode | Behavior |
|------|----------|
| None | Refresh duration only |
| Intensity | Stack effects multiply |
| Duration | Stack extends time |
| Independent | Each application separate |

**Behaviors**

| Trigger | Action | Example |
|---------|--------|---------|
| OnTick | Periodic | Poison damage |
| OnHit | When damaged | Thorns reflect |
| OnAttack | When attacking | Lifesteal |
| OnMove | When moving | Leave trail |
| OnDeath | When dying | Explode |

**Operations**

`apply_effect(entity, effect_id, source)` - Add effect

- Check stacking rules
- Add to entity
- Apply immediate effects

`remove_effect(entity, effect_id)` - Remove effect

- Remove modifiers
- Stop behaviors
- Cleanup

`tick_effects(entity, dt)` - Update timed

- Reduce durations
- Trigger periodic behaviors
- Remove expired

`has_effect(entity, effect_id)` - Check active

- Return if effect present

`get_effects_by_tag(entity, tag)` - Query by tag

- Return effects with tag

**Invariants**

- Stacking rules respected
- Removal cleans up completely
- Behaviors fire correctly
- Effects serializable for networking

**Design Notes**

- Specific effects left to design
- Behavior implementations left to design
- Consider existing Bevy libraries

---

## Bevy Integration

**Components**

- ActiveEffects { effects: Vec<AppliedEffect> }
- AppliedEffect { definition_id, source, remaining_duration, stacks }

**Data**

- EffectDefinition { id, duration_type, stacking, modifiers, behaviors, tags }

**Messages/Commands**

- ApplyEffect { entity, effect_id, source }
- RemoveEffect { entity, effect_id }
- ClearEffects { entity, tag } (optional: by tag)

**Events**

- EffectApplied { entity, effect_id }
- EffectRemoved { entity, effect_id }
- EffectTicked { entity, effect_id }
- EffectExpired { entity, effect_id }

**Systems**

- Tick effect durations
- Execute periodic behaviors
- Remove expired effects
- Apply/remove stat modifiers

**Behavior Execution**

```rust
fn execute_behavior(trigger: Trigger, entity: Entity, world: &mut World) {
    for effect in entity.get::<ActiveEffects>() {
        for behavior in effect.behaviors {
            if behavior.trigger == trigger {
                behavior.execute(entity, world);
            }
        }
    }
}
```

**Scripting Compatibility**

- Effect application as commands
- Effects readable
- Events hookable
- Behaviors scriptable

*See: architecture/scripting.md, architecture/data.md*
