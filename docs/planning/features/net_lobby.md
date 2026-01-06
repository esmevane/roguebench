# Network Lobby

Pre-game gathering space for players to join before starting a session.

## Core Logic

**State**

- Lobby ID
- Host player
- Player list with ready states
- Lobby settings

**Player Slot**

- Player ID
- Display name
- Ready state (bool)
- Character selection (optional)

**Operations**

`create_lobby(settings)` - Host creates lobby

- Generate lobby ID
- Host becomes first player
- Initialize settings

`join_lobby(id)` - Client joins lobby

- Validate lobby exists and has space
- Add player to list
- Sync current state to joiner

`leave_lobby()` - Player leaves

- Remove from player list
- If host leaves, migrate or close
- Notify other players

`set_ready(ready)` - Toggle ready state

- Update player ready state
- Check if all ready

`start_game()` - Host initiates game

- Require all players ready
- Transition to gameplay
- Spawn all players

**Settings**

| Setting | Type | Default |
|---------|------|---------|
| Max players | int | 4 |
| Friendly fire | bool | false |
| Difficulty | enum | Normal |
| Private | bool | false |

**Invariants**

- Only host can start game
- All players must be ready to start
- Host migration if host disconnects (optional)
- Lobby closes if empty

---

## Bevy Integration

**States**

- Screen::Lobby or MenuState::Lobby
- LobbyState { Creating, Joining, InLobby, Starting }

**Resources**

- CurrentLobby { id, settings, players }
- LocalPlayerId

**Messages**

- CreateLobby { settings }
- JoinLobby { id }
- LeaveLobby
- SetReady { ready }
- StartGame
- LobbyUpdate { players, settings }

**Systems**

- Handle lobby state machine
- Sync player list
- Render lobby UI
- Process ready checks

**UI**

- Player list with ready indicators
- Settings panel (host only)
- Start button (host only, enabled when all ready)
- Leave button
- Chat (optional)
