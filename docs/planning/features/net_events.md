# Network Events

Reliable delivery of important game events across the network.

## Core Logic

**Event Types**

| Event | Direction | Reliability | Description |
|-------|-----------|-------------|-------------|
| EntitySpawned | S→C | Reliable | New entity created |
| EntityDespawned | S→C | Reliable | Entity removed |
| DamageTaken | S→C | Reliable | Health change |
| EntityDied | S→C | Reliable | Death occurred |
| WaveStarted | S→C | Reliable | New wave begins |
| WaveCompleted | S→C | Reliable | Wave cleared |
| PlayerInput | C→S | Unreliable | Movement/actions |
| AttackFired | C→S | Reliable | Attack initiated |

**Reliable vs Unreliable**

- Reliable: guaranteed delivery, ordered, retransmitted
- Unreliable: fire and forget, may drop, newer overwrites

**Operations**

`send_reliable(event)` - Guaranteed delivery

- Queue for reliable channel
- Track until acknowledged
- Retransmit if needed

`send_unreliable(event)` - Best effort

- Send immediately
- No tracking or retry
- For frequent updates

`receive_events()` - Process incoming

- Handle reliable in order
- Handle unreliable by timestamp

**Event Sequencing**

- Reliable events have sequence numbers
- Process in order
- Buffer out-of-order until gap filled

**Invariants**

- Reliable events never lost
- Reliable events arrive in order
- Unreliable may be lost or reordered
- Events processed after state snapshot

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Reliable buffer | 64 | Pending reliable events |
| Retry interval | 100ms | Retransmit unacked |
| Max retries | 10 | Before disconnect |

---

## Bevy Integration

**Message Trait**

```rust
#[derive(Message, Serialize)]
struct EntitySpawned {
    network_id: NetworkId,
    entity_type: EntityType,
    position: Vec2,
}
```

**Channels**

- ReliableChannel - for events
- UnreliableChannel - for state/input

**Systems**

- Queue outgoing events
- Process incoming events
- Track reliable acknowledgments
- Trigger local event handlers

**Local Playback**

- Received events trigger local Bevy events
- Same handlers work for local and networked
- Example: EntityDied triggers death particles regardless of source

**Event Ordering**

- Apply state snapshot first
- Then process events for that tick
- Ensures consistency
