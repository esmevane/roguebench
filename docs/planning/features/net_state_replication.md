# State Replication

Server broadcasting authoritative state to clients. Defines what replicates and how.

## Core Logic

**Replicated Data**

| Component | Priority | Frequency | Notes |
|-----------|----------|-----------|-------|
| Position | High | Every snapshot | All entities |
| Velocity | Medium | Every snapshot | For interpolation |
| Health | Medium | On change | Players, enemies |
| Animation state | Low | On change | Visual only |
| Wave state | Low | On change | Global |

**Snapshot**

- Server tick number
- Entity states (position, velocity, etc.)
- Events since last snapshot

**Operations**

`create_snapshot()` - Server builds snapshot

- Gather all replicated components
- Delta compress against last acked
- Prioritize by relevance

`send_snapshot(client)` - Transmit state

- Full or delta snapshot
- Include server tick
- Unreliable (frequent)

`receive_snapshot()` - Client receives

- Buffer by tick number
- Apply to interpolation buffer
- Trigger reconciliation if needed

**Relevancy**

- Distance-based: nearby entities higher priority
- Team-based: teammates always relevant
- Event-based: damage sources always sent

**Invariants**

- Server tick always included
- Clients never ahead of server
- Missing snapshots handled gracefully
- Delta encoding reduces bandwidth

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Snapshot rate | 20hz | Network send rate |
| Delta baseline | 3 | Snapshots for delta |
| Max entities | 256 | Per snapshot |

---

## Bevy Integration

**Components**

- Replicated - marker for replicated entities
- ReplicationPriority(u8)
- NetworkId(u64) - stable across network

**Resources**

- SnapshotBuffer { snapshots: VecDeque }
- LastAckedSnapshot(u64)

**Messages**

- StateSnapshot { tick, entities, events }
- SnapshotAck { tick }

**Systems**

Server:
- Build snapshot each network tick
- Track client ack for delta
- Send to all clients

Client:
- Receive and buffer snapshots
- Ack received snapshots
- Feed to interpolation system

**Attributes**

```rust
#[derive(Component, Replicate)]
struct Position(Vec2);
```

Or manual registration of replicated types.
