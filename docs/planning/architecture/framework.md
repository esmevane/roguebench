# Framework Architecture

Approach for building foundational systems before feature implementation.

## Hierarchy

Our specs exist in three layers:

```
┌─────────────────────────────────────────────────┐
│  ARCHITECTURE (constraints)                     │
│  - data.md, scripting.md, protocol.md, etc.    │
│  - Patterns all code must follow               │
│  - No implementation, just constraints         │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│  FRAMEWORK (reusable systems)                   │
│  - command_bus.md, entity_id.md, etc.          │
│  - Implement architecture constraints          │
│  - Enable features to be built consistently    │
│  - Built BEFORE features                       │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│  FEATURES (concrete functionality)              │
│  - player_dash.md, enemy_grunt.md, etc.        │
│  - Built ON TOP of frameworks                  │
│  - Reference frameworks they depend on         │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│  CONTENT (design-defined)                       │
│  - Specific items, enemies, quests             │
│  - Not in specs - created by designers         │
│  - Uses frameworks and features                │
└─────────────────────────────────────────────────┘
```

## Framework Principles

1. **Build once, use everywhere** - Frameworks solve common problems
2. **Architecture-compliant** - Frameworks implement architecture constraints
3. **Feature-agnostic** - Frameworks don't know about specific features
4. **Extensible** - Designers/features can extend frameworks
5. **Tested independently** - Frameworks have their own tests

## Framework Categories

### Core Infrastructure
Fundamental systems everything depends on:
- Command/message bus
- Entity identity
- Data loading
- State machines
- Event hooks

### Game Systems
Reusable game logic patterns:
- Stats and modifiers
- Effects and behaviors
- Timers and cooldowns
- Spawning and pooling

### Presentation
Rendering and feedback:
- Animation framework
- Particle framework
- Audio framework
- UI framework

### Physics
Collision and movement:
- Collision framework
- Movement framework

## Implementation Order

See the development roadmap for detailed phasing. Summary:

1. **Phase 1: Core Infrastructure**
   - Lightyear + Avian 2D integration (networking + physics foundation)
   - Command bus (enables game events, scripting)
   - Data loading (enables content)
   - State machine (enables AI, character states)

2. **Phase 2: Game Systems**
   - Timer framework (tick-based)
   - Spawn framework
   - Stats framework (when combat features need it)
   - Effect framework (when combat features need it)

3. **Phase 3: Presentation**
   - Animation framework
   - Particle framework
   - Audio framework

4. **Phase 4: Features**
   - Built on solid foundation

**Note:** Entity identity framework is deferred until save/load implementation—Lightyear handles network entity mapping. Stats and effect frameworks will be specified when first feature requires them.

## Feature → Framework Relationship

Features declare framework dependencies:

```markdown
# Some Feature

## Framework Dependencies
- `framework/command_bus.md` - For game events
- `framework/timer_framework.md` - For cooldowns
- `framework/state_machine.md` - For behavior states
```

## Benefits

- **Consistency**: All features work the same way
- **Scripting**: Works automatically for all features
- **Networking**: Data patterns already network-safe
- **Testing**: Frameworks tested independently
- **Onboarding**: Clear patterns for new developers
