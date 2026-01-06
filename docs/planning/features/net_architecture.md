# Network Architecture

Core networking model defining client/server roles, tick rates, and protocol choices.

## Core Logic

**Model**

- Server-authoritative: server is source of truth
- Clients send inputs, receive state
- Server validates all actions

**Tick Rate**

- Server simulation: fixed timestep (e.g., 60hz)
- Network send rate: may differ (e.g., 20-30hz)
- Client render: decoupled, interpolated

**Roles**

| Role | Responsibilities |
|------|------------------|
| Server | Simulation, validation, broadcast, AI, spawning |
| Client | Input capture, prediction, rendering, interpolation |
| Host | Server + Client combined (listen server) |

**Protocol**

- UDP-based for low latency
- Reliable channel for critical events (deaths, spawns)
- Unreliable channel for frequent updates (positions)

**Operations**

`server_tick()` - Run one server frame

- Process buffered client inputs
- Step simulation
- Broadcast state snapshot

`client_tick()` - Run one client frame

- Send buffered inputs
- Apply prediction
- Receive and reconcile state

**Invariants**

- Server state is canonical
- Clients never directly modify authoritative state
- All gameplay logic runs on server
- Clients may run duplicate logic for prediction only

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Tick rate | 60hz | Server simulation |
| Send rate | 20hz | Network updates |
| Protocol | QUIC/UDP | Via quinn or similar |

---

## Bevy Integration

**Scheduling**

- Server: FixedUpdate for simulation
- Client: FixedUpdate for prediction, Update for rendering
- Separate schedules for network send/receive

**Resources**

- NetworkRole { Server, Client, Host }
- TickRate { simulation_hz, network_hz }
- NetworkStats { rtt, packet_loss, jitter }

**Plugins**

- ServerPlugin - runs authoritative simulation
- ClientPlugin - runs prediction and rendering
- SharedPlugin - common types and systems

**Conditional Compilation**

- Feature flags: `server`, `client`, `host`
- Or runtime role selection
