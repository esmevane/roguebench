# Stat System

Base stats, modifiers, and derived value calculation for entities.

## Core Logic

**Relationship to Effect Framework**

- `stat_system.md` (this): Defines stats and modifier calculation
- `effect_framework.md`: Applies timed effects that include stat modifiers

Effects apply stat modifiers; the stat system calculates final values. Both are frameworks - specific stats and effects are design-defined.

**Concept**

- Entities have base stats
- Modifiers alter stats (equipment, effects, buffs)
- Final values calculated from base + modifiers
- Framework for any stat types

**Stat Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Stat identifier |
| base_value | float | Unmodified value |
| modifiers | list | Active modifiers |
| final_value | float | Calculated result |
| min | option | Minimum clamp |
| max | option | Maximum clamp |

**Modifier Types**

| Type | Calculation | Example |
|------|-------------|---------|
| Flat | +/- value | +10 health |
| Percent | ×(1 + percent) | +25% damage |
| Override | = value | Set to 100 |

**Modifier Properties**

| Property | Type | Description |
|----------|------|-------------|
| stat | string | Which stat |
| modifier_type | enum | Flat, Percent, Override |
| value | float | Modifier amount |
| source | string | Where it came from |
| priority | int | Application order |

**Calculation Order**

1. Start with base value
2. Apply flat modifiers (sum)
3. Apply percent modifiers (multiply)
4. Apply overrides (last wins)
5. Clamp to min/max

**Operations**

`get_stat(entity, stat_id)` - Get final value

- Calculate from base + modifiers
- Cache if expensive

`set_base(entity, stat_id, value)` - Set base

- Update base value
- Recalculate final

`add_modifier(entity, modifier)` - Apply modifier

- Add to modifier list
- Recalculate final

`remove_modifier(entity, source)` - Remove modifier

- Remove by source
- Recalculate final

`list_modifiers(entity, stat_id)` - Show breakdown

- For UI display
- Debug info

**Invariants**

- Final value always valid (clamped)
- Modifiers stack correctly by type
- Removal by source cleans all from that source
- Calculation deterministic

**Design Notes**

- Specific stats left to design
- Modifier sources left to design
- Balance left to design

---

## Bevy Integration

**Components**

- Stats { values: HashMap<StatId, StatValue> }
- StatModifiers { modifiers: Vec<StatModifier> }

**Calculation**

```rust
fn calculate_stat(base: f32, modifiers: &[StatModifier]) -> f32 {
    let flat: f32 = modifiers.iter()
        .filter(|m| m.modifier_type == Flat)
        .map(|m| m.value).sum();

    let percent: f32 = modifiers.iter()
        .filter(|m| m.modifier_type == Percent)
        .map(|m| m.value).sum();

    (base + flat) * (1.0 + percent)
}
```

**Messages/Commands**

- SetBaseStat { entity, stat_id, value }
- AddStatModifier { entity, modifier }
- RemoveStatModifiers { entity, source }

**Events**

- StatChanged { entity, stat_id, old_value, new_value }

**Systems**

- Recalculate on modifier change
- Emit events on stat change
- Cache final values

**Scripting Compatibility**

- Stat operations as commands
- Stats readable
- Events hookable

*See: architecture/scripting.md, architecture/data.md*
