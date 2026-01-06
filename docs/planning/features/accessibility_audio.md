# Audio Accessibility

Options for players with hearing impairments including visual cues, subtitles, and audio adjustments.

## Core Logic

**Concept**

- Accommodate hearing impairments
- Visual alternatives to audio cues
- Subtitle support
- Separate volume controls

**Visual Audio Indicators**

| Sound Type | Visual Alternative |
|------------|-------------------|
| Enemy attack | Directional indicator on screen edge |
| Damage taken | Screen flash (configurable) |
| Important pickup | Particle burst |
| Dialogue | Subtitles |
| Ambient danger | UI warning icon |
| Footsteps | Minimap pings |

**Subtitle Options**

| Option | Type | Description |
|--------|------|-------------|
| Enabled | bool | Show subtitles |
| Speaker names | bool | Show who's talking |
| Size | enum | Text size |
| Background | enum | None, transparent, opaque |
| Position | enum | Bottom, top |
| Sound descriptions | bool | [footsteps], [explosion] |

**Volume Controls**

| Channel | Purpose |
|---------|---------|
| Master | Overall volume |
| Music | Background music |
| SFX | Sound effects |
| Dialogue | Voice/speech |
| UI | Interface sounds |
| Ambient | Environmental |

**Operations**

`show_sound_indicator(direction, type)` - Visual cue

- Display directional indicator
- For off-screen sounds

`display_subtitle(speaker, text, duration)` - Show subtitle

- Render subtitle text
- Queue if multiple

**Invariants**

- No gameplay information audio-only
- All critical sounds have visual alternative
- Subtitles don't obscure gameplay

**Design Notes**

- Specific indicator visuals left to design
- Subtitle styling left to design

---

## Bevy Integration

**Resources**

- AudioAccessibility { indicators_enabled, subtitles, volumes }
- SubtitleQueue { entries: VecDeque }

**Components**

- SoundIndicator - on-screen directional marker

**Systems**

- Listen for spatial sounds, spawn indicators
- Manage subtitle display
- Apply volume settings per channel

**Scripting Compatibility**

- Sound events hookable for custom indicators
- Subtitle display as command

*See: spatial_audio.md, dialogue_system.md*
