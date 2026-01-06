# Tutorial Hints

Contextual control hints and guidance displayed during gameplay.

## Core Logic

**Concept**

- Subtle reminders of controls
- Context-sensitive display
- Fade after learning
- Configurable visibility

**Hint Types**

| Type | Display | Example |
|------|---------|---------|
| Control | Key/button icon | "E to interact" |
| Contextual | Near relevant object | "Chest - E to open" |
| Combo | Multiple inputs | "Space then Click" |
| Reminder | Periodic | "Don't forget to dash!" |

**Hint Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Hint identifier |
| context | enum | When to show |
| input | list | Controls to display |
| text | string | Hint text |
| position | enum | Screen location |
| duration | float | Show time |
| fade_after_uses | int | Hide after N uses |

**Display Modes**

| Mode | Behavior |
|------|----------|
| Always | Always visible when relevant |
| Learning | Fade after demonstrated mastery |
| On demand | Only with hint button held |
| Off | Never show |

**Operations**

`show_hint(context)` - Display relevant hint

- Find hint for context
- Display if not mastered
- Track usage

`record_action(action)` - Track learning

- Count successful uses
- Fade hints after threshold

`set_hint_mode(mode)` - Configure display

- Apply selected mode

**Invariants**

- Hints don't obstruct gameplay
- Input icons match current device
- Fade smoothly
- Respect accessibility settings

**Design Notes**

- Specific hints left to design
- Icon design left to implementation
- Consider input device switching

---

## Bevy Integration

**Resources**

- HintConfig { mode, fade_threshold }
- HintUsage { counts: HashMap<HintId, u32> }

**Components**

- HintTrigger { hint_id } - on objects that trigger hints

**Systems**

- Detect hint contexts
- Display appropriate hint
- Track usage for fading
- Update icons for input device

**UI**

- Icon + text display
- Position near relevant object or screen edge
- Animate in/out

**Input Device Awareness**

- Detect keyboard vs gamepad
- Show correct icons
- Update on device change

**Scripting Compatibility**

- Hints definable in data
- Custom hints via scripting

*See: tutorial_system.md, controls_hint_ui.md*
