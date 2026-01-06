# Aggro Detection

Distance-based player detection for enemies. Determines when enemies become aware of and target the player.

## Core Logic

**State**

- Aggro range (f32) - detection radius
- Has target (bool) - currently tracking player
- Target entity (optional) - current target

**Operations**

`check_aggro(enemy_pos, player_pos, range)` - Test detection

- Calculate distance between positions
- Return distance <= range

`acquire_target(player)` - Start tracking

- Set target to player entity
- Transition to chasing state

`lose_target()` - Stop tracking

- Clear target
- Transition to idle state

**Invariants**

- Only detects player (not other enemies)
- Aggro range is circular
- Once aggro'd, may have different lose conditions
- All enemies use same detection logic, different ranges

**Defaults**

| Enemy  | Aggro Range | Description |
| ------ | ----------- | ----------- |
| Grunt  | 400         | Standard    |
| Archer | 400         | Standard    |
| Dasher | 400         | Standard    |
| Brute  | 350         | Shorter     |

---

## Bevy Integration

**Components**

- AIConfig { aggro_range, attack_range, ... }

**Systems**

- Query player position
- Check each enemy distance to player
- Update AIState based on aggro

**Optimization**

- Spatial partitioning for many enemies (optional)
- Simple distance check sufficient for current scale
