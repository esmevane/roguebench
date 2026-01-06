# Protocol Architecture

Constraints and patterns for network communication. **Lightyear handles most of this automatically.**

## Relationship to Lightyear

Lightyear provides the networking layer. This document describes:

1. What Lightyear handles (most things)
2. What we configure (protocol definition, channels)
3. Patterns for our custom messages
4. Constraints features should follow

```
┌─────────────────────────────────────────────────────────────┐
│  Lightyear (networking library)                             │
│  - Input replication (bevy_enhanced_input via lightyear_inputs_bei)
│  - Component replication (Replicate marker)                 │
│  - Tick synchronization (FixedMain schedule)                │
│  - Connection management                                    │
│  - Serialization (via serde)                                │
└─────────────────────────────────────────────────────────────┘
                              ↑
┌─────────────────────────────────────────────────────────────┐
│  Our Protocol Definition                                    │
│  - Which components to replicate                            │
│  - Input type definitions                                   │
│  - Custom message types                                     │
│  - Channel configuration                                    │
└─────────────────────────────────────────────────────────────┘
```

## Principles

1. **Let Lightyear handle it** - Don't build custom sync when Lightyear provides it
2. **Server authoritative** - Server runs simulation, clients receive state
3. **Tick-aligned** - All gameplay in FixedMain, tagged with tick number
4. **Bandwidth-conscious** - Use delta compression, prioritization
5. **Deterministic inputs** - Same inputs at same tick = same result

## What Lightyear Handles

| Concern | Lightyear Feature |
|---------|-------------------|
| Input replication | `lightyear_inputs_bei` (bevy_enhanced_input) |
| State sync | `Replicate` component marker |
| Entity mapping | Automatic server→client entity ID |
| Tick sync | `FixedMain` schedule integration |
| Interpolation | Built-in interpolation for smooth visuals |
| Prediction | Client-side prediction support |
| Lag compensation | Via lightyear_avian2d |

## What We Define

### Protocol Registration

```rust
// In glumglade-protocol crate
pub struct GlumgladeProtocol;

impl Protocol for GlumgladeProtocol {
    fn configure(registry: &mut ProtocolRegistry) {
        // Components to replicate
        registry.register_component::<Transform>();
        registry.register_component::<LinearVelocity>();
        registry.register_component::<Health>();
        registry.register_component::<PlayerState>();
        // ... etc
        
        // Input type
        registry.register_input::<PlayerInput>();
        
        // Custom messages
        registry.register_message::<ChatMessage>();
        registry.register_message::<SpawnParticle>();
    }
}
```

### Input Definition

```rust
// Input sent from client to server each tick
#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerInput {
    pub movement: Vec2,      // Direction from bevy_enhanced_input
    pub aim: Vec2,           // Aim direction
    pub actions: ActionBits, // Bitfield of pressed actions
}
```

### Custom Messages

For things that aren't component state or input:

```rust
// Server → Client: spawn particle effect
#[derive(Serialize, Deserialize)]
pub struct SpawnParticle {
    pub effect_id: String,
    pub position: Vec2,
}

// Ephemeral, doesn't need reliability
```

## Channels

Lightyear uses channels for different message types:

| Channel | Reliability | Use Case |
|---------|-------------|----------|
| Input | Unreliable | Per-tick player input |
| State | Unreliable | Component replication |
| Events | Reliable | Game events (damage, death) |
| Commands | Reliable | Actions requiring confirmation |

Configure in protocol:

```rust
registry.add_channel::<UnreliableChannel>(/* config */);
registry.add_channel::<ReliableChannel>(/* config */);
```

## Bandwidth Optimization

**Delta compression:** Lightyear supports this. Enable for frequently-changing components.

**Prioritization:** Lightyear can prioritize entities by relevance (distance, importance).

**Interest management:** Use Lightyear's visibility system to not replicate distant entities.

## Tick Alignment

Lightyear enforces tick-based execution:

- Server runs simulation in `FixedMain` at configured tick rate (e.g., 60 ticks/sec)
- Inputs are tagged with the tick they were generated
- State snapshots are tagged with tick number
- Client interpolates between received states

Use Lightyear's `GameTick` resource for all timing:

```rust
fn my_system(tick: Res<GameTick>) {
    let current_tick: u64 = tick.0;
    // Use tick for timing, not Time::delta_secs()
}
```

## Constraints for Features

Features that involve networking should:

1. **Use replicated components** for state that clients need
2. **Use Lightyear input** for player actions (not custom messages)
3. **Use command bus** for game events (damage, spawns) — these run on server
4. **Use custom messages** only for non-state data (particles, sounds)
5. **Tag time-sensitive data** with tick number
6. **Handle prediction** if relevant (player movement predicted locally)
7. **Document what's replicated** vs what's server-only vs client-only

## What's Not Here

- Custom serialization (use serde)
- Custom transport (Lightyear handles UDP/WebRTC)
- Protocol versioning (Lightyear handles)
- Handshake (Lightyear handles)

*See: framework/command_bus.md (game events), framework/spawn_framework.md (entity replication)*
