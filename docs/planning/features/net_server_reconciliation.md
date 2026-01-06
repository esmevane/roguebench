# Server Reconciliation

Correcting client prediction errors when server state differs from predicted state.

## Core Logic

**Concept**

- Client predicts, server confirms
- When server disagrees, client corrects
- Re-simulate from correction point

**Process**

1. Receive server snapshot for tick N
2. Compare server state to predicted state at tick N
3. If different beyond threshold, reconcile
4. Reset to server state at tick N
5. Re-apply inputs from tick N+1 to current

**Operations**

`check_divergence(server_state, predicted_state)` - Compare

- Calculate position difference
- Return true if beyond threshold

`reconcile(server_tick, server_state)` - Correct prediction

- Reset local state to server state
- Replay all inputs since server_tick
- Update prediction history

`replay_inputs(from_tick, to_tick)` - Re-simulate

- Apply stored inputs in order
- Run prediction logic for each
- Arrive at corrected current state

**Thresholds**

| Value | Threshold | Action |
|-------|-----------|--------|
| Position | 0.1 units | Reconcile |
| Velocity | 1.0 units/s | Reconcile |
| State mismatch | Any | Reconcile |

**Invariants**

- Server state always wins
- Reconciliation is invisible if prediction correct
- Replay uses stored inputs, not re-captured
- Frequent reconciliation indicates prediction bug

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Position threshold | 0.1 | Units before correction |
| Max replay ticks | 30 | Limit re-simulation |
| Smoothing | 0.1s | Blend correction visually |

---

## Bevy Integration

**Resources**

- ReconciliationConfig { thresholds, smoothing }
- ReconciliationStats { corrections_per_second }

**Systems**

- Compare on snapshot receive
- Trigger reconciliation if diverged
- Replay input buffer
- Optionally smooth visual correction

**Visual Smoothing**

- Snap gameplay state immediately
- Blend rendered position over short duration
- Prevents jarring teleports

**Debugging**

- Log reconciliation frequency
- Visualize server vs predicted positions
- Alert if reconciling too often (prediction bug)
