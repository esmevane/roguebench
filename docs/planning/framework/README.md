# Frameworks

Reusable systems built before features. See `architecture/framework.md` for the overall approach.

## Core Infrastructure

Fundamental systems everything depends on:

| Framework | Purpose |
|-----------|---------|
| [command_bus](command_bus.md) | Message routing for scripting, networking, undo |
| [entity_id](entity_id.md) | Stable entity identification for persistence |
| [data_loading](data_loading.md) | Content loading and hot reload |
| [state_machine](state_machine.md) | Generic hierarchical state machine |
| [event_hooks](event_hooks.md) | Extensible event system for scripting |

## Game Systems

Reusable game logic patterns:

| Framework | Purpose |
|-----------|---------|
| [timer_framework](timer_framework.md) | Cooldowns, durations, periodic triggers |
| [spawn_framework](spawn_framework.md) | Entity spawning and lifecycle |

**Note**: `stat_system.md` and `effect_framework.md` in features/ are framework-level specs that will be referenced by other features.

## Presentation

Rendering and feedback:

| Framework | Purpose |
|-----------|---------|
| [animation_framework](animation_framework.md) | Sprite animation and state-driven playback |
| [particle_framework](particle_framework.md) | Particle effect spawning and lifecycle |
| [audio_framework](audio_framework.md) | Sound playback, music, spatial audio |
| [ui_framework](ui_framework.md) | User interface layout and widgets |

## Physics

Collision and movement:

| Framework | Purpose |
|-----------|---------|
| [collision_framework](collision_framework.md) | Collision detection and resolution |
| [movement_framework](movement_framework.md) | Entity velocity and movement processing |

## Implementation Order

1. **Phase 1: Core Infrastructure** - command_bus, entity_id, data_loading, state_machine
2. **Phase 2: Game Systems** - timer, spawn, (stat_system, effect_framework from features/)
3. **Phase 3: Presentation** - animation, particle, audio, ui
4. **Phase 4: Physics** - collision, movement
5. **Phase 5: Features** - build on solid foundation

## Usage Pattern

Features declare framework dependencies:

```markdown
## Framework Dependencies
- `framework/command_bus.md` - For scripted triggers
- `framework/state_machine.md` - For AI behavior
- `framework/collision_framework.md` - For hit detection
```
