# Team System

Identifies which side entities belong to. Prevents friendly fire and determines valid targets.

## Core Logic

**Teams**

- Player - player and player attacks
- Enemy - enemies and enemy attacks

**Operations**

`is_hostile(team_a, team_b)` - Check if teams oppose

- Player hostile to Enemy
- Enemy hostile to Player
- Same team not hostile

`can_damage(attacker_team, target_team)` - Check valid damage

- Return is_hostile(attacker_team, target_team)

**Invariants**

- All damageable entities have a Team
- Attacks inherit team from spawner
- Friendly fire not possible
- Neutral entities (walls) have no team

**Team Assignment**

| Entity        | Team   |
| ------------- | ------ |
| Player        | Player |
| Player melee  | Player |
| Player bullet | Player |
| Enemy         | Enemy  |
| Enemy melee   | Enemy  |
| Enemy bullet  | Enemy  |

---

## Bevy Integration

**Component**

- Team enum { Player, Enemy }

**Usage**

- Hit detection checks opposing teams
- AI targets entities of opposing team
- Collision layers also separate by team

**Spawning**

- Player spawned with Team::Player
- Enemies spawned with Team::Enemy
- Attacks copy team from spawning entity
