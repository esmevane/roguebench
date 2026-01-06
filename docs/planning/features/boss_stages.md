# Boss Stages

Multi-phase boss encounters with stage transitions and unique mechanics.

## Core Logic

**Concept**

- Bosses have multiple phases
- Phases triggered by health thresholds
- Each phase has different behavior
- Transition moments are dramatic

**Stage Properties**

| Property | Type | Description |
|----------|------|-------------|
| index | int | Stage number (1, 2, 3) |
| health_threshold | float | Percent to trigger |
| behavior | string | AI/attack pattern |
| abilities | list | Available in this stage |
| transition | string | Transition event |

**Typical 3-Stage Boss**

| Stage | Health | Pattern |
|-------|--------|---------|
| 1 | 100-66% | Basic attacks, learning phase |
| 2 | 66-33% | New abilities, increased aggression |
| 3 | 33-0% | Desperate attacks, enrage |

**Transition Events**

| Event | Description |
|-------|-------------|
| Invulnerable | Brief immunity during transition |
| Animation | Dramatic transformation |
| Adds | Spawn minions |
| Arena change | Modify environment |
| Heal | Partial health restore (optional) |

**Operations**

`check_stage_transition(boss)` - Evaluate

- Compare health to thresholds
- Trigger if crossed

`transition_stage(boss, new_stage)` - Change phase

- Play transition event
- Update behavior
- Unlock new abilities

`get_current_stage(boss)` - Query

- Return current stage index

**Invariants**

- Stages progress in order
- Health can only decrease (no backwards)
- Transitions complete before resuming
- Each stage reached once

**Design Notes**

- Specific bosses left to design
- Stage abilities left to design
- 2-3 stages typical, more possible

---

## Bevy Integration

**Components**

- Boss { stages: Vec<BossStage>, current_stage: usize }
- BossStage { health_threshold, behavior, abilities, transition }
- BossTransitioning(Timer) - during transition

**Data**

- BossDefinition { id, stages, base_health, ... }

**Messages/Commands**

- TriggerBossTransition { boss_id, stage }

**Events**

- BossStageTransition { boss_id, from_stage, to_stage }
- BossEnraged { boss_id } - final stage
- BossDefeated { boss_id }

**Systems**

- Monitor boss health
- Trigger transitions at thresholds
- Handle transition state (invuln, animation)
- Switch behavior/abilities per stage

**Transition Handling**

```
Health crosses threshold
→ Boss becomes invulnerable
→ Transition animation plays
→ Arena/environment updates
→ New stage behavior activates
→ Boss becomes vulnerable
```

**Integration Points**

- `ai_state_machine.md` - stage behaviors
- `health_system.md` - threshold monitoring
- `story_events.md` - transition cinematics

**Scripting Compatibility**

- Stages definable in data
- Transitions scriptable
- Events hookable

*See: architecture/scripting.md, architecture/editor.md*
