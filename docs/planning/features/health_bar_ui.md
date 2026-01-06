# Health Bar UI

Visual display of player health. Shows current health as filled bar with frame decoration.

## Core Logic

**Components**

- Frame - decorative border
- Background - empty bar area
- Fill - colored health portion

**State**

- Current health (read from player)
- Max health (read from player)
- Fill percentage (current / max)

**Operations**

`update(current, max)` - Refresh display

- Calculate percentage
- Set fill width to percentage of max width

**Invariants**

- Fill never exceeds frame bounds
- Updates in real-time as health changes
- Fill color indicates health level (optional)
- Always visible during gameplay

**Defaults**

| Field       | Value     | Description |
| ----------- | --------- | ----------- |
| Position    | Top-left  | Screen corner |
| Width       | 200       | Pixels      |
| Height      | 20        | Pixels      |
| Fill color  | Red/Green | Health-based |

---

## Bevy Integration

**Components**

- HealthBarFrame
- HealthBarBackground
- HealthBarFill

**Hierarchy**

- Frame (parent)
  - Background (child)
  - Fill (child)

**Systems**

- Query player Health
- Update fill Node width based on percentage

**Styling**

- UI Node with Image/Sprite
- Medieval frame graphics
- Positioned via Style component
