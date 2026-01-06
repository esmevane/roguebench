# Music System

Background music playback with state-based track selection and transitions.

## Core Logic

**Concept**

- Background music plays continuously
- Track changes based on game state
- Smooth transitions between tracks
- Volume and playback control

**Music States**

| State | Track Type | Trigger |
|-------|------------|---------|
| Menu | Menu theme | Main menu |
| Explore | Ambient/calm | Non-combat rooms |
| Combat | Intense | Enemy present |
| Boss | Epic | Boss fight |
| Shop | Relaxed | Shop room |
| Victory | Triumphant | Level complete |

**Track Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Track identifier |
| asset | path | Audio file |
| loop | bool | Loop playback |
| volume | float | Base volume |
| intro | option | Non-looping intro |
| transition_points | list | Beat-sync points |

**Operations**

`play(track_id)` - Start track

- Fade out current
- Fade in new track
- Respect transition

`stop()` - Stop playback

- Fade out
- Silence

`set_volume(volume)` - Adjust volume

- Apply volume setting
- Respect master/music sliders

`crossfade(track_id, duration)` - Smooth transition

- Overlap fade between tracks

`set_state(music_state)` - State-based selection

- Choose track for state
- Handle transition

**Transitions**

| Type | Behavior |
|------|----------|
| Cut | Immediate switch |
| Crossfade | Overlap blend |
| Fade out/in | Gap between |
| Beat-sync | Wait for beat |

**Invariants**

- Only one track at a time
- Transitions smooth
- Volume respects settings
- Looping seamless

**Design Notes**

- Specific tracks left to design
- State-track mapping left to design
- Transition timing left to design

---

## Bevy Integration

**Resources**

- MusicState { current_track, volume, state }
- MusicConfig { state_tracks: HashMap<MusicState, TrackId> }

**Assets**

- Music tracks as AudioSource
- Loaded on demand or preloaded

**Messages/Commands**

- PlayMusic { track_id }
- StopMusic
- SetMusicVolume { volume }
- SetMusicState { state }
- CrossfadeMusic { track_id, duration }

**Events**

- MusicStarted { track_id }
- MusicStopped
- MusicStateChanged { old_state, new_state }

**Systems**

- Monitor game state for music triggers
- Handle transitions
- Apply volume settings

**Audio Integration**

```rust
fn play_music(
    commands: &mut Commands,
    assets: Res<MusicAssets>,
    track_id: &str,
) {
    commands.spawn((
        AudioPlayer::new(assets.get(track_id)),
        PlaybackSettings::LOOP.with_volume(music_volume),
        MusicTrack,
    ));
}
```

**Scripting Compatibility**

- Play/stop/crossfade as commands
- Current state readable
- Events hookable

*See: architecture/scripting.md*
