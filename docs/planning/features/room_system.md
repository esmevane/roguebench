# Room System

Abstract room concept for discrete gameplay spaces and transitions.

## Core Logic

**Concept**

- World divided into "rooms"
- Rooms are discrete gameplay spaces
- Transitions move between rooms
- Room is design concept, flexible presentation

**Room Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique identifier |
| type | enum | Combat, shop, rest, puzzle, etc. |
| connections | list | Adjacent rooms |
| state | enum | Unexplored, active, cleared |
| contents | data | Enemies, items, features |
| layout | asset | Visual/spatial layout |

**Room Categories**

- Categories are extensible (designer-defined)
- Each category has different behavior
- Categories determine generation, contents, and rules

**Example Categories**

| Category | Behavior | Generation |
|----------|----------|------------|
| Arena | Combat encounter | `arena_generation.md` |
| Elite | Tough combat | Arena variant |
| Boss | Boss encounter | `boss_stages.md` |
| Shop | NPC merchant | Fixed layout |
| Treasure | Loot room | Simple generation |
| Rest | Safe zone | Fixed layout |
| Event | Story moment | Scripted |

*Note: "Arena" is the primary combat room category. See `arena_generation.md`.*

**Room State**

| State | Description |
|-------|-------------|
| Hidden | Not yet revealed |
| Revealed | Visible on map |
| Current | Player is here |
| Cleared | Completed |
| Locked | Requires key/condition |

**Operations**

`enter_room(room_id)` - Move to room

- Transition animation
- Load room contents
- Set as current

`exit_room()` - Leave current

- Cleanup current room
- Prepare for transition

`clear_room()` - Mark complete

- Set state to Cleared
- Unlock rewards/exits

`get_adjacent()` - List connections

- Return connected rooms
- For movement choices

`reveal_room(room_id)` - Show on map

- Make visible without entering

**Invariants**

- One current room at a time
- Transitions are discrete (not continuous)
- Room state persists within run
- Connections are bidirectional (or not, by design)

**Design Notes**

- Room generation left to design
- Room types left to design
- Transition style left to design

---

## Bevy Integration

**Resources**

- CurrentRoom(RoomId)
- RoomGraph { rooms: HashMap<RoomId, RoomData> }

**Components**

- Room { id, room_type, state, connections }
- RoomContents { enemies, items, features }

**Messages/Commands**

- EnterRoom { room_id }
- ExitRoom
- ClearRoom { room_id }
- RevealRoom { room_id }

**Events**

- RoomEntered { room_id, room_type }
- RoomExited { room_id }
- RoomCleared { room_id }
- RoomRevealed { room_id }

**Transitions**

- Fade, slide, or instant
- Loading during transition
- Spawn room contents on enter

**Scripting Compatibility**

- Room operations as commands
- Room state readable
- Events hookable

---

## Framework Dependencies

- `framework/command_bus.md` - Room operations as commands
- `framework/state_machine.md` - Room state transitions
- `framework/spawn_framework.md` - Room content spawning
- `framework/event_hooks.md` - Room enter/exit/clear hooks
- `framework/data_loading.md` - Room definitions from data

*See: architecture/scripting.md, architecture/data.md*
