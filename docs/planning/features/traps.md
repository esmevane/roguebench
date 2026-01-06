# Traps

Minimal framework for environmental hazards accessible to level design and scripting.

## Core Logic

**Concept**

- Environmental hazards
- Triggered by player/enemy presence
- Scripting defines behavior
- Integrates with tile/room systems

**Trap Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Trap identifier |
| trigger | enum | What activates it |
| effect | string | Script/effect to run |
| rearm | bool | Reactivates after trigger |
| visible | bool | Player can see trap |
| team | enum | Who it affects |

**Trigger Types**

| Trigger | Activation |
|---------|------------|
| Proximity | Enter area |
| Step | Stand on tile |
| Timed | Periodic activation |
| Remote | Script-triggered |
| Destroyed | When trap destroyed |

**Basic Effects**

| Effect | Description |
|--------|-------------|
| Damage | Deal damage |
| Status | Apply effect |
| Spawn | Create entities |
| Push | Knockback |
| Teleport | Move player |
| Script | Custom behavior |

**Operations**

`spawn_trap(definition, position)` - Place trap

- Create trap entity
- Configure trigger

`trigger_trap(trap)` - Activate

- Run effect
- Handle rearm

`disarm_trap(trap)` - Disable

- Prevent triggering
- May require skill/item

`reveal_trap(trap)` - Make visible

- Show hidden trap
- From perception or ability

**Invariants**

- Traps respect team alignment
- Effects use existing systems
- Visibility respects detection
- Scripts handle complex behavior

**Design Notes**

- Specific traps left to design
- Detection mechanics left to design
- Visual indicators left to design

---

## Bevy Integration

**Components**

- Trap { trigger_type, effect, rearm, visible, team }
- TrapTrigger - collision sensor
- TrapArmed(bool)

**Data**

- TrapDefinition { id, trigger, effect, ... }

**Messages/Commands**

- SpawnTrap { definition_id, position }
- TriggerTrap { trap_id }
- DisarmTrap { trap_id }
- RevealTrap { trap_id }

**Events**

- TrapTriggered { trap_id, victim }
- TrapDisarmed { trap_id }
- TrapRevealed { trap_id }

**Systems**

- Detect trigger conditions
- Execute trap effects
- Handle rearm timing
- Manage visibility

**Integration Points**

- `damage_system.md` - damage effects
- `effect_framework.md` - status effects
- `tile_types.md` - tile-based traps
- `collision_layers.md` - trigger detection

**Scripting Compatibility**

- Trap behavior scriptable
- Effects scriptable
- Events hookable
- Spawnable from scripts

*See: architecture/scripting.md, tile_types.md*
