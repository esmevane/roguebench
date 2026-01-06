# Light Strobe

Rhythmic on/off pulsing for alarms, combat feedback, and dramatic effects.

## Core Logic

**Concept**

- Light alternates between on and off
- Regular or irregular timing
- High-impact visual effect

**Strobe Patterns**

| Pattern | On/Off Ratio | Use Case |
|---------|--------------|----------|
| Regular | 50/50 | Alarm, warning |
| Quick | 20/80 | Lightning flash |
| Slow | 70/30 | Heartbeat, tension |
| Double | 2 quick, pause | Emergency signal |
| Morse | Variable | Communication |

**Parameters**

| Property | Type | Description |
|----------|------|-------------|
| on_duration | f32 | Time light is on |
| off_duration | f32 | Time light is off |
| on_intensity | f32 | Brightness when on |
| off_intensity | f32 | Brightness when off (0 = full off) |
| pattern | enum | Strobe pattern type |
| repeat | int | Times to repeat (-1 = infinite) |

**Operations**

`start_strobe()` - Begin strobing

- Reset phase
- Start pattern

`stop_strobe()` - End strobing

- Return to base intensity
- Clear pattern

`update_strobe(dt)` - Advance state

- Track time in cycle
- Switch on/off at thresholds

**Pattern Sequences**

```
Regular:  [ON][OFF][ON][OFF]...
Double:   [ON][OFF][ON][OFF][----][ON][OFF][ON][OFF]...
Quick:    [ON][--------][ON][--------]...
```

**Invariants**

- Transition is instant (no fade)
- Pattern repeats or ends cleanly
- Photosensitivity: limit frequency
- Can layer with other effects

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| On duration | 0.1s | Flash length |
| Off duration | 0.1s | Dark length |
| Max frequency | 10hz | Safety limit |

---

## Bevy Integration

**Components**

- LightStrobe { on_duration, off_duration, pattern, phase, remaining }
- StrobePattern enum

**Systems**

- Update strobe phase
- Toggle intensity at thresholds
- Handle pattern completion

**Implementation**

```rust
fn update_strobes(
    time: Res<Time>,
    mut query: Query<(&mut PointLight2d, &mut LightStrobe)>,
) {
    for (mut light, mut strobe) in &mut query {
        strobe.phase += time.delta_secs();
        let cycle = strobe.on_duration + strobe.off_duration;
        let in_cycle = strobe.phase % cycle;

        light.intensity = if in_cycle < strobe.on_duration {
            strobe.on_intensity
        } else {
            strobe.off_intensity
        };
    }
}
```

**Events**

- StrobeCompleted { entity } - when finite strobe ends

**Presets**

```rust
impl LightStrobe {
    pub fn alarm() -> Self { /* 5hz regular */ }
    pub fn lightning() -> Self { /* quick flash */ }
    pub fn emergency() -> Self { /* double pulse */ }
}
```
