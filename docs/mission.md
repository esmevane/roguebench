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

## Example: What Success Looks Like

If the workflow is "Build Enemies", then it is minimally viable when a non-programmer can:

1. **Define a new enemy type** — Set stats, choose behavior, see it spawn
2. **Create a room layout** — Place tiles, set spawns, play through it
3. **Script a simple behavior** — Enemy patrols, door opens on trigger
4. **Save and resume** — Work persists across sessions

All of these together are a vertical slice: editor → data → runtime → persistence → verification.

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

## Guiding Principles

### Authoring Playable Experiences

Every architectural decision should ask: "Does this help someone create playable content?"

Gameplay features matter only insofar as they can be authored. A combat system that's fun but hardcoded is less valuable than a simpler system that's data-driven.

### Functional Over Fancy

The editor should work, not impress. In early stages it is more important to get things working, before polishing.

### Frameworks Before Features

Features which need infrastructure require framework infrastructure to be built first. Identify the infrastructure and build it out for the first appearance of each new kind of feature.

### Vertical Over Horizontal

This is a networked system and involves many moving parts. All workflows must cross each system end-to-end in order to be considered ready. Completing workflows fully across all boundaries is more important than partial progress in one layer. 

### Fast Feedback

Prioritize choices that lead to quick feedback loops, always looking for options to enhance quicker feedback loops and enable user iteration through use of the system.
