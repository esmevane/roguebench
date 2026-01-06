# Client Prediction

Local simulation of player actions for responsive gameplay despite network latency.

## Core Logic

**Concept**

- Client simulates own player locally
- Don't wait for server confirmation
- Correct later if server disagrees

**Predicted State**

| State | Predicted | Why |
|-------|-----------|-----|
| Player position | Yes | Movement feels instant |
| Player velocity | Yes | Smooth movement |
| Dash state | Yes | Responsive ability |
| Attack state | Yes | Immediate feedback |
| Enemy position | No | Server authoritative |
| Health | No | Server authoritative |

**Operations**

`predict(input)` - Apply input locally

- Run same logic as server
- Store predicted state with tick
- Render immediately

`store_state(tick)` - Save for reconciliation

- Record predicted state
- Associate with input tick
- Keep history buffer

`get_predicted_position()` - Query current

- Return locally predicted value
- Used for rendering own player

**Invariants**

- Prediction uses same logic as server
- Only predict own player (not others)
- Keep history for reconciliation window
- Visual feedback immediate, even if wrong

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| History size | 64 | Ticks of state history |
| Max prediction | 200ms | Limit prediction window |

---

## Bevy Integration

**Components**

- Predicted - marker for predicted entities
- PredictedState { position, velocity, ... }
- StateHistory { states: VecDeque<(Tick, State)> }

**Resources**

- PredictionConfig { enabled, max_ticks }

**Systems**

- Apply local input to predicted state
- Store state snapshot each tick
- Use predicted position for local player rendering
- Feed corrections to reconciliation

**Separation**

- Predicted state: what client thinks
- Authoritative state: what server says
- Rendered state: what player sees (may blend)

**Local vs Remote**

```rust
// Local player: use predicted position
// Remote player: use interpolated position
fn get_render_position(entity) -> Vec2 {
    if is_local_player(entity) {
        predicted_position(entity)
    } else {
        interpolated_position(entity)
    }
}
```
