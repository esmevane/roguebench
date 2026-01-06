# Splash Screen

Initial screen shown on game launch. Displays branding, then auto-advances.

## Core Logic

**State**

- Display timer (f32) - time to show splash
- Assets loading (bool) - waiting for assets

**Operations**

`start()` - Begin splash

- Show splash visuals
- Start timer

`update(dt)` - Advance

- Tick timer
- When complete, transition to next screen

**Invariants**

- Shown once per game launch
- Auto-advances (no input required)
- Minimum display time
- Can overlay asset loading

**Defaults**

| Field    | Value | Description       |
| -------- | ----- | ----------------- |
| Duration | 1.5   | Seconds           |
| Next     | Title | Screen to advance to |

**Content**

- Game logo/title
- Studio branding (optional)
- Loading indicator (optional)

---

## Bevy Integration

**Screen State**

- Screen::Splash variant

**Systems**

- splash_timer ticks duration
- Transitions to Screen::Title when complete

**Assets**

- Can begin asset loading during splash
- Or purely decorative
