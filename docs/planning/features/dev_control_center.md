# Dev Control Center

Centralized GUI panel for development tools, system toggles, and state inspection.

## Core Logic

**Concept**

- Unified dev tools interface
- Visual toggles and controls
- State inspection
- Development builds only

**Panel Sections**

| Section | Contents |
|---------|----------|
| Gameplay | God mode, infinite currency, skip rooms |
| Spawning | Entity spawner, item dropper |
| Rendering | Layer toggles, debug viz |
| Physics | Collision viz, physics pause |
| Audio | Mute, volume overrides |
| Network | Connection status, lag simulation |
| Scripting | Script status, reload, errors |
| Time | Pause, slow-mo, speed up |

**Toggle Controls**

| Toggle | Effect |
|--------|--------|
| God mode | Player invincible |
| Infinite ammo | No resource consumption |
| No cooldowns | Instant abilities |
| Show colliders | Render collision shapes |
| Show AI | Render AI state/paths |
| Freeze AI | Enemies don't act |
| Free camera | Detach camera |

**Spawner Tools**

- Entity dropdown + spawn button
- Position picker (click in world)
- Quantity slider
- Variant selection

**Inspector**

- Select entity in world
- View/edit components
- Modify values live

**Operations**

`toggle_control(name)` - Flip toggle

- Change control state
- Apply effect immediately

`spawn_from_panel(definition)` - Spawn entity

- Use panel configuration

`inspect_entity(entity)` - Select for inspection

- Show components
- Enable editing

`register_section(name, contents)` - Extend panel

- Systems add their controls

**Invariants**

- Changes are temporary (dev session)
- Doesn't corrupt save data
- Dev builds only
- Extensible by systems

**Design Notes**

- Layout configurable
- Sections collapsible
- Consider docking/floating

---

## Bevy Integration

**Resources**

- ControlCenterState { visible, selected_entity, toggles }
- DevToggles { god_mode, infinite_ammo, ... }

**Conditional Compilation**

```rust
#[cfg(debug_assertions)]
app.add_plugins(ControlCenterPlugin);
```

**UI**

- egui or bevy_ui based
- Collapsible sections
- Input fields, sliders, dropdowns
- World entity picker

**Systems**

- Render control panel
- Apply toggles to gameplay
- Handle entity inspection
- Process spawning

**Extension Pattern**

```rust
fn register_physics_controls(mut center: ResMut<ControlCenter>) {
    center.add_section("Physics", |ui| {
        ui.toggle("Show colliders", &mut debug.show_colliders);
        ui.toggle("Pause physics", &mut physics.paused);
    });
}
```

**Integration Points**

- All systems can register controls
- Scripting can add controls
- Metrics display embedded
- Console accessible

**Scripting Compatibility**

- Controls can trigger scripts
- Script status visible
- Reload button

*See: dev_metrics.md, dev_console.md, debug_visualization.md*
