# Options Menu

Settings menu for audio, controls, and game feel preferences. Accessible from main menu or pause menu.

## Core Logic

**State**

- Selected setting index
- Setting values (persisted)

**Categories**

- Audio
- Game Feel
- (Future: Controls, Display)

**Settings**

| Setting | Type | Range | Default |
| ------- | ---- | ----- | ------- |
| Master Volume | float | 0.0-1.0 | 0.7 |
| Music Volume | float | 0.0-1.0 | 0.5 |
| SFX Volume | float | 0.0-1.0 | 0.8 |
| Camera Shake | bool | on/off | on |
| Screen Flash | bool | on/off | on |
| Hitstop | bool | on/off | on |

**Operations**

`navigate(direction)` - Move between settings

- Up/Down changes selected setting
- Wraps at boundaries

`adjust(direction)` - Change setting value

- Left/Right adjusts value
- Sliders: increment/decrement
- Toggles: flip state

`back()` - Return to previous menu

- Save settings
- Close options menu

**Invariants**

- Settings persist across sessions
- Changes apply immediately (preview)
- Back saves automatically
- Invalid values clamped to range

---

## Bevy Integration

**Resources**

- AudioSettings { master, music, sfx }
- GameFeelConfig { toggles... }

**Persistence**

- Save to file on change
- Load on app start
- Default if file missing

**Input**

- W/S or Up/Down for navigation
- A/D or Left/Right for adjustment
- Escape or B to back

**Systems**

- Render settings list
- Handle navigation
- Handle adjustment
- Apply to resources
- Persist to disk

**UI**

- Setting name on left
- Value/slider on right
- Category headers
- Back button at bottom
