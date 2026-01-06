# Wave System

Progressive enemy spawning in waves. Each wave spawns enemies, next wave starts when all defeated.

## Core Logic

**State**

- Current wave (i32) - wave number (1-indexed)
- Total waves (i32) - maximum waves
- Enemies remaining (i32) - alive enemies this wave
- Wave delay timer (f32) - pause between waves
- Wave state (enum) - spawning, active, delay, complete

**Operations**

`start_wave(wave_num)` - Begin a wave

- Calculate enemy count for wave
- Determine enemy distribution
- Spawn enemies at spawn points

`enemy_died()` - Track enemy death

- Decrement enemies remaining
- If zero, wave complete

`check_wave_complete()` - Test completion

- Return enemies remaining == 0

`next_wave()` - Advance to next wave

- Increment current wave
- Start delay timer
- When delay expires, start_wave

**Invariants**

- Waves progress sequentially
- All enemies must die to advance
- Delay between waves for breathing room
- Victory after final wave

**Defaults**

| Field        | Value | Description          |
| ------------ | ----- | -------------------- |
| Total waves  | 10    |                      |
| Wave delay   | 2.0   | Seconds              |
| Base enemies | 3     | Wave 1 enemy count   |
| Scaling      | 0.5   | Additional per wave  |

**Enemy Count Formula**

`count = base + (wave - 1) * scaling`

---

## Bevy Integration

**Resource**

- WaveState { current, total, remaining, timer, state }

**Events**

- WaveStarted { wave_num }
- WaveComplete { wave_num }
- AllWavesComplete (victory condition)

**Systems**

- wave_spawning spawns enemies at wave start
- wave_tracking counts enemy deaths
- wave_progression handles transitions

---

## Framework Dependencies

- `framework/spawn_framework.md` - Enemy spawning
- `framework/timer_framework.md` - Wave delay timing
- `framework/state_machine.md` - Wave state management
- `framework/event_hooks.md` - Wave start/complete hooks
- `framework/data_loading.md` - Wave configuration

*See: architecture/scripting.md, architecture/data.md*
