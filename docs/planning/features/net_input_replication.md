# Input Replication

Sending client inputs to server with timestamps for deterministic processing.

## Core Logic

**Input Packet**

- Tick number (when input was generated)
- Input state (movement, actions)
- Sequence number (for ordering)

**Input Types**

| Input | Data | Frequency |
|-------|------|-----------|
| Movement | Vec2 direction | Every tick |
| Attack | Attack type, aim direction | On press |
| Dash | Direction | On press |
| Interact | None | On press |

**Operations**

`capture_input()` - Record local input

- Sample input devices
- Timestamp with current tick
- Add to send buffer

`send_inputs()` - Transmit to server

- Bundle recent inputs (redundancy)
- Include last N inputs for packet loss
- Send unreliably (frequent)

`receive_inputs(client)` - Server receives

- Buffer by tick number
- Handle out-of-order
- Discard too-old inputs

`apply_input(tick, input)` - Server processes

- Apply to simulation at correct tick
- Validate input is legal
- Update authoritative state

**Invariants**

- Inputs timestamped at generation, not send time
- Redundant sending handles packet loss
- Server processes inputs in tick order
- Invalid inputs rejected, not corrected

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Redundancy | 3 | Inputs sent per packet |
| Buffer size | 60 | Ticks of input history |
| Max input age | 500ms | Reject older inputs |

---

## Bevy Integration

**Components**

- InputBuffer { inputs: VecDeque<TimestampedInput> }
- LastProcessedTick(u64)

**Resources**

- LocalInputBuffer
- ClientInputBuffers (server-side, per client)

**Messages**

- InputPacket { tick, inputs, sequence }

**Systems**

Client:
- Capture input each FixedUpdate
- Send input packet each network tick
- Store for prediction

Server:
- Receive and buffer inputs
- Apply inputs during simulation tick
- Track last processed tick per client

**Serialization**

- Compact binary format
- Delta compression for movement
- Bitflags for actions
