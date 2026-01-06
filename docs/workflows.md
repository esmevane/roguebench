# Workflows

How users accomplish tasks in the workbench. Each workflow describes the user journey, technical path, required skills, and what "functional but not fancy" looks like.

---

## Skeleton-First Approach

**Every workflow should be built as a walking skeleton first.**

Before fleshing out any workflow, build the thinnest possible end-to-end version:

1. **Identify the core loop** — What's the simplest version of "user does X → sees Y"?
2. **Stub every layer** — Editor (one field), API (one endpoint), Storage (one table), Runtime (one visible result)
3. **Prove the path** — Can you demonstrate the workflow working, even if ugly?
4. **Then flesh out** — Add fields, validation, polish—but keep it working at each step

**Why skeleton-first:**
- Exposes integration issues immediately (not after building rich layers)
- Validates assumptions about how layers connect
- Creates a working system from day one
- Each increment is demonstrable progress

**Anti-pattern:** Building the "full" editor UI, then the "full" API, then the "full" runtime. This hides integration problems until the end and produces nothing usable until everything is done.

---

## Content Authoring Workflows

### Create a New Enemy Type

**Walking Skeleton:**
```
Editor: Text field for name, number field for health
API: POST /api/enemies { name, health }
Storage: INSERT INTO enemies (id, name, health)
Runtime: Spawn colored rectangle with name label and health number
Hot reload: Change health → number updates on rectangle
```

**User Journey (full version):**
1. Open editor in browser (localhost:8080)
2. Navigate to Enemies → New
3. Fill form with stats and behavior
4. Click Save
5. Game hot-reloads, enemy available to spawn

**Editor Interface (Functional But Not Fancy):**
```
┌─────────────────────────────────────┐
│ New Enemy                           │
├─────────────────────────────────────┤
│ Name:        [________________]     │
│ Health:      [25    ]               │
│ Speed:       [100   ]               │
│ Damage:      [10    ]               │
│ Behavior:    [idle.script     ▼]    │
│                                     │
│ Sprite:      [grunt.png       ▼]    │
│ Color Tint:  [#ffffff]              │
│                                     │
│ [Save] [Cancel]                     │
└─────────────────────────────────────┘
```

**Technical Path:**
```
Editor UI
  → POST /api/enemies
  → enemies.ron updated
  → DataPipeline detects change
  → EnemyRegistry reloads
  → New type available for spawning
```

**User Skill Required:**
- Navigate web forms
- Understand game stats (health = hits to kill)
- Select from dropdown of existing behaviors

**Verification:**
- Test: Define enemy → spawn command → enemy appears with correct stats

---

### Create a Room Layout

**Walking Skeleton:**
```
Editor: 8x8 grid, click to toggle wall/floor
API: POST /api/rooms { tiles: [[0,1,1,...], ...] }
Storage: INSERT INTO rooms (id, tiles_json)
Runtime: Render grid of colored squares (brown=wall, gray=floor)
Hot reload: Change tile → square color updates
```

**User Journey (full version):**
1. Open editor in browser
2. Navigate to Rooms → New
3. Set room dimensions
4. Click tiles to place floor/walls
5. Click to place spawn points
6. Save and test in-game

**Editor Interface (Functional But Not Fancy):**
```
┌─────────────────────────────────────────────┐
│ Room: tutorial_arena                        │
├──────────────────────┬──────────────────────┤
│ Palette:             │ Grid:                │
│ [.] Floor            │ ████████████████     │
│ [#] Wall             │ █..............█     │
│ [P] Player Spawn     │ █..............█     │
│ [E] Enemy Spawn      │ █....P.........█     │
│                      │ █..............█     │
│ Size: 16 x 12        │ █.........E....█     │
│                      │ █..............█     │
│ [Save] [Test]        │ ████████████████     │
└──────────────────────┴──────────────────────┘
```

**Technical Path:**
```
Editor UI (click tiles)
  → PUT /api/rooms/{id}
  → rooms/{id}.ron updated
  → RoomLoader detects change
  → LoadRoom event fired
  → Room rebuilds in-game
```

**User Skill Required:**
- Click on grid cells
- Understand tile types (floor = walkable, wall = solid)
- Concept of spawn points

**Verification:**
- Test: Create room with specific layout → load room → layout matches

---

### Script a Simple Behavior

**Walking Skeleton:**
```
Editor: Textarea for Lua code
API: POST /api/scripts { name, code }
Storage: INSERT INTO scripts (id, code)
Runtime: Script's on_spawn() prints to console when enemy spawns
Hot reload: Change script → re-run on_spawn() for existing entities
```

**User Journey (full version):**
1. Open editor in browser
2. Navigate to Scripts → New
3. Write script (or copy/modify example)
4. Save
5. Assign script to enemy/trigger in their respective editors
6. Test in-game

**Editor Interface (Functional But Not Fancy):**
```
┌─────────────────────────────────────────────┐
│ Script: patrol.script                       │
├─────────────────────────────────────────────┤
│ -- Simple patrol behavior                   │
│ on spawn:                                   │
│   set patrol_point_a to self.position       │
│   set patrol_point_b to self.position + 100 │
│                                             │
│ on update:                                  │
│   if at patrol_point_a:                     │
│     move_toward patrol_point_b              │
│   else if at patrol_point_b:                │
│     move_toward patrol_point_a              │
│                                             │
│ [Save] [Validate] [Examples ▼]              │
└─────────────────────────────────────────────┘
```

Note: Actual script syntax depends on chosen scripting language (TBD: Lua/Rhai/other).

**Technical Path:**
```
Editor UI (text input)
  → PUT /api/scripts/{id}
  → Validate syntax
  → scripts/{id}.script saved
  → ScriptRuntime hot-reloads
  → Entities using script get new behavior
```

**User Skill Required:**
- Basic text editing
- Understand examples and modify them
- Concept of events (on spawn, on update)
- Simple conditionals (if/else)

**Verification:**
- Test: Create patrol script → assign to enemy → enemy patrols between points

---

### Define a New Item

**Walking Skeleton:**
```
Editor: Text field for name, dropdown for type (weapon/consumable/key)
API: POST /api/items { name, type }
Storage: INSERT INTO items (id, name, type)
Runtime: Spawn colored sprite based on type, show name on hover
Hot reload: Change name → hover text updates
```

**User Journey (full version):**
1. Open editor in browser
2. Navigate to Items → New
3. Fill form with item properties
4. Save
5. Item available in loot tables / spawn commands

**Editor Interface (Functional But Not Fancy):**
```
┌─────────────────────────────────────┐
│ New Item                            │
├─────────────────────────────────────┤
│ Name:        [Health Potion    ]    │
│ Type:        [Consumable      ▼]    │
│ Sprite:      [potion_red.png  ▼]    │
│                                     │
│ Effects:                            │
│ ┌─────────────────────────────────┐ │
│ │ [+] Add Effect                  │ │
│ │ • Heal: 25                [x]   │ │
│ └─────────────────────────────────┘ │
│                                     │
│ [Save] [Cancel]                     │
└─────────────────────────────────────┘
```

**Technical Path:**
```
Editor UI
  → POST /api/items
  → items.ron updated
  → ItemRegistry reloads
  → Item available for spawning/granting
```

**User Skill Required:**
- Navigate forms
- Understand item types (consumable, equipment, key)
- Add/remove effects from list

**Verification:**
- Test: Define item → spawn item → player picks up → effect applies

---

## Development Workflows

### Start a New Session

**Process:**
1. Pull latest changes
2. Check issue tracker for context (`/beads:ready`)
3. Consult mission-lead agent for current priorities
4. Identify work item
5. Begin with test or spike as appropriate

**Commands:**
```bash
git pull
claude
> /beads:ready
> Use mission-lead to assess current priorities
```

### Add a New Content Type

**Process:**
1. Define data schema (what fields, what types, what validation)
2. Add editor endpoints (CRUD)
3. Add editor UI (forms)
4. Add runtime loading (data pipeline integration)
5. Add runtime behavior (spawning, effects)
6. Add persistence (save/load)
7. Write verification tests

This is a vertical slice. Don't start the next content type until this one works end-to-end.

### Add a New Framework

**When:**
- Known pattern with a concrete first consumer, OR
- Pattern emerges from 2-3 similar implementations

**Process:**
1. Identify the consumer (what workflow will use this?)
2. Design the interface alongside that consumer
3. Build minimal implementation
4. Prove it works in the consumer workflow
5. If extracting: migrate remaining features
6. Document the framework

Build frameworks with consumers, not in isolation. See `docs/approach.md` for details.

### Debug an Issue

**Process:**
1. Write a test that reproduces the issue
2. Use the test to understand the boundaries
3. Fix the code (not the test)
4. Verify the test passes
5. Check for similar issues elsewhere

Don't debug through print statements. Debug through tests.

---

## Testing Workflows

### Test a New Feature

**Outside-In Approach:**
1. What does the user do? (action)
2. What should happen? (effect)
3. Write test: action → effect
4. Implementation is a black box

**Example:**
```rust
#[test]
fn enemy_spawns_with_defined_health() {
    // Arrange: Define enemy with 50 health via editor API
    let definition = EnemyDefinition { health: 50, .. };
    editor.create_enemy(definition);

    // Act: Spawn the enemy
    game.spawn_enemy("test_enemy");

    // Assert: Enemy has correct health
    let enemy = game.find_enemy("test_enemy");
    assert_eq!(enemy.health, 50);
}
```

### Test an Authoring Workflow

**End-to-End Approach:**
1. Simulate editor interaction
2. Verify runtime behavior
3. Verify persistence
4. Verify hot reload

**Example:**
```rust
#[test]
fn authored_room_loads_correctly() {
    // Create room via editor
    editor.create_room("test_room", room_data);

    // Load room in game
    game.load_room("test_room");

    // Verify layout matches
    assert_eq!(game.tile_at(0, 0), TileKind::Wall);
    assert_eq!(game.tile_at(5, 5), TileKind::Floor);

    // Modify and verify hot reload
    editor.update_room("test_room", modified_data);
    game.wait_for_reload();
    assert_eq!(game.tile_at(5, 5), TileKind::Wall);
}
```
