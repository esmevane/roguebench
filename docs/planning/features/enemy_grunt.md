# Enemy: Grunt

Basic melee enemy that charges directly at the player. Low health, moderate damage, aggressive behavior.

## Core Logic

**Stats**

- Health: 30
- Speed: 150 pixels/sec
- Damage: 15 (melee)
- Aggro range: 400 pixels
- Attack range: 50 pixels

**Behavior**

- Idles until player in aggro range
- Chases player directly when aggro'd
- Attacks when within melee range
- Continues chasing while attacking

**AI Pattern**

1. Idle → detect player → Chase
2. Chase → reach attack range → Attack
3. Attack → cooldown → continue Chase
4. Player leaves aggro → return to Idle

**Invariants**

- Always approaches player (no retreat)
- Simple direct pathfinding
- Attacks continuously when in range
- No special abilities

**Defaults**

| Field        | Value | Description |
| ------------ | ----- | ----------- |
| Health       | 30    |             |
| Speed        | 150   | Slow        |
| Melee damage | 15    |             |
| Attack range | 50    | Close       |
| Cooldown     | 1.0   | Slow attacks |

---

## Bevy Integration

**Components**

- Enemy marker
- EnemyType::Grunt
- AIState, AIConfig
- Health, Team::Enemy
- WeaponInventory (melee only)

**Spawning**

- Spawned by wave system
- Random position in arena
- Configured via enemies.ron

**Visual**

- Character sprite with directional animation
- Color tint for identification

---

## Framework Dependencies

- `framework/state_machine.md` - AI state (Idle, Chase, Attack)
- `framework/movement_framework.md` - Chase movement
- `framework/spawn_framework.md` - Wave spawning
- `framework/timer_framework.md` - Attack cooldowns
- `framework/data_loading.md` - enemies.ron configuration
- `features/stat_system.md` - Health and damage stats

*See: architecture/data.md, architecture/scripting.md*
