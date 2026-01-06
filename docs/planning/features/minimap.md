# Minimap

Revealed area display showing explored rooms and current position.

## Core Logic

**Concept**

- Visual overview of room graph
- Shows explored and revealed rooms
- Current position highlighted
- Navigation aid for players

**Display Elements**

| Element | Representation |
|---------|----------------|
| Rooms | Shapes/icons by type |
| Connections | Lines between rooms |
| Current | Highlighted marker |
| Unexplored | Hidden or dimmed |
| Cleared | Checkmark or color |
| Special | Icons (shop, boss, etc.) |

**Room Icons**

- Shape or icon per room type
- Color indicates state
- Size may indicate importance

**Operations**

`update_map()` - Refresh display

- Reflect current room graph state
- Update positions and connections

`reveal_area(room_id)` - Show room

- Add room to visible map
- Show connections to known rooms

`set_marker(room_id, marker)` - Add marker

- Custom markers on rooms
- Player bookmarks, objectives

`center_on(room_id)` - Focus view

- Pan map to show room
- For larger maps

`toggle_fullscreen()` - Expand view

- Switch between mini and full map

**Invariants**

- Only revealed rooms shown
- Current room always visible
- Connections accurate to graph
- Updates reflect room state

**Design Notes**

- Visual style left to design
- Size and position left to UI design
- Interaction (click to travel?) left to design

---

## Bevy Integration

**Resources**

- MinimapState { visible_rooms, markers, centered_on }

**Components**

- MinimapNode - UI representation of room
- MinimapConnection - line between nodes

**UI**

- Corner overlay (minimap)
- Full-screen option (world map)
- Room icons
- Connection lines

**Messages/Commands**

- ToggleMinimap
- ToggleFullMap
- CenterMinimapOn { room_id }
- AddMinimapMarker { room_id, marker_type }

**Events**

- MinimapUpdated
- MinimapMarkerAdded { room_id }

**Systems**

- Listen for room reveals
- Update minimap nodes
- Sync with room graph
- Render connections

**Integration with Bookmarks**

- Bookmarked rooms show indicator
- Objective rooms highlighted
- Path hints optional

**Scripting Compatibility**

- Markers addable via commands
- Visible rooms readable
- Events hookable

*See: architecture/scripting.md*
