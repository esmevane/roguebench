# Event Hooks Framework

Extensible event system for scripting and mod integration.

## Server Authority

Hooks that affect gameplay run on **server only** in `FixedMain` schedule. This ensures deterministic execution synchronized with Lightyear ticks.

**Hook execution context:**

| Hook Type | Runs On | Schedule | Example |
|-----------|---------|----------|---------|
| Gameplay | Server | FixedMain | OnDamage, OnDeath |
| Visual | Client | Update | OnHitVisual (spawn particles) |
| Both | Both | Respective | OnSpawn (server: spawn, client: effect) |

Clients should not run gameplay-affecting hooks. They receive the results via Lightyear replication.

---

## Core Logic

**Concept**

- Game events fire at specific moments
- Scripts and mods can hook into events
- Hooks execute before/after core logic
- Enables content authoring without code changes

**Hook Types**

| Type | When | Use Case |
|------|------|----------|
| Pre | Before event processing | Validation, cancellation |
| Post | After event processing | Reactions, side effects |
| Replace | Instead of default | Override behavior |

**Hook Lifecycle**

```
Event Fired (server, FixedMain)
    ↓
Pre-Hooks (can cancel)
    ↓
Core Processing
    ↓
Post-Hooks
    ↓
Event Complete
    ↓
State replicated to clients
    ↓
Client visual hooks (if any)
```

**Operations**

`register_hook(event_type, hook)` - Add hook

- Store in priority order
- Multiple hooks per event

`unregister_hook(hook_id)` - Remove hook

- For runtime modification

`fire_event(event)` - Trigger event

- Execute pre-hooks
- Execute core (unless cancelled)
- Execute post-hooks

`cancel_event(event)` - Prevent processing

- From pre-hook only
- Core logic skipped

**Hookable Events**

| Category | Events | Authority |
|----------|--------|-----------|
| Combat | OnDamage, OnDeath, OnHeal | Server |
| Movement | OnDash, OnKnockback | Server |
| Inventory | OnItemPickup, OnItemUse | Server |
| Progression | OnLevelUp, OnUnlock | Server |
| World | OnRoomEnter, OnSpawn | Server |
| Visual | OnHitEffect, OnDeathEffect | Client |

**Invariants**

- Hooks execute in priority order
- Cancelled events don't process
- Hooks cannot modify event order mid-execution
- Hook errors logged but don't crash
- Gameplay hooks run in FixedMain only

---

## Bevy Integration

**Resources**

```rust
#[derive(Resource)]
struct HookRegistry {
    hooks: HashMap<TypeId, Vec<Hook>>,
}
```

**Hook Definition**

```rust
struct Hook {
    id: HookId,
    hook_type: HookType,
    priority: i32,
    handler: Box<dyn Fn(&mut HookContext) + Send + Sync>,
}

struct HookContext<'a, E> {
    event: &'a E,
    world: &'a World,
    cancelled: bool,
}
```

**System Scheduling**

```rust
// Gameplay hooks run in FixedMain (server)
app.add_systems(FixedMain, process_gameplay_hooks);

// Visual hooks run in Update (client)
app.add_systems(Update, process_visual_hooks);
```

**Event Processing**

```rust
fn process_with_hooks<E: GameEvent>(
    event: E,
    registry: &HookRegistry,
    world: &mut World,
) -> EventResult {
    let mut context = HookContext::new(&event, world);

    // Pre-hooks
    for hook in registry.get_pre_hooks::<E>() {
        (hook.handler)(&mut context);
        if context.cancelled {
            return EventResult::Cancelled;
        }
    }

    // Core processing
    let result = event.process(world);

    // Post-hooks
    for hook in registry.get_post_hooks::<E>() {
        (hook.handler)(&mut context);
    }

    EventResult::Processed(result)
}
```

---

## Scripting Integration

Scripts register hooks at load time:

```rust
// Example: script reduces boss damage
hooks.register("OnDamage", HookType::Pre, |ctx| {
    if ctx.event.target.has_component::<Boss>() {
        ctx.event.amount *= 0.5; // Bosses take half damage
    }
});
```

Scripts cannot bypass server authority. All script-registered hooks run on server during FixedMain.

---

## Client-Side Visual Hooks

For visual feedback, clients observe replicated state changes:

```rust
// Client-only system observes damage events
fn on_damage_visual(
    mut events: EventReader<DamageApplied>, // Replicated event
    mut commands: Commands,
) {
    for event in events.read() {
        // Spawn hit particle locally
        spawn_hit_particle(&mut commands, event.position);
    }
}
```

This is technically not a "hook" in the scripting sense—it's a regular Bevy system observing replicated events.

---

## What's Not Here

- Hook persistence (runtime only for now)
- Hook debugging/tracing tools
- Hook sandboxing for untrusted mods

*See: architecture/scripting.md, framework/command_bus.md*
