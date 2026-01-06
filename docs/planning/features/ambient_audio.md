# Ambient Audio

Environmental soundscapes providing atmosphere for different areas.

## Core Logic

**Concept**

- Background environmental sounds
- Layered with music
- Changes based on location
- Enhances immersion

**Ambient Types**

| Type | Examples |
|------|----------|
| Environment | Wind, rain, cave drips |
| Activity | Crowd murmur, machinery |
| Nature | Birds, insects, water |
| Tension | Heartbeat, breathing |

**Ambient Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Ambient identifier |
| layers | list | Sound layers |
| volume | float | Base volume |
| fade_time | float | Transition duration |

**Layer Properties**

| Property | Type | Description |
|----------|------|-------------|
| sound | asset | Audio file |
| volume | float | Layer volume |
| loop | bool | Continuous loop |
| random_interval | option | Random playback timing |
| chance | float | Play probability |

**Operations**

`set_ambient(ambient_id)` - Change soundscape

- Fade out current
- Fade in new ambient
- All layers start

`add_layer(layer)` - Add sound layer

- Layer on top of current
- For dynamic additions

`remove_layer(layer_id)` - Remove layer

- Fade out specific layer

`set_volume(volume)` - Adjust volume

- Apply to all layers

**Invariants**

- Layers blend smoothly
- Transitions don't pop
- Volume respects settings
- Random sounds feel natural

**Design Notes**

- Specific ambients left to design
- Room-ambient mapping left to design
- Layer composition left to design

---

## Bevy Integration

**Resources**

- CurrentAmbient { ambient_id, active_layers }
- AmbientConfig { room_ambients: HashMap<RoomType, AmbientId> }

**Assets**

- Ambient sounds as AudioSource
- Looping and one-shot variants

**Messages/Commands**

- SetAmbient { ambient_id }
- AddAmbientLayer { layer }
- RemoveAmbientLayer { layer_id }
- SetAmbientVolume { volume }

**Events**

- AmbientChanged { old, new }
- AmbientLayerAdded { layer_id }

**Systems**

- Monitor room changes
- Select appropriate ambient
- Manage layer playback
- Handle random layer triggers

**Layer Management**

```rust
struct AmbientLayer {
    sound: Handle<AudioSource>,
    volume: f32,
    looping: bool,
    random_interval: Option<Range<f32>>,
    next_play: Option<f32>,
}
```

**Room Integration**

- Rooms specify ambient type
- Transition on room enter
- Blend during transitions

**Scripting Compatibility**

- Ambient control as commands
- Current ambient readable
- Events hookable

*See: architecture/scripting.md*
