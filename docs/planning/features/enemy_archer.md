# Enemy: Archer

Ranged enemy that keeps distance and fires projectiles. Low health, prefers to stay away from player.

## Core Logic

**Stats**

- Health: 20
- Speed: 120 pixels/sec
- Damage: 10 (ranged)
- Aggro range: 400 pixels
- Attack range: 200-300 pixels (preferred)

**Behavior**

- Idles until player in aggro range
- Maintains preferred distance from player
- Stops moving to fire projectiles
- Retreats if player gets too close

**AI Pattern**

1. Idle → detect player → position for attack
2. Too far → approach to attack range
3. Too close → retreat to safe distance
4. At range → stop, aim, fire
5. After firing → reposition

**Invariants**

- Prefers ranged combat
- Stops to attack (no moving shots)
- Retreats when player approaches
- Fragile (lowest health)

**Defaults**

| Field         | Value | Description    |
| ------------- | ----- | -------------- |
| Health        | 20    | Fragile        |
| Speed         | 120   | Slow           |
| Ranged damage | 10    |                |
| Preferred range | 250 | Optimal distance |
| Cooldown      | 1.5   | Slow fire rate |

---

## Bevy Integration

**Components**

- Enemy marker
- EnemyType::Archer
- AIState, AIConfig
- Health, Team::Enemy
- WeaponInventory (ranged only)

**Spawning**

- Spawned by wave system
- Tends to appear in later waves
- Configured via enemies.ron

**Visual**

- Character sprite with directional animation
- Different color than Grunt
