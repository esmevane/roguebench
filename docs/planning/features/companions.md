# Companions

Minimal framework for AI allies that follow and assist players, accessible to scripting and ability designers.

## Core Logic

**Concept**

- AI-controlled allies
- Follow and assist player
- Scripting defines specifics
- Minimal framework, extensible

**Companion Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Companion identifier |
| owner | entity | Player they follow |
| behavior | string | AI script |
| abilities | list | Available actions |
| persistent | bool | Survives between rooms |

**Basic Behaviors**

| Behavior | Description |
|----------|-------------|
| Follow | Stay near owner |
| Attack | Fight nearby enemies |
| Support | Heal/buff owner |
| Idle | Wait at position |
| Custom | Script-defined |

**Operations**

`summon_companion(definition, owner)` - Create companion

- Spawn entity
- Attach to owner
- Start behavior

`dismiss_companion(companion)` - Remove

- Despawn entity
- Clean up

`command_companion(companion, command)` - Direct

- Override current behavior
- Execute command

`get_companion(owner)` - Query

- Return owner's companion(s)

**Minimal AI**

- Follow owner at distance
- Attack enemies when owner attacks
- Return to owner when too far
- Scripts handle complex behavior

**Invariants**

- Companions belong to one owner
- Don't block owner movement
- Respect room boundaries
- State accessible to scripts

**Design Notes**

- Specific companions left to design
- Abilities left to design
- AI complexity left to scripts

---

## Bevy Integration

**Components**

- Companion { owner, behavior_script }
- CompanionAbilities { abilities: Vec<AbilityId> }

**Data**

- CompanionDefinition { id, sprite, behavior, abilities, ... }

**Messages/Commands**

- SummonCompanion { definition_id, owner }
- DismissCompanion { companion_id }
- CommandCompanion { companion_id, command }

**Events**

- CompanionSummoned { companion_id, owner }
- CompanionDismissed { companion_id }
- CompanionActed { companion_id, action }

**Systems**

- Handle summoning/dismissal
- Run companion AI
- Process commands
- Manage persistence

**AI Integration**

- Uses ai_state_machine or simpler
- Scripts can override states
- Responds to combat events

**Scripting Compatibility**

- Companion behavior is script-driven
- Abilities scriptable
- State accessible
- Events hookable

*See: architecture/scripting.md, ai_state_machine.md*
