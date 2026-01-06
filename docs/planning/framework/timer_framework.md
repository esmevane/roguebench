# Timer Framework

Cooldowns, durations, and periodic triggers using tick-based timing.

## Relationship to Networking

Timers for gameplay logic run on the **server** in `FixedMain` schedule, synchronized with Lightyear ticks. This ensures deterministic behavior across server and clients.

**Timer categories by authority:**

| Category | Runs On | Timing | Example |
|----------|---------|--------|---------|
| Gameplay | Server | Tick-based | Attack cooldown, buff duration |
| Visual | Client | Frame-based | UI animation, particle lifetime |

Clients receive timer state via component replication. They don't run their own gameplay timers—they display what the server tells them.

---

## Core Logic

**Concept**

- Many systems need time-based behavior
- Cooldowns prevent repeated actions
- Durations track temporary states
- Periodic triggers fire at intervals
- All gameplay timers use tick counts, not seconds

**Timer Types**

| Type | Behavior | Example |
|------|----------|---------|
| Cooldown | Prevents action until elapsed | Attack cooldown |
| Duration | Tracks ticks remaining | Buff duration |
| Periodic | Fires repeatedly | Poison tick |
| Delay | One-shot after delay | Spawn delay |

**Timer Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Timer identifier |
| timer_type | enum | Cooldown, Duration, Periodic, Delay |
| duration_ticks | u64 | Total duration in ticks |
| remaining_ticks | u64 | Ticks left |
| paused | bool | Currently ticking |

**Operations**

`start(entity, timer_id, duration_ticks)` - Begin timer

- Initialize timer with tick count
- Start ticking each FixedMain

`pause(entity, timer_id)` - Stop ticking

- Preserve remaining ticks

`resume(entity, timer_id)` - Continue ticking

- Resume from paused state

`reset(entity, timer_id)` - Restart timer

- Set remaining to duration

`cancel(entity, timer_id)` - Remove timer

- Stop and remove

`is_ready(entity, timer_id)` - Check cooldown

- True if elapsed or not present

`get_remaining(entity, timer_id)` - Ticks left

- For UI display (convert to seconds client-side)

`get_progress(entity, timer_id)` - Progress 0-1

- For progress bars

**Invariants**

- Gameplay timers tick once per FixedMain tick
- Same tick count produces same behavior (deterministic)
- Paused timers don't tick
- Expired timers fire events
- Periodic timers auto-restart

---

## Tick-Based Timing

Gameplay timers count **ticks**, not seconds. This ensures determinism:

```rust
// Duration of 60 ticks at 60 ticks/sec = 1 second
// But it's ALWAYS 60 ticks, regardless of frame rate variance
const ATTACK_COOLDOWN_TICKS: u64 = 30; // 0.5 seconds at 60 ticks/sec
```

**Converting for display:**

```rust
fn ticks_to_seconds(ticks: u64, tick_rate: f32) -> f32 {
    ticks as f32 / tick_rate
}
```

Clients display remaining time using this conversion, but the authoritative countdown is tick-based on the server.

---

## Bevy Integration

**Components**

```rust
#[derive(Component)]
struct Timers {
    active: HashMap<TimerId, Timer>,
}

struct Timer {
    timer_type: TimerType,
    duration_ticks: u64,
    remaining_ticks: u64,
    paused: bool,
}

enum TimerType {
    Cooldown,
    Duration,
    Periodic,
    Delay,
}
```

**Systems (run in FixedMain)**

```rust
fn tick_timers(
    mut query: Query<(Entity, &mut Timers)>,
    mut events: EventWriter<TimerEvent>,
) {
    // Called once per tick in FixedMain
    for (entity, mut timers) in query.iter_mut() {
        let mut expired = Vec::new();
        
        for (id, timer) in timers.active.iter_mut() {
            if timer.paused { continue; }

            if timer.remaining_ticks > 0 {
                timer.remaining_ticks -= 1;
            }

            if timer.remaining_ticks == 0 {
                match timer.timer_type {
                    TimerType::Periodic => {
                        events.send(TimerEvent::Ticked { entity, id: *id });
                        timer.remaining_ticks = timer.duration_ticks;
                    }
                    _ => {
                        events.send(TimerEvent::Expired { entity, id: *id });
                        expired.push(*id);
                    }
                }
            }
        }
        
        for id in expired {
            timers.active.remove(&id);
        }
    }
}
```

**Events**

```rust
enum TimerEvent {
    Started { entity: Entity, id: TimerId },
    Expired { entity: Entity, id: TimerId },
    Ticked { entity: Entity, id: TimerId }, // for periodic
    Cancelled { entity: Entity, id: TimerId },
}
```

**Replication**

Timer components replicated via Lightyear. Clients receive current `remaining_ticks` and can display countdown UI.

---

## Time Scale (Hitstop, Slow-Mo)

For visual effects like hitstop or slow-motion:

**Option A: Pause timers during hitstop**
- Set `paused = true` on affected timers
- Resume when hitstop ends

**Option B: Skip ticks**
- Don't run FixedMain during hitstop (affects all gameplay)
- Simpler but affects everything

Time scale is a design decision. The timer framework supports pausing individual timers for selective slow-down.

---

## Client-Side Visual Timers

For UI animations, particle lifetimes, and other visual-only timing, use Bevy's built-in `Timer` with wall-clock time:

```rust
// Visual-only timer (client-side, frame-based)
fn fade_out_ui(time: Res<Time>, mut query: Query<&mut FadeTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta()); // Wall-clock time is fine here
    }
}
```

These don't need tick synchronization because they don't affect gameplay state.

---

## What's Not Here

- Timer pooling (optimization, add when needed)
- Hierarchical timers (timer triggers timer)
- Timer serialization for save/load (deferred with persistence)

*See: framework/state_machine.md (also uses tick-based timing)*
