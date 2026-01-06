# Input Buffering

Stores recent inputs to execute when possible, improving responsiveness. Allows actions queued slightly before they're available.

## Core Logic

**State**

- Buffer (list of inputs) - recent inputs with timestamps
- Buffer window (f32) - how long inputs remain valid
- Current time (f32) - for expiration checking

**Operations**

`add(input, timestamp)` - Buffer an input

- Add input with current timestamp
- Remove expired inputs (older than buffer window)

`consume(input_type)` - Try to use buffered input

- Check if input of type exists in buffer
- If found, remove it and return true
- Otherwise return false

`clear()` - Remove all buffered inputs

- Empty the buffer

`tick(dt)` - Advance time

- Update current time
- Remove expired inputs

**Invariants**

- Inputs expire after buffer window
- Each buffered input consumed only once
- Newer inputs don't replace older ones (queue)
- Buffer has max size to prevent memory growth

**Defaults**

| Field         | Value | Description        |
| ------------- | ----- | ------------------ |
| Buffer window | 0.15  | Seconds (150ms)    |
| Max size      | 5     | Maximum buffered inputs |

**Buffered Actions**

| Input  | Description          |
| ------ | -------------------- |
| Dash   | Dash when available  |
| Melee  | Attack when cooldown ready |
| Ranged | Attack when cooldown ready |

---

## Bevy Integration

**Component**

- InputBuffer on player entity
- Stores vec of (InputType, Timer)

**Usage**

- On input press, add to buffer
- Each frame, check buffer against available actions
- Consume and execute if action becomes available

**Timing**

- Runs in Update schedule
- Checks before movement/action systems
