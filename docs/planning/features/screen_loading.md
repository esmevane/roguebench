# Loading Screen

Asset loading screen shown before gameplay. Waits for assets to be ready.

## Core Logic

**State**

- Assets loaded (bool)
- Progress (f32) - 0.0 to 1.0

**Operations**

`check_assets()` - Verify loading complete

- Query asset server states
- Return true if all loaded

`update_progress()` - Calculate progress

- Count loaded vs total assets
- Update progress value

**Invariants**

- Shown while assets load
- Auto-advances when complete
- Shows progress indication

**Defaults**

| Field    | Value    | Description    |
| -------- | -------- | -------------- |
| Next     | Gameplay | On load complete |

**Content**

- "Loading..." text
- Progress bar (optional)
- Tips/hints (optional)

---

## Bevy Integration

**Screen State**

- Screen::Loading variant

**Asset Tracking**

- Query AssetServer for load states
- Check all required handles

**Systems**

- loading_check queries asset states
- Transitions to Screen::Gameplay when ready

**Assets Loaded**

- Sprite sheets
- Audio files
- Config files
- Particle effects
