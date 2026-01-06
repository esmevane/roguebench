# Audio Framework

Sound playback, music management, and spatial audio.

## Client-Side Execution

Audio is **purely local** and runs **entirely on the client**. It is not replicated.

```
Server: Game event (damage, death, pickup)
    ↓
Lightyear replication or message
    ↓
Client: Observes event → Plays sound locally
```

**No audio on server.** Server doesn't know about sounds, music, or volume settings.

---

## Triggering Audio

Audio is triggered by observing replicated state or events:

```rust
// Client-side system
fn play_damage_sound(
    mut damage_events: EventReader<DamageApplied>, // Replicated event
    audio: Res<Audio>,
    library: Res<AudioLibrary>,
) {
    for event in damage_events.read() {
        let sound = library.get("hit_impact")?;
        audio.play_spatial(sound, event.position);
    }
}
```

Audio triggers are **ephemeral**—they don't go through the command bus.

---

## Core Logic

**Concept**

- Unified audio playback interface
- Categories with independent volume (user settings)
- Music with crossfade transitions
- Spatial audio for positional sounds

**Audio Categories**

| Category | Use Case |
|----------|----------|
| Master | Global volume control |
| Music | Background music |
| SFX | Sound effects |
| Ambient | Environmental sounds |
| UI | Interface sounds |
| Voice | Dialogue/narration |

**Sound Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Sound identifier |
| asset | path | Audio file path |
| category | enum | Volume category |
| volume | f32 | Base volume 0-1 |
| pitch | range | Pitch variation |
| spatial | bool | Position-based |
| loop | bool | Repeat playback |

**Operations**

`play(sound_id)` - Play sound

- Load if needed
- Apply category volume
- Fire and forget

`play_at(sound_id, position)` - Spatial sound

- Play with position
- Volume by distance from listener

`play_attached(sound_id, entity)` - Follow entity

- Update position with entity

`stop(handle)` - Stop sound

- Immediate stop

`fade_out(handle, duration)` - Fade stop

- Gradual volume decrease

---

## Bevy Integration

**Resources**

```rust
#[derive(Resource)]
struct AudioVolumes {
    master: f32,
    music: f32,
    sfx: f32,
    ambient: f32,
    ui: f32,
    voice: f32,
}

#[derive(Resource)]
struct AudioLibrary {
    sounds: HashMap<String, SoundDefinition>,
}

#[derive(Resource)]
struct CurrentMusic {
    track: Option<Handle<AudioSource>>,
    // For crossfade
}
```

**Components**

```rust
#[derive(Component)]
struct AudioEmitter {
    sound_id: String,
    looping: bool,
}

#[derive(Component)]
struct SpatialAudio {
    max_distance: f32,
}

#[derive(Component)]
struct AudioListener; // Usually on camera or player
```

**Sound Definition**

```rust
struct SoundDefinition {
    id: String,
    asset: AssetPath,
    category: AudioCategory,
    base_volume: f32,
    pitch_variance: Range<f32>,
    spatial: bool,
    max_distance: f32,
}
```

**Systems (client-side, Update schedule)**

```rust
fn update_spatial_audio(
    listener: Query<&Transform, With<AudioListener>>,
    emitters: Query<(&Transform, &SpatialAudio, &AudioSink)>,
) {
    let listener_pos = listener.single().translation.truncate();

    for (transform, spatial, sink) in emitters.iter() {
        let emitter_pos = transform.translation.truncate();
        let distance = listener_pos.distance(emitter_pos);
        let attenuation = (1.0 - distance / spatial.max_distance).max(0.0);
        sink.set_volume(attenuation);
    }
}
```

---

## Common Trigger Patterns

**On damage:**
```rust
fn on_damage_sound(mut events: EventReader<DamageApplied>, audio: Res<Audio>, ...) {
    for event in events.read() {
        audio.play_at("hit_impact", event.position);
    }
}
```

**On UI interaction:**
```rust
fn on_button_sound(mut events: EventReader<ButtonPressed>, audio: Res<Audio>, ...) {
    for _ in events.read() {
        audio.play("ui_click");
    }
}
```

**Music changes (based on game state):**
```rust
fn music_for_state(
    game_state: Res<State<GamePhase>>,
    mut current_music: ResMut<CurrentMusic>,
    audio: Res<Audio>,
) {
    // GamePhase is replicated from server
    let track = match game_state.get() {
        GamePhase::Exploration => "music_exploration",
        GamePhase::Combat => "music_combat",
        GamePhase::Boss => "music_boss",
    };
    
    if current_music.track_id != track {
        audio.crossfade_to(track, Duration::from_secs(2));
        current_music.track_id = track.to_string();
    }
}
```

---

## Volume Settings

Volume settings are **local user preferences**, not replicated:

```rust
fn apply_volume_settings(
    settings: Res<UserSettings>, // Local, not replicated
    mut volumes: ResMut<AudioVolumes>,
) {
    volumes.master = settings.audio.master;
    volumes.music = settings.audio.music;
    volumes.sfx = settings.audio.sfx;
    // etc.
}
```

---

## Implementation Options

Consider these crates for actual audio playback:
- **bevy_audio** — Built-in, simple
- **bevy_kira_audio** — More features, better crossfade

This framework provides the pattern; choose implementation based on needs.

---

## What's Not Here

- Adaptive/layered music (add when needed)
- Audio occlusion (sound blocked by walls)
- Reverb zones

*See: framework/particle_framework.md (similar client-side pattern), framework/ui_framework.md (UI sounds)*
