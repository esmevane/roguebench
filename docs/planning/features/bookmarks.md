# Bookmarks

Player-pinned objectives with visual indicators in the world.

## Core Logic

**Concept**

- Players can favorite/pin objectives
- Bookmarked items shown prominently
- World provides visual hints toward bookmarks
- Helps guide room/path selection

**Bookmark Types**

| Type | Source | Examples |
|------|--------|----------|
| Quest objective | Quest system | "Find the artifact" |
| Location | Map/world | "Return to town" |
| Item | Inventory/shop | "Find better sword" |
| Custom | Player-created | Personal notes |

**Bookmark Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique identifier |
| type | enum | Bookmark type |
| reference | string | What's bookmarked |
| label | string | Display text |
| priority | int | Sort order |

**Operations**

`add_bookmark(type, reference)` - Create bookmark

- Add to bookmark list
- Generate/use label
- Assign priority

`remove_bookmark(bookmark_id)` - Delete bookmark

- Remove from list
- Remove indicators

`reorder(bookmark_id, priority)` - Change priority

- Update sort order

`get_bookmarks()` - List all

- Return sorted by priority

`get_relevant(context)` - Query for context

- Return bookmarks relevant to current situation
- For room selection hints

**Invariants**

- Limited bookmark count (prevent clutter)
- Completed objectives auto-remove
- Bookmarks persist across sessions

**Design Notes**

- Max bookmarks left to design
- Indicator visuals left to design
- Room selection integration left to design

---

## Bevy Integration

**Resources**

- PlayerBookmarks { bookmarks: Vec<Bookmark> }

**Components**

- BookmarkIndicator - on entities/locations with bookmarks

**Messages/Commands**

- AddBookmark { bookmark_type, reference }
- RemoveBookmark { bookmark_id }
- ReorderBookmark { bookmark_id, priority }

**Events**

- BookmarkAdded { bookmark }
- BookmarkRemoved { bookmark_id }
- BookmarkCompleted { bookmark_id } (auto-remove trigger)

**UI**

- Bookmark list in quest/map UI
- Star/pin icon on bookmarked items
- Highlighted in world

**World Indicators**

- Room selection shows bookmark relevance
- "This room contains bookmarked objective"
- Path highlighting toward bookmarked locations

**Scripting Compatibility**

- Bookmark operations as commands
- Bookmark list readable
- Events hookable

*See: architecture/scripting.md, architecture/data.md*
