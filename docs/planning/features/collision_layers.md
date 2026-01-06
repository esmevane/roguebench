# Collision Layers

Physics layer system controlling what can collide with what. Separates entity types for proper interaction.

## Core Logic

**Layers**

1. Default - unused base
2. Player - player entity
3. Enemy - enemy entities
4. PlayerAttack - player melee/projectiles
5. EnemyAttack - enemy melee/projectiles
6. Wall - arena boundaries
7. Pickup - collectible items
8. Cadaver - dead bodies

**Collision Matrix**

| Layer        | Collides With                      |
| ------------ | ---------------------------------- |
| Player       | Wall, EnemyAttack, Pickup, Cadaver |
| Enemy        | Wall, PlayerAttack, Cadaver        |
| PlayerAttack | Enemy, Wall                        |
| EnemyAttack  | Player, Wall                       |
| Wall         | All moving entities                |
| Pickup       | Player                             |
| Cadaver      | Player, Enemy, Wall, Cadaver       |

**Operations**

`get_layers(entity_type)` - Get layer config

- Return CollisionLayers for entity type
- Sets memberships and filters

**Invariants**

- Attacks only hit opposing team
- Walls block everything
- Entities don't collide with own attacks
- Cadavers pushable but don't block

---

## Bevy Integration

**Component**

- CollisionLayers { memberships, filters }

**Helper Functions**

- player_layers()
- enemy_layers()
- player_attack_layers()
- enemy_attack_layers()
- wall_layers()
- cadaver_layers()

**Physics Engine**

- Avian2D LayerMask
- Set on entity spawn
