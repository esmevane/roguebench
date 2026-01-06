# Wave Counter UI

Text display showing current wave progress. Updates as waves complete.

## Core Logic

**State**

- Current wave (from wave system)
- Total waves (from wave system)

**Display Format**

- "Wave X / Y"
- X = current wave number
- Y = total waves

**Operations**

`update(current, total)` - Refresh text

- Format string with current values
- Update text component

**Invariants**

- Always shows accurate wave count
- Updates immediately on wave change
- Visible throughout gameplay

**Defaults**

| Field    | Value      | Description    |
| -------- | ---------- | -------------- |
| Position | Below health bar | Screen location |
| Font size| 24         | Pixels         |
| Color    | White      | Text color     |

---

## Bevy Integration

**Components**

- WaveCounterText
- Text component with sections

**Systems**

- Query WaveState resource
- Update Text on wave change

**Styling**

- UI Node with Text
- Semi-transparent background (optional)
- Fixed screen position
