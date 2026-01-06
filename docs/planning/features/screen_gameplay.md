# Gameplay Screen

Main game screen where combat and waves occur. Contains all gameplay systems.

## Core Logic

**Sub-states**

- Setup - initialize arena, spawn player
- Combat - active gameplay
- Paused - game frozen
- Victory - all waves complete
- GameOver - player died

**Operations**

`enter()` - Start gameplay

- Generate arena
- Spawn player
- Initialize wave system
- Start first wave

`update()` - Run gameplay

- Process input
- Run simulation
- Check win/lose conditions

`pause()` / `resume()` - Toggle pause

`exit()` - Leave gameplay

- Despawn game entities
- Return to title

**Invariants**

- All gameplay occurs in this screen
- Entities despawned on exit
- Sub-states control flow

---

## Bevy Integration

**Screen State**

- Screen::Gameplay variant

**Entity Cleanup**

- GameEntity marker on all gameplay entities
- DespawnOnExit(Screen::Gameplay) for auto-cleanup

**Systems**

- All gameplay systems run_if in_state(Screen::Gameplay)
- Pausable systems in PausableSystems set

**State Transitions**

- Victory → could go to reward screen
- GameOver → could go to game over screen
- Pause → overlay, not screen change
