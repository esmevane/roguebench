# Rhythm Timer System

## Overview

A tempo-aware timing abstraction that extends beyond simple countdowns. Rhythm provides a unified vocabulary for all time-based behavior while enabling musical synchronization, reversible timelines, and event-driven cadences.

## Motivation

Current timer patterns in the codebase use raw `f32` fields with inconsistent APIs:
- `Dash.dash_timer` / `cooldown_timer` (manual tick methods)
- `Attack.lifetime` (Option<f32>, decremented in systems)
- `Invincible.remaining` (Option<f32> with tick method)
- `AIConfig.cooldown_timer` (f32 with tick_cooldown method)

This primitive obsession obscures intent and prevents reuse. A Rhythm abstraction would:
1. Replace scattered timer logic with a consistent component
2. Enable BPM-aligned animations and effects
3. Support reversal/rewind for time manipulation abilities
4. Allow event-driven progression ("3 hits" not "3 seconds")

## Core Concepts

### Beat

The fundamental unit of rhythm. Can be defined by:
- **Time**: Traditional duration in seconds
- **Tempo**: BPM-relative (beat = 60/bpm seconds)
- **Events**: Count of occurrences (hits, steps, triggers)

### Cadence

A sequence or pattern of beats. Defines how a rhythm progresses:
- **Linear**: Standard countdown/countup
- **Loop**: Repeats after completion
- **Bounce**: Reverses direction at boundaries
- **Custom**: User-defined beat patterns

### Phase

Position within a rhythm cycle (0.0 to 1.0). Useful for:
- Animation curves
- Shader effects
- Audio synchronization
- UI feedback

## Proposed API

```rust
/// Core rhythm component
#[derive(Component)]
pub struct Rhythm {
    /// Current phase (0.0 to 1.0)
    phase: f32,
    /// Progression mode
    cadence: Cadence,
    /// Beat definition
    beat: Beat,
    /// Direction (1.0 forward, -1.0 reverse)
    direction: f32,
    /// Paused state
    paused: bool,
}

/// How beats are measured
pub enum Beat {
    /// Fixed duration in seconds
    Duration(f32),
    /// BPM-relative timing
    Tempo { bpm: f32, beats: f32 },
    /// Event-driven (external tick)
    Events { target: u32, current: u32 },
}

/// Progression pattern
pub enum Cadence {
    /// Runs once, fires Completed
    Once,
    /// Loops forever
    Loop,
    /// Loops N times
    LoopN(u32),
    /// Bounces between 0 and 1
    Bounce,
    /// Custom beat pattern
    Pattern(Vec<f32>),
}

impl Rhythm {
    /// Create a simple countdown
    pub fn countdown(seconds: f32) -> Self;
    
    /// Create a tempo-synced rhythm
    pub fn at_tempo(bpm: f32, beats: f32) -> Self;
    
    /// Create an event-driven countdown
    pub fn after_events(count: u32) -> Self;
    
    /// Set cadence
    pub fn with_cadence(self, cadence: Cadence) -> Self;
    
    /// Reverse direction
    pub fn reverse(&mut self);
    
    /// Rewind to start
    pub fn rewind(&mut self);
    
    /// Check if complete (for Once cadence)
    pub fn is_complete(&self) -> bool;
    
    /// Get current phase for animation/effects
    pub fn phase(&self) -> f32;
    
    /// Get phase with easing applied
    pub fn eased(&self, curve: EasingCurve) -> f32;
}
```

## Usage Examples

### Cooldown Timer (replaces current pattern)

```rust
// Before
pub struct Dash {
    cooldown_timer: f32,
    cooldown: f32,
}

// After
commands.spawn((
    Dash::default(),
    Rhythm::countdown(0.5).with_cadence(Cadence::Once),
));
```

### BPM-Synced Animation

```rust
// Bounce animation synced to 120 BPM
commands.spawn((
    Sprite::default(),
    Rhythm::at_tempo(120.0, 1.0).with_cadence(Cadence::Bounce),
));

fn animate_to_beat(mut query: Query<(&Rhythm, &mut Transform)>) {
    for (rhythm, mut transform) in query.iter_mut() {
        // Scale bounces with the beat
        let scale = 1.0 + rhythm.eased(EasingCurve::SineInOut) * 0.1;
        transform.scale = Vec3::splat(scale);
    }
}
```

### Event-Driven Combo Window

```rust
// Combo window: 3 hits within the rhythm
commands.spawn((
    ComboTracker::default(),
    Rhythm::after_events(3).with_cadence(Cadence::Once),
));

fn on_hit(mut query: Query<&mut Rhythm, With<ComboTracker>>) {
    for mut rhythm in query.iter_mut() {
        rhythm.tick_event(); // Progress by one event
    }
}
```

### Time Manipulation Ability

```rust
// Rewind time ability
fn activate_rewind(mut rhythms: Query<&mut Rhythm>) {
    for mut rhythm in rhythms.iter_mut() {
        rhythm.reverse(); // Everything now runs backward
    }
}
```

## Integration Points

### With Bevy Timer

`Rhythm` can wrap or complement `bevy::time::Timer`:
- Use Bevy's `Timer` for the underlying time tracking
- Add phase calculation, cadence, and beat semantics on top

### With Animation

Phase output maps directly to animation curves:
- `rhythm.phase()` → keyframe position
- `rhythm.eased(curve)` → smooth transitions

### With Audio

BPM-relative beats enable music sync:
- Global BPM resource
- Rhythms auto-adjust when BPM changes
- Phase aligns to musical measures

### With Tweens

Rhythm can drive tween progress:
- Replace tween duration with Rhythm component
- Gain reversal, looping, tempo sync for free

## Implementation Notes

### Observer Events

```rust
#[derive(Event)]
pub struct RhythmCompleted { pub entity: Entity }

#[derive(Event)]  
pub struct RhythmPhaseChanged { pub entity: Entity, pub phase: f32 }

#[derive(Event)]
pub struct RhythmBeat { pub entity: Entity, pub beat: u32 }
```

### System Ordering

Rhythm ticking should happen early in the frame:
- `PreUpdate` or dedicated `RhythmSet`
- Before systems that read phase/completion

### Serialization

All rhythm state should be serializable for:
- Save/load
- Network replication
- Replay systems

## Future Extensions

- **Polyrhythm**: Multiple layered rhythms
- **Swing**: Non-uniform beat spacing
- **Quantization**: Snap events to nearest beat
- **Conductor**: Global tempo controller with gradual BPM changes

## Priority

P3 - Nice to have, not blocking current work. The immediate need (timer unification) can be addressed with Bevy's `Timer` component. Rhythm is the aspirational target for when we want musical/temporal expressiveness.
