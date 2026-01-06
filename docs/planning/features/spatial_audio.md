# Spatial Audio

Positional sound effects with distance falloff and stereo positioning.

## Core Logic

**Concept**

- Sounds have world position
- Volume decreases with distance
- Stereo panning based on direction
- Creates sense of space

**Spatial Properties**

| Property | Type | Description |
|----------|------|-------------|
| position | Vec2 | World position |
| max_distance | float | Audible range |
| falloff | enum | Attenuation curve |
| reference_distance | float | Full volume range |
| doppler | bool | Pitch shift on movement |

**Falloff Types**

| Type | Behavior |
|------|----------|
| Linear | Even decrease |
| Inverse | 1/distance |
| Exponential | Rapid falloff |
| None | No distance effect |

**Panning**

- Calculate angle from listener to source
- Pan based on horizontal offset
- Clamp to stereo range

**Operations**

`play_at(sound_id, position)` - Play spatial sound

- Spawn at position
- Calculate initial volume/pan
- Begin playback

`update_listener(position)` - Set listener position

- Usually camera or player
- Affects all spatial sounds

`get_volume(source_pos, listener_pos)` - Calculate volume

- Apply distance falloff
- Return attenuated volume

`get_pan(source_pos, listener_pos)` - Calculate stereo

- Return left/right balance

**Invariants**

- Sounds outside max_distance silent
- Panning natural feeling
- Listener position updated smoothly
- Multiple spatial sounds supported

**Design Notes**

- Falloff curves left to implementation
- Max distances per sound type left to design
- Integration with combat feedback

---

## Bevy Integration

**Components**

- SpatialAudio { max_distance, falloff, reference_distance }
- AudioListener - marks listener entity

**Resources**

- ListenerPosition(Vec2)
- SpatialAudioConfig { default_max_distance, global_falloff }

**Systems**

- Update listener position from camera/player
- Calculate volume/pan for spatial sources
- Apply to audio playback

**Messages/Commands**

- PlaySpatialSound { sound_id, position }
- SetListenerPosition { position }

**Events**

- SpatialSoundPlayed { sound_id, position }

**Implementation**

```rust
fn update_spatial_audio(
    listener: Res<ListenerPosition>,
    mut query: Query<(&Transform, &SpatialAudio, &mut AudioSink)>,
) {
    for (transform, spatial, mut sink) in &mut query {
        let distance = transform.translation.truncate()
            .distance(listener.0);

        let volume = calculate_falloff(distance, spatial);
        let pan = calculate_pan(transform.translation.truncate(), listener.0);

        sink.set_volume(volume);
        sink.set_pan(pan);
    }
}
```

**Combat Integration**

- Hit sounds at impact position
- Weapon sounds at attacker
- Death sounds at dying entity

**Scripting Compatibility**

- Spatial playback as command
- Listener position settable
- Events hookable

*See: architecture/scripting.md*
