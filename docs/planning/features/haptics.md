# Haptics

Controller rumble and vibration feedback for game events.

## Core Logic

**Concept**

- Tactile feedback via controller vibration
- Enhances game feel
- Configurable intensity
- Event-driven triggering

**Haptic Events**

| Event | Intensity | Duration | Pattern |
|-------|-----------|----------|---------|
| Hit landed | Medium | Short | Pulse |
| Damage taken | Strong | Medium | Double pulse |
| Dash | Light | Short | Quick |
| Heavy attack | Strong | Medium | Ramp up |
| Death | Strong | Long | Fade out |
| Pickup | Light | Short | Tick |
| UI confirm | Light | Short | Click |

**Intensity Levels**

| Level | Motor strength | Use case |
|-------|----------------|----------|
| None | 0% | Disabled |
| Light | 25% | Subtle feedback |
| Medium | 50% | Standard events |
| Strong | 75% | Important moments |
| Max | 100% | Critical events |

**Pattern Types**

| Pattern | Behavior |
|---------|----------|
| Pulse | Single vibration |
| Double | Two quick pulses |
| Ramp | Increase intensity |
| Fade | Decrease intensity |
| Continuous | Sustained vibration |

**Operations**

`play_haptic(pattern, intensity, duration)` - Trigger feedback

- Send to controller
- Respect settings

`stop_haptic()` - Cancel current

- Immediately stop vibration

`set_haptic_enabled(enabled)` - Toggle

- Enable or disable all haptics

`set_haptic_intensity(multiplier)` - Adjust strength

- Global intensity modifier

**Invariants**

- Respects user preferences
- No haptics if disabled
- Duration limits prevent stuck vibration
- Works with supported controllers

**Design Notes**

- Specific event mappings left to design
- Platform support varies
- Test on target controllers

---

## Bevy Integration

**Resources**

- HapticSettings { enabled, intensity_multiplier }

**Messages/Commands**

- PlayHaptic { pattern, intensity, duration }
- StopHaptic
- SetHapticIntensity { multiplier }

**Events**

- HapticTriggered { pattern }

**Systems**

- Listen for game events
- Map to haptic patterns
- Send to controller API
- Respect settings

**Platform Integration**

```rust
fn trigger_haptic(
    gamepads: Res<Gamepads>,
    mut rumble: ResMut<GamepadRumble>,
    pattern: HapticPattern,
) {
    for gamepad in gamepads.iter() {
        rumble.add(gamepad, pattern.intensity, pattern.duration);
    }
}
```

**Accessibility**

- Global enable/disable
- Intensity slider
- Consider motion sensitivity

*See: accessibility_motor.md, settings_input.md*
