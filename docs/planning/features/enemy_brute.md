# Enemy: Brute

Heavy melee enemy with high health and damage. Slow but dangerous, tank-like behavior.

## Core Logic

**Stats**

- Health: 80
- Speed: 80 pixels/sec
- Damage: 30 (melee)
- Aggro range: 350 pixels
- Attack range: 60 pixels

**Behavior**

- Slow, deliberate approach
- High damage melee attacks
- Does not retreat
- Absorbs significant damage

**AI Pattern**

1. Idle → detect player → slow Chase
2. Chase → reach attack range → Attack
3. Attack → cooldown → continue Chase
4. Never retreats

**Invariants**

- Slowest enemy type
- Highest health pool
- Highest melee damage
- Simple but threatening

**Defaults**

| Field        | Value | Description   |
| ------------ | ----- | ------------- |
| Health       | 80    | Tank          |
| Speed        | 80    | Very slow     |
| Melee damage | 30    | High          |
| Attack range | 60    | Medium reach  |
| Cooldown     | 1.5   | Slow but heavy |

---

## Bevy Integration

**Components**

- Enemy marker
- EnemyType::Brute
- AIState, AIConfig
- Health, Team::Enemy
- WeaponInventory (melee only)

**Spawning**

- Spawned by wave system
- Rare in early waves, more common later
- Configured via enemies.ron

**Visual**

- Character sprite with directional animation
- Larger collision size
- Distinct color (darker/heavier)
