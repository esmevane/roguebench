# Credits

Attribution screen listing contributors, tools, and assets used in the game.

## Core Logic

**State**

- Scroll position (if scrolling)
- Is visible (bool)

**Content Sections**

- Game title and version
- Development
- Tools and technology
- Assets and attribution
- Special thanks

**Operations**

`show()` - Display credits

- Reset scroll position
- Begin display/animation

`scroll(direction)` - Manual scroll (optional)

- Move view up/down
- Clamp to content bounds

`back()` - Return to previous menu

- Close credits screen

**Invariants**

- All attributions accurate
- Asset licenses honored
- Scrollable if content exceeds screen
- Any input returns to menu

**Content**

| Section | Items |
| ------- | ----- |
| Development | Designer, Programmer |
| Engine | Bevy Engine |
| Audio | Sound sources, licenses |
| Art | Asset sources, licenses |
| Libraries | Key dependencies |
| Special Thanks | Contributors, inspiration |

---

## Bevy Integration

**Screen State**

- MenuState::Credits
- Or Screen::Credits

**Input**

- Any key or button to return
- Optional: Up/Down to scroll
- Escape to return immediately

**Systems**

- Render credit sections
- Handle scroll input (optional)
- Detect back input
- Transition to previous screen

**UI Options**

1. Static: All content visible, scrollable
2. Rolling: Auto-scroll like film credits
3. Paged: Multiple pages with navigation

**Text**

- Centered or left-aligned sections
- Section headers larger/bold
- Links optional (itch.io, GitHub)

**Defaults**

| Field | Value | Description |
| ----- | ----- | ----------- |
| Style | Static | No auto-scroll |
| Return | Any key | Easy exit |
