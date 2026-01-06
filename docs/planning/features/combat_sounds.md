# Combat Sounds

Audio feedback for combat actions. Plays sounds on hits, attacks, and other combat events.

## Core Logic

**Sound Types**

- Hit sounds - when attacks connect
- Critical hit sounds - on critical damage
- Attack sounds - when attacks are performed
- Death sounds - when entities die

**Operations**

`play_hit()` - Play hit sound

- Select random variant
- Play at combat volume

`play_crit()` - Play critical hit sound

- Select from crit variants
- Louder/more impactful

**Invariants**

- Sounds don't overlap excessively
- Volume balanced with music
- Random variation prevents repetition
- Spatial audio (optional)

**Defaults**

| Sound      | Variants | Volume | Description |
| ---------- | -------- | ------ | ----------- |
| Hit        | 10       | 0.3    | A-J variants |
| Crit       | 5        | 0.5    | Combo sounds |

---

## Bevy Integration

**Resources**

- AudioAssets { hit_sounds, crit_sounds, ... }

**Events**

- Listen to HitEvent, DamageEvent
- Play appropriate sound

**Systems**

- play_hit_sounds on HitEvent
- Queries AudioAssets for handles
- Spawns AudioBundle

**Asset Loading**

- Loaded during Loading screen
- Verified before gameplay
