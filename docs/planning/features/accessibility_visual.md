# Visual Accessibility

Options for players with visual impairments including colorblind modes, contrast settings, and visual clarity.

## Core Logic

**Concept**

- Accommodate various visual impairments
- Alternative color palettes
- Enhanced contrast and clarity
- Configurable visual indicators

**Colorblind Modes**

| Mode | Accommodation |
|------|---------------|
| Deuteranopia | Red-green (most common) |
| Protanopia | Red-green (red-weak) |
| Tritanopia | Blue-yellow |
| Achromatopsia | Monochrome |
| Custom | User-defined palette |

**Options**

| Option | Type | Description |
|--------|------|-------------|
| Colorblind mode | enum | Palette adjustment |
| High contrast | bool | Enhanced edge/element contrast |
| UI scale | float | Enlarge interface elements |
| Font size | enum | Text size multiplier |
| Screen shake | bool/float | Reduce or disable |
| Flash effects | bool/float | Reduce or disable |
| Enemy outlines | bool | Highlight enemy silhouettes |
| Item highlights | bool | Enhanced pickup visibility |

**Operations**

`apply_colorblind_mode(mode)` - Adjust palette

- Apply color transformation
- Affects all game rendering

`set_contrast(level)` - Adjust contrast

- Modify shader parameters
- Enhance visibility

**Color Replacement Strategy**

- Map problematic colors to distinguishable alternatives
- Maintain color meaning (red=danger) via luminance/pattern
- Add shapes/icons alongside color coding

**Invariants**

- All critical information readable in any mode
- Gameplay not affected by accessibility settings
- Settings persist across sessions

**Design Notes**

- Specific palettes left to implementation
- Test with actual colorblind users

---

## Bevy Integration

**Resources**

- AccessibilitySettings { colorblind_mode, contrast, ... }

**Implementation Options**

- Post-processing shader for color adjustment
- Palette swap in sprite rendering
- Both approaches have tradeoffs

**Systems**

- Apply color correction to render pipeline
- Scale UI elements
- Toggle visual effects based on settings

**Scripting Compatibility**

- Settings readable by scripts
- Design can offer accessibility-aware content

*See: architecture/data.md (persistence)*
