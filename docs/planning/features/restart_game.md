# Restart Game

Reset game state to begin a new run. Cleans up current game and starts fresh.

## Core Logic

**Trigger**

- Player input from game over screen
- Retry button press

**Operations**

`restart()` - Reset game

- Despawn all gameplay entities
- Reset wave state
- Reset player stats
- Regenerate arena (optional)
- Respawn player
- Start wave 1

**Cleanup**

- Remove all enemies
- Remove all attacks
- Remove all cadavers
- Remove all particles
- Clear events

**Reset State**

- Wave counter to 1
- Player health to max
- All cooldowns cleared
- Score/stats reset (if tracked)

**Invariants**

- Complete fresh start
- No state bleeding between runs
- Arena may be same or regenerated
- All resources reset to defaults

---

## Bevy Integration

**Trigger Options**

1. Screen transition: GameOver → Gameplay
2. Button press on game over overlay
3. Key press (R for retry)

**Cleanup**

- DespawnOnExit handles entity cleanup
- Or manual query + despawn

**Implementation**

Option A: Exit to Title, re-enter Gameplay
Option B: Stay in Gameplay, reset in place

**Systems**

- restart_game system
- Queries GameEntity, despawns all
- Resets resources (WaveState, etc.)
- Re-runs gameplay setup
