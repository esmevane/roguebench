# UI Sounds

Audio feedback for user interface interactions. Plays on button clicks and hovers.

## Core Logic

**Sound Types**

- Click sounds - button pressed
- Hover sounds - mouse enters button

**Operations**

`play_click()` - Play click sound

- Select random variant
- Play at UI volume

`play_hover()` - Play hover sound

- Select random variant
- Subtle volume

**Invariants**

- Immediate feedback on interaction
- Consistent across all UI elements
- Not intrusive during gameplay
- Random variation for interest

**Defaults**

| Sound  | Variants | Volume | Description |
| ------ | -------- | ------ | ----------- |
| Click  | 5        | 0.4    | Confirmation |
| Hover  | 3        | 0.2    | Subtle      |

---

## Bevy Integration

**Resources**

- AudioAssets { click_sounds, hover_sounds, ... }

**Components**

- UISoundOnClick - marker for clickable elements
- UISoundOnHover - marker for hoverable elements

**Systems**

- Detect Interaction changes on marked entities
- Play sound on Pressed / Hovered transitions

**Usage**

- Add markers to buttons
- System handles sound playback
