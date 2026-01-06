# Approach

How to approach building this project: workflow-driven development with integrated frameworks.

---

## Core Principle: Workflows First

**Start from user capabilities, build end-to-end, prove integration early.**

The goal is always: "A user can do X." Work backward from that.

```
User capability → Walking skeleton → Flesh out → Verify → Next capability
```

Frameworks support workflows. Build them together, not in isolation.

---

## The Walking Skeleton Approach

For any new capability:

### 1. Identify the Workflow

What does the user do? What should they see?

Example: "Designer creates an enemy that appears in the game."

### 2. Build the Thinnest Path

Touch every layer, but barely:

| Layer | Skeleton Version |
|-------|------------------|
| Editor UI | One text field, one button |
| API | One endpoint, minimal validation |
| Storage | One table, one row |
| Runtime | One visible result (colored rectangle) |
| Hot Reload | Change propagates visibly |

### 3. Prove It Works

Can you demonstrate the workflow? Even if ugly?

- Open browser
- Type name
- Click save
- See rectangle appear in game
- Change name
- See rectangle update

### 4. Flesh Out Incrementally

Add depth while keeping everything working:

- Add health field → rectangle shows health number
- Add sprite selector → rectangle becomes sprite
- Add behavior dropdown → sprite moves
- Add validation → form shows errors
- Add more fields → form grows

### 5. Repeat

Next skeleton, next capability, each one building on proven integration.

---

## Frameworks and Workflows

Frameworks are valuable—they make repeated work easier. The key is building them *with* workflows, not *before* them.

### When to Build a Framework

| Situation | Approach |
|-----------|----------|
| **Known pattern, first use** | Build the framework alongside its first consumer |
| **Pattern emerges after 2-3 similar things** | Extract the framework, refactor prior work to use it |
| **Speculative, no concrete consumer** | Wait until you have a real use case |

### Good: Framework with Consumer

1. Start workflow A (editor → runtime for enemies)
2. Recognize: "This needs content loading, and items/rooms will too"
3. Build ContentRegistry framework as part of skeleton A
4. Skeleton A proves both the workflow AND the framework
5. Skeleton B (items) reuses the proven framework

### Risky: Framework in Isolation

1. Design ContentRegistry framework with imagined requirements
2. Build it without a consumer to prove integration
3. Build enemies, discover framework doesn't fit
4. Rework framework or work around it

**The difference:** Frameworks built with a consumer get tested immediately. Frameworks built in isolation carry hidden integration risk.

---

## Dependencies Still Matter

Some things genuinely depend on others:

| Dependency | Why |
|------------|-----|
| Storage before content | Content needs somewhere to live |
| Scripting before behaviors | Behaviors need a runtime |
| State machines before AI | AI needs states to transition between |

But these are **capabilities**, not phases:

- "Can we store content?" — Yes → Continue
- "Can scripts execute?" — Yes → Continue
- "Can state machines transition?" — Yes → Continue

If no, build the thinnest skeleton that proves the capability, then continue.

---

## Resolved Decisions

These decisions have been made. See `docs/decisions/` for full rationale.

| Decision | Choice | See |
|----------|--------|-----|
| Scripting language | mlua with Luau | decisions/scripting.md |
| Content storage | SQLite | decisions/storage.md |
| Networking | Lightyear | decisions/networking.md |

When building skeletons, use these choices. Don't revisit unless there's specific evidence they're wrong.

---

## Proven Patterns

These patterns emerged from prior work. See `docs/patterns/` for details.

| Pattern | Use When | See |
|---------|----------|-----|
| Command Bus | Game mutations that need logging/replay/scripting | patterns/command-bus.md |
| Content Registry | Loading authored content from SQLite | patterns/content-registry.md |
| State Machine | Data-driven entity behaviors | patterns/state-machine.md |

When building skeletons, use these patterns where they fit. But don't force them—if a skeleton doesn't need them, don't add them.

---

## How to Decide What's Next

1. **What workflow is incomplete?**
   - Is there a skeleton that isn't fleshed out?
   - Is there a workflow with no skeleton?

2. **What's the highest-impact incomplete workflow?**
   - Which capability would most help users?
   - See `docs/mission.md` for guidance

3. **What's the thinnest skeleton for that workflow?**
   - What's the smallest thing you can build and demonstrate?
   - Build that first

4. **If blocked, what's the blocker?**
   - Is it a missing capability? Build skeleton for that
   - Is it an unresolved decision? Resolve it
   - Is it unclear requirements? Clarify with user

---

## Anti-Patterns

### Deferred Integration

> "We'll connect the editor to the runtime later."

"Later" hides integration problems until they're expensive to fix. Prove integration early with skeletons.

### Horizontal Before Vertical

> "Let's build all the editor UI, then all the API, then all the runtime."

Each layer built in isolation. Integration breaks when they meet. Build thin vertical slices instead.

### Framework Without Consumer

> "Let's build the perfect content system before we have any content."

Frameworks built without a real consumer accumulate speculative requirements. Build frameworks alongside their first use case—the consumer proves the design.

---

## Quick Reference

| Question | Answer |
|----------|--------|
| What should I build? | The thinnest skeleton for the highest-impact workflow |
| When do I build a framework? | When you have a consumer to build it with—first use or pattern emergence |
| When is a skeleton "done"? | When you can demonstrate the workflow end-to-end |
| When do I flesh out? | When the skeleton is proven and you need more depth |
| What if I'm blocked? | Trace back to the root blocker; that's your actual work |
