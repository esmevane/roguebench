# Physics Bodies

Rigid body types and configurations for entities. Determines how entities interact physically.

## Core Logic

**Body Types**

- Dynamic - affected by forces (player, enemies, cadavers)
- Kinematic - manual movement, no forces (attacks)
- Static - immovable (walls)

**Properties**

- Linear velocity - movement speed
- Linear damping - friction/drag
- Locked axes - prevent rotation
- Collider shape - collision geometry

**Entity Configurations**

| Entity     | Body     | Shape     | Damping |
| ---------- | -------- | --------- | ------- |
| Player     | Dynamic  | Circle    | 8.0     |
| Enemy      | Dynamic  | Circle    | 5.0     |
| Projectile | Kinematic| Circle    | 0.0     |
| Melee      | Kinematic| Rectangle | 0.0     |
| Wall       | Static   | Rectangle | N/A     |
| Cadaver    | Dynamic  | Circle    | 8.0     |

**Operations**

`apply_velocity(entity, velocity)` - Set movement

- For Dynamic: set LinearVelocity
- For Kinematic: move manually

**Invariants**

- All entities have rotation locked
- No gravity (top-down game)
- Damping stops movement when input ends
- Collider size matches visual size

---

## Bevy Integration

**Components**

- RigidBody enum { Dynamic, Kinematic, Static }
- Collider { shape }
- LinearVelocity(Vec2)
- LinearDamping(f32)
- LockedAxes::ROTATION_LOCKED

**Physics Engine**

- Avian2D
- Length unit: 100 pixels = 1 meter
- Gravity: (0, 0)

**Spawning**

- Each entity type has physics bundle
- Configured at spawn time
