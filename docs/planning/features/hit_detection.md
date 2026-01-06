# Hit Detection

Collision-based detection of attacks hitting targets. Uses physics overlap to detect hits.

## Core Logic

**Components**

- Collider shape (rectangle for melee, circle for projectile)
- Sensor flag (no physical response, just detection)
- Team (determines valid targets)

**Operations**

`check_collision(attack, target)` - Detect hit

- Physics engine reports collision
- Check teams are opposing
- Check not already hit (HitTracker)
- If valid, record hit and emit event

`mark_hit(attack, target)` - Record hit

- Add target to attack's HitTracker
- Prevents duplicate hits

**Invariants**

- Same target hit only once per attack
- Friendly fire prevented by team check
- Sensor colliders don't cause physics response
- Hit detection runs every physics frame

**Hit Tracking**

- Melee: tracks all hit entities, can hit multiple
- Projectile: typically despawns on first hit

---

## Bevy Integration

**Physics**

- Avian2D collision events
- CollisionEventsEnabled component
- Sensor component for attacks

**Events**

- CollisionStarted from physics
- Filtered to HitEvent for valid attack-target pairs

**Layers**

- PlayerAttack layer collides with Enemy, Wall
- EnemyAttack layer collides with Player, Wall

**Systems**

- Reads collision events
- Filters by layer and team
- Emits HitEvent for valid hits
