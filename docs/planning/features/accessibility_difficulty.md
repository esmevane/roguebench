# Difficulty Accessibility

Granular difficulty options allowing players to customize challenge independently across systems.

## Core Logic

**Concept**

- Separate difficulty from accessibility
- Granular control over challenge
- No single "easy mode" stigma
- Player agency over experience

**Difficulty Axes**

| Axis | Options | Affects |
|------|---------|---------|
| Damage taken | 0.25x - 2x | Incoming damage multiplier |
| Damage dealt | 0.5x - 2x | Outgoing damage multiplier |
| Enemy aggression | Low - High | Attack frequency |
| Enemy count | 0.5x - 2x | Spawn multiplier |
| Resource drops | 0.5x - 3x | Loot frequency |
| Timer pressure | Off - Strict | Time limits |
| Death penalty | None - Full | What's lost on death |

**Preset Profiles**

| Profile | Description |
|---------|-------------|
| Story | Focus on narrative, minimal challenge |
| Standard | Intended experience |
| Challenge | Increased difficulty |
| Custom | Player-defined settings |

**Operations**

`set_difficulty_axis(axis, value)` - Adjust single axis

- Modify specific difficulty parameter
- Update derived values

`apply_preset(profile)` - Load preset

- Apply all axis values from preset
- Allow further customization

`get_effective_damage(base, target)` - Calculate damage

- Apply difficulty modifiers
- Return adjusted value

**Assist Mode**

- Invincibility toggle (temporary or permanent)
- Skip encounter option
- Resource boost button
- No penalty for using

**Invariants**

- All content accessible at any difficulty
- No unlocks locked by difficulty
- Difficulty adjustable mid-game
- Multiplayer uses host settings (or consensus)

**Design Notes**

- Specific multiplier values left to balance
- Preset configurations left to design
- Consider "recommended" not "normal"

---

## Bevy Integration

**Resources**

- DifficultySettings { axes: HashMap<DifficultyAxis, f32> }
- CurrentDifficultyProfile

**Systems**

- Apply damage multipliers
- Adjust spawn rates
- Modify drop tables
- Handle death penalties

**Integration Points**

- `damage_system.md` - damage multipliers
- `wave_system.md` - enemy count
- `loot_tables.md` - drop rates
- `run_state.md` - death penalties

**Multiplayer**

- Host sets baseline
- Guests see settings before joining
- Or vote-based selection

**Scripting Compatibility**

- Difficulty values readable
- Content can scale with settings
- Presets definable via data

*See: architecture/data.md, architecture/editor.md*
