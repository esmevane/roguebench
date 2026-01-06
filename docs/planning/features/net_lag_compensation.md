# Lag Compensation

Server-side hit validation accounting for client latency.

## Core Logic

**Problem**

- Client sees enemy at position A (due to latency)
- Client shoots at position A
- By server time, enemy is at position B
- Without compensation, shot misses unfairly

**Solution**

- Server stores position history
- When processing attack, rewind to client's view time
- Validate hit against historical positions
- Apply damage in present

**Operations**

`store_history(entity, position, tick)` - Record positions

- Store position with tick timestamp
- Maintain rolling buffer
- One entry per entity per tick

`rewind(tick)` - Get historical state

- Query positions at specific tick
- Return snapshot of world at that time

`validate_hit(attacker, target, attack_tick)` - Check hit

- Rewind to attacker's view time
- Check collision at that time
- Return hit/miss

`apply_hit(target)` - Apply damage now

- Damage applied at current tick
- Not historical tick

**View Time Calculation**

```
client_view_tick = current_tick - (client_rtt / 2) - interpolation_delay
```

**Invariants**

- Only rewind for hit validation
- Damage always applied in present
- Maximum rewind limit (prevent abuse)
- History buffer sized for max latency

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| History length | 1s | Max rewind window |
| Max rewind | 200ms | Abuse prevention |
| Granularity | Per tick | Position samples |

---

## Bevy Integration

**Components**

- PositionHistory { buffer: VecDeque<(Tick, Vec2)> }
- LagCompensated - marker for entities needing history

**Resources**

- LagCompensationConfig { max_rewind, history_length }

**Systems**

Server-side only:
- Record position history each tick
- Process attacks with rewind
- Validate collisions historically
- Apply results in present

**Attack Processing**

```rust
fn process_attack(attack: Attack, attacker_rtt: Duration) {
    let view_tick = calculate_view_tick(attacker_rtt);
    let historical_positions = rewind_to(view_tick);

    if check_hit(attack, historical_positions) {
        apply_damage(target, attack.damage);
    }
}
```

**Tradeoffs**

- Favors shooter (they hit what they saw)
- Target may feel hit "unfairly" (already moved)
- Industry standard for shooters
- Less critical for melee (shorter range)
