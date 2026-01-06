# Cartridge Ejection

Shell casing ejected when firing a gun. Adds tactile feedback and realism to ranged attacks.

## Core Logic

**Parameters**

- Position (vec2) - ejection port location
- Direction (vec2) - which way entity is facing
- Weapon type - determines casing appearance

**Behavior**

- Spawns at weapon ejection port (offset from entity)
- Ejects perpendicular to aim direction (typically up-right or up-left)
- Tumbles while flying (rotation)
- Affected by gravity (arcing trajectory)
- Bounces or settles on ground
- Fades and despawns after short time

**Invariants**

- One casing per shot
- Ejection side consistent (based on weapon or entity facing)
- Rotation adds visual interest
- Short lifetime (1-3 seconds)
- Can accumulate on ground briefly before despawn

**Defaults**

| Field         | Value      | Description               |
| ------------- | ---------- | ------------------------- |
| Eject speed   | 50-100     | Pixels per second         |
| Eject angle   | 45-90°     | From aim direction        |
| Gravity       | 200        | Downward acceleration     |
| Rotation      | 360-720°/s | Tumble speed              |
| Lifetime      | 2.0        | Seconds before despawn    |

---

## Bevy Integration

**Trigger**

- On projectile attack fired (gun weapons only)
- Position: entity position + weapon offset
- Direction: attack direction (for ejection angle calc)

**Spawning**

- Create small sprite entity
- Apply initial velocity perpendicular to aim
- Add gravity and rotation each frame

**Physics**

- Optional: bounce off ground/walls
- Or: just arc and fade

**Lifecycle**

- Self-manages lifetime and despawn
- No collision with entities (visual only)
