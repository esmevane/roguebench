# Glossary

Precise definitions for terms used throughout this project. When these terms appear in documentation or conversation, they mean exactly what's defined here.

---

## Architectural Terms

### Walking Skeleton

The thinnest possible end-to-end implementation of a workflow.

**Purpose:**
- Proves the integration path works before investing in depth
- Exposes missing pieces and wrong assumptions early
- Creates a working system that can be fleshed out incrementally

**Characteristics:**
- Touches all layers (editor → API → storage → runtime)
- Does one thing, poorly but completely
- Is ugly, minimal, maybe hardcoded
- Actually runs and can be demonstrated

**Example walking skeleton:** "Author creates enemy that appears in game"
- Editor: Single text field for enemy name
- API: One POST endpoint, minimal validation
- Storage: One row in SQLite
- Runtime: Spawn a colored rectangle with a label
- Hot reload: Change name → rectangle label updates

**Not a skeleton:** Building the full enemy form, then the full API, then the full runtime. That's horizontal layers, not a skeleton.

**Process:**
1. Identify the workflow to prove
2. Build the thinnest path through all layers
3. Verify it works end-to-end
4. Flesh out each layer incrementally
5. Each increment should still work end-to-end

### Vertical Slice

A complete path through all system layers for one capability.

**Includes:**
- Editor UI (how users create/edit)
- Data Schema (how it's structured and validated)
- Runtime Behavior (how it works in-game)
- Persistence (how it's saved and loaded)
- Verification (how we know it works)

**Example vertical slice:** "Define custom item"
- Editor: Form to create item definition
- Schema: Item data structure with validation
- Runtime: Item spawns with defined properties
- Persistence: Definition survives restart
- Verification: Test confirms authored item appears correctly

**Not a vertical slice:** "Complete combat system" — this is horizontal (one layer, many features).

### Horizontal Layer

A single architectural layer across many features.

**Examples:**
- All editor UI (but no runtime)
- All networking code (but no gameplay)
- All particle effects (but no integration)

Horizontal work feels productive but doesn't create user capability until vertical slices connect the layers.

### Framework

A reusable abstraction that multiple features depend on.

**Characteristics:**
- Provides infrastructure, not behavior
- Multiple features build on top of it
- Must exist before dependent features can be built
- Changes to it affect all dependents

**Examples:**
- Command Bus (enables: scripting, replay, network sync, testing)
- State Machine (enables: enemy AI, player states, animation)
- Data Pipeline (enables: hot reload, editor integration, validation)

### Feature

A user-visible capability built on top of frameworks.

**Characteristics:**
- Provides specific behavior
- Built using framework infrastructure
- Can be authored/configured by users
- Has a vertical slice (editor → runtime → persistence)

**Examples:**
- Enemy type "Grunt" (uses: state machine, spawning, combat)
- Dash ability (uses: input, physics, animation)
- Room "Tutorial Arena" (uses: tilemap, spawning, collision)

### TBD (To Be Decided)

A decision that blocks implementation.

**Rules:**
- TBDs must be resolved before dependent work begins
- Resolving a TBD means choosing an option and documenting why
- Working around a TBD (instead of resolving it) creates debt

**Examples:**
- "Scripting language: Lua vs Rhai vs WASM" — blocks all scripting work
- "Entity ID scheme" — blocks persistence and networking identity

### Deferral

Postponing a decision or task to a later time.

**This is an anti-pattern.** When you feel the urge to defer:
1. Stop
2. Identify what tension is causing the deferral
3. Surface it for discussion
4. Either resolve it now or explicitly acknowledge the cost

Implicit deferral (just not doing something) is worse than explicit deferral (documenting why it's postponed).

---

## Role Terms

### Director

Coordinates across the entire project. Interacts with Leads. Responsible for overall prioritization and coherence.

**Asks:** "What should we be working on? Are the pieces fitting together?"

### Lead

Owns a vertical slice of functionality. Coordinates multiple concerns to deliver a complete capability.

**Asks:** "How do we get this feature working end-to-end?"

**Examples:** Multiplayer Lead, Content Authoring Lead

### Advocate

Owns a horizontal layer. Ensures that layer works well and isn't compromised unfairly by feature demands.

**Asks:** "Is this layer healthy? Are we accumulating debt here?"

**Examples:** Dev Tools Advocate, Editor Advocate, Game Server Advocate

### Architect

Observes patterns across the codebase. Identifies emerging abstractions, inconsistencies, and structural issues.

**Asks:** "What patterns are forming? What should be extracted? What's becoming inconsistent?"

---

## Content Terms

### Content Type

A category of authorable game element.

**Examples:** Items, Enemies, Rooms, Quests, Dialogues, Effects

Each content type needs:
- A data schema
- Editor UI for CRUD
- Runtime behavior
- Persistence support

### Template

A reusable definition for spawning entities.

**Example:** An "Archer" enemy template defines health, speed, behavior. Spawning creates instances from the template.

### Instance

A specific entity in the game world, created from a template.

**Example:** The archer at position (100, 200) is an instance of the "Archer" template.

---

## Process Terms

### Hot Reload

Applying changes without restarting.

**Scope in this project:**
- Asset based (vs database-originated) content definitions (items, enemies, rooms) — should hot reload
- Scripts — should hot reload
- Rust code — requires rebuild (acceptable)

### Functional But Not Fancy

Design principle for the editor and tools.

**Means:**
- Forms, not canvases
- Text fields, not drag handles
- Working ugly over polished incomplete
- Browser-native UI, no complex frameworks

**Does not mean:**
- Unusable or hostile UI
- Requiring users to edit raw files
- No visual feedback

### Outside-In Testing

Testing from the user's perspective inward.

**Approach:**
1. Define what the user does (action)
2. Define what should happen (effect)
3. Write test asserting action → effect
4. Implementation details are hidden

**Example:** "User defines enemy with 50 health → enemy spawns with 50 health"
Not: "EnemyDefinition struct has health field set to 50"
