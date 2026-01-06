# Cognitive Accessibility

Options for players with cognitive differences including reduced complexity, reminders, and clarity enhancements.

## Core Logic

**Concept**

- Accommodate cognitive differences
- Reduce information overload
- Provide reminders and guidance
- Simplify decision-making

**Complexity Reduction**

| Option | Type | Description |
|--------|------|-------------|
| Enemy count limit | int | Max simultaneous enemies |
| Simplified UI | bool | Show only essential info |
| Reduced effects | bool | Fewer particles/distractions |
| Objective markers | bool | Clear goal indicators |
| Quest reminders | bool | Periodic objective hints |

**Memory Assistance**

| Option | Type | Description |
|--------|------|-------------|
| Control hints | enum | Always, contextual, off |
| Objective reminders | bool | Periodic notification |
| NPC indicators | bool | Mark interactable NPCs |
| Path hints | bool | Suggest next direction |
| Recap on resume | bool | Summary when loading |

**Reading Assistance**

| Option | Type | Description |
|--------|------|-------------|
| Text speed | float | Dialogue display rate |
| Reading time | float | Extended before auto-advance |
| Dyslexia font | bool | Alternative font option |
| Text highlighting | bool | Highlight key words |

**Decision Support**

| Option | Type | Description |
|--------|------|-------------|
| Item comparisons | bool | Show stat differences |
| Recommended actions | bool | Suggest choices |
| Undo support | bool | Allow reversing decisions |
| Confirmation prompts | bool | Confirm important actions |

**Operations**

`show_reminder(type, content)` - Display reminder

- Objective, control, or path reminder
- Non-intrusive notification

`simplify_display(level)` - Reduce complexity

- Hide non-essential UI elements
- Reduce visual noise

**Invariants**

- Core gameplay preserved
- Assistance is optional
- No patronizing presentation
- Settings persist

**Design Notes**

- Specific reminder timing left to design
- Test with neurodivergent users

---

## Bevy Integration

**Resources**

- CognitiveAccessibility { simplified_ui, reminders, ... }

**Systems**

- Manage enemy spawn limits
- Schedule reminders
- Filter UI complexity
- Apply reading assistance

**Integration Points**

- `quest_framework.md` - objective reminders
- `dialogue_system.md` - reading speed
- `wave_system.md` - enemy count limit
- `ui_*.md` - simplified UI modes

**Scripting Compatibility**

- Settings readable
- Reminders hookable
- Content can adapt to settings

*See: tutorial_system.md*
