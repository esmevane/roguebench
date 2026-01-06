# Enemy: Dasher

Agile melee enemy that circles and darts at player. Medium health, fast movement, unpredictable patterns.

## Core Logic

**Stats**

- Health: 25
- Speed: 200 pixels/sec
- Damage: 12 (melee)
- Aggro range: 400 pixels
- Attack range: 40 pixels

**Behavior**

- Circles around player at medium distance
- Darts in for quick melee attacks
- Retreats after attacking
- Erratic movement pattern

**AI Pattern**

1. Idle → detect player → Circle
2. Circle → build up → Dash attack
3. Attack → hit or miss → Retreat
4. Retreat → reach safe distance → Circle
5. Repeat

**Invariants**

- Does not chase directly
- Moves perpendicular to player often
- Quick hit-and-run attacks
- Harder to hit than Grunt

**Defaults**

| Field        | Value | Description |
| ------------ | ----- | ----------- |
| Health       | 25    | Medium      |
| Speed        | 200   | Fast        |
| Melee damage | 12    | Lower       |
| Attack range | 40    | Very close  |
| Cooldown     | 0.8   | Fast attacks |

---

## Bevy Integration

**Components**

- Enemy marker
- EnemyType::Dasher
- AIState (includes Circling state)
- AIConfig
- Health, Team::Enemy
- WeaponInventory (melee only)

**Spawning**

- Spawned by wave system
- More common in mid-late waves
- Configured via enemies.ron

**Visual**

- Character sprite with directional animation
- Fast animation speed
