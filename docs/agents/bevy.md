# On Bevy code

Specific to the Rust framework Bevy:

- Prefer a single concept to a single file
- Leverage plugins and data-driven approaches over free functions
- One file = one plugin (in many cases, not always!)
- Always read the docs
- Prefer singleton components (`Single<C>`) over resources

Emphasize message and event/observer driven designs:

- Messages for broader communication and command-like behavior or delayed cause/effect
- Events / observers for direct instruction to queried components
- If a component enables an observer, expose it with a custom method
- Provide access to custom method using extension traits / `EntityCommand` patterns

On testing:

- Use the Yarnspinner repo for example test harness setup.
- Test directly and through the app itself.
- Test discrete cause/effect, not underlying structure or component state.
- Leverage "wait until" loops with timeouts instead of repeated update calls

On observability:

- Instrument for observability across several planes
- TRACE and DEBUG logging with `trace!` and `debug!`: most common
- `info!` / `warn!` only when humans might need to know, be selective
- `error!` when issues occur
- avoid `eprintln!`/`println!`
- Observability should include visibile diagnostics / dev tools

Dev tools:

- Leverage the command palette pattern - all key operations should have a command, and that command should correspond to a message or event trigger
- Dev tools have a console and the console can display trace output with selectors

On reactivity:

- Observe reactive designs by replacing UI instead of updating values
- Identify discrete boundaries for UI replacement
- Leverage observers/events for bespoke reactivity needs

Useful boilerplate for new files:

Use this pattern when you're making a "local" plugin to power an "offical" boundary, I.E., for smaller plugins that might not need to be globally available:

```rs
mod module_name {
    use bevy::prelude::*;

    pub(crate) mod prelude {
        // Structs, etc go here - not plugin
    }
    
    pub(super) fn plugin(_app: &mut App) {}
    
}
```

Otherwise, if the package / plugin has an official identity or represents a boundary of some sort, use this pattern:

```rs
mod module_name {
    use bevy::prelude::*;

    pub(crate) mod prelude {
        // Structs, etc go here - not plugin
    }
    
    pub(crate) struct PackagePlugin;
    
    impl Plugin for PackagePlugin {
        fn build(_app: &mut App) {}
    }
}
```

## Bevy Architectural Requirements

### Command-Driven Architecture

All game behavior flows through a command bus:

1. **Actions**: User input, AI decisions, external events
2. **Commands**: Serializable intent, routed through the bus (mpsc-style firehose)
3. **Effects**: Plugins handle commands internally, dependencies stay hidden

This enables: replay, scripting, network sync, and uniform testability.

### Testing Philosophy

- Test workflows, not details: a full user-persona-driven story per test
- Test workflows by triggering actions and observing effects in test harnesses, not implementation details
- Dependencies (Lightyear, Avian, etc.) are "under the hood" - never in test plumbing
- Cross-client observation pattern: "Client A does X → Client B sees Y"
- If tests need to know about a dependency, the abstraction is leaking

### Compliance Check

Before any feature is complete, verify:

- [ ] All inputs flow through commands, not direct reads
- [ ] Tests assert cause/effect across network boundaries
- [ ] No dependency types appear in test assertions
- [ ] Dev tools can trigger the same behavior via console commands
- [ ] Test
- [ ] Scripting tools can trigger behavior via scripting hooks
