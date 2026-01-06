# Mission

## Goal

Build a game workbench for collaborative roguelike creation with non-programmers.

This is not a game. This is a tool for making games together.

## Users

### Primary: The Kids

Non-technical collaborators who want to create game content without writing Rust.

**Capabilities:**
- Can navigate web interfaces (forms, buttons, dropdowns)
- Can understand game concepts (health, damage, speed)
- Can write simple structured text if given examples
- Can iterate through trial and error with fast feedback

**Needs:**
- See results quickly (hot reload, not rebuild)
- Understand what went wrong (clear error messages)
- Experiment safely (can't break the system)
- Create meaningful things (not just tweak numbers)

### Secondary: The Developer

Full technical capability. Builds the workbench, extends systems, debugs issues.

**Capabilities:**
- Rust, Bevy, systems programming
- Can read and write any part of the codebase
- Can extend frameworks and add new content types

**Needs:**
- Clean architecture for maintainability
- Good observability for debugging
- Patterns that scale as complexity grows

## What Success Looks Like

The workbench is minimally viable when a non-programmer can:

1. **Define a new enemy type** — Set stats, choose behavior, see it spawn
2. **Create a room layout** — Place tiles, set spawns, play through it
3. **Script a simple behavior** — Enemy patrols, door opens on trigger
4. **Save and resume** — Work persists across sessions

Each of these is a vertical slice: editor → data → runtime → persistence → verification.

## Reference Points

These are the experiences that inform the vision. Each offers something different:

### Minecraft (Creative Mode)

**What it does well:**
- Immediate, tangible creation (place block, see block)
- Shared worlds where collaborators see each other's work
- Simple primitives that combine into complex creations
- No code required for basic creation

**What we take:** The immediacy. Changes should be visible fast. Collaboration should feel live.

### Roblox Studio

**What it does well:**
- Full game creation, not just levels
- Scripting for custom behavior (Lua)
- Publish and share creations
- Gradual complexity (start simple, learn scripting over time)

**What we take:** The scripting layer. Users start with data-driven content, graduate to scripts when ready.

### Super Mario Maker

**What it does well:**
- Constrained but expressive (limited palette, infinite combinations)
- Play-test loop is instant (edit, play, edit)
- Doesn't require understanding game internals
- Focused scope (levels, not systems)

**What we take:** The constraints. We're making roguelikes, not everything. The focus enables depth.

### What We're Not

Unlike these references, we're:
- **Not a platform** — This is for us, not the public
- **Not a general engine** — Roguelikes specifically
- **Not visual-programming** — Scripts are text (but simple text)
- **Not asset creation** — Sprites and audio come from elsewhere

## What This Is Not

- **Not a moddable game** — No mod distribution, workshop, or public API (yet)
- **Not a visual programming tool** — Scripts are text, not node graphs
- **Not an asset creation tool** — Sprites and audio come from elsewhere
- **Not multiplayer-first** — Networking exists but isn't the primary concern

## Guiding Principles

### Authoring Over Playing

Every architectural decision should ask: "Does this help someone create content?"

Gameplay features matter only insofar as they can be authored. A combat system that's fun but hardcoded is less valuable than a simpler system that's data-driven.

### Functional Over Fancy

The editor should work, not impress. Forms over canvases. Text fields over drag handles. A working ugly thing beats a polished incomplete thing.

### Frameworks Before Features

Features that need infrastructure should wait for that infrastructure. Building features without their foundations creates debt that blocks future work.

### Vertical Over Horizontal

Completing one content type end-to-end is more valuable than partial progress on many. A user who can define items (but not enemies) can still create. A user with half-built systems for everything can create nothing.

### Fast Feedback Over Correctness

Hot reload, runtime errors with context, immediate visual feedback. Users learn through iteration, not documentation.
