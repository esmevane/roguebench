# UI Framework

User interface layout, widgets, and interaction.

## Client-Side Execution

UI runs **entirely on the client**. It is not replicated.

- UI reads from replicated game state (health, inventory, etc.)
- UI input is local (menu navigation, button clicks)
- Gameplay actions triggered by UI go through normal input flow

```
Replicated Game State (from Lightyear)
    ↓
UI Framework (client-only)
    ↓ displays
Screen
    ↓ user interacts
UI Input (bevy_enhanced_input)
    ↓ gameplay actions
Lightyear input protocol → Server
```

---

## Input Integration

UI input uses **bevy_enhanced_input** for consistency with gameplay input.

```rust
// Define UI input actions
#[derive(InputAction)]
enum UiAction {
    Navigate,    // D-pad / arrows
    Confirm,     // A / Enter / Space
    Cancel,      // B / Escape
    TabNext,     // Tab
    TabPrev,     // Shift+Tab
}

// UI navigation is local, doesn't go through Lightyear
fn handle_ui_navigation(
    query: Query<&ActionState<UiAction>>,
    mut focus: ResMut<UiFocus>,
    focusables: Query<&UiFocusable>,
) {
    // ... navigation logic
}
```

**Important:** UI input does NOT go through Lightyear. It's local-only. When a UI action should affect gameplay (e.g., "use item"), it queues that as a gameplay input which then goes through Lightyear.

---

## Data Binding

UI binds to **replicated game state**. Don't bind to local predictions or client-only state for gameplay data.

```rust
#[derive(Component)]
struct UiBinding {
    /// Path to replicated data
    source: BindingSource,
}

enum BindingSource {
    PlayerHealth,        // From replicated Player component
    PlayerInventory,     // From replicated Inventory component
    WaveNumber,          // From replicated WaveState resource
    LocalSetting(String), // Local-only (volume, keybinds)
}

fn update_health_bar(
    player_query: Query<&Health, (With<LocalPlayer>, Changed<Health>)>,
    mut ui_query: Query<&mut ProgressBar, With<HealthBarUi>>,
) {
    // Health is replicated from server
    if let Ok(health) = player_query.get_single() {
        for mut bar in ui_query.iter_mut() {
            bar.value = health.current as f32 / health.max as f32;
        }
    }
}
```

---

## Core Logic

**Widget Types**

| Widget | Purpose |
|--------|---------|
| Container | Layout children |
| Text | Display text |
| Image | Display sprite |
| Button | Clickable action |
| Slider | Value selection |
| Toggle | Boolean switch |
| List | Scrollable items |
| Panel | Styled container |
| ProgressBar | Value visualization |

**Layout Modes**

| Mode | Behavior |
|------|----------|
| Row | Horizontal children |
| Column | Vertical children |
| Stack | Overlapping children |
| Grid | Row/column grid |
| Anchor | Position by anchor points |

**UI Layers**

| Layer | Z-Order | Use Case |
|-------|---------|----------|
| World | 0-10 | Health bars, names |
| HUD | 100-199 | Player stats, minimap |
| Menu | 200-299 | Pause, inventory |
| Overlay | 300-399 | Dialogue, notifications |
| System | 400+ | Loading, errors |

---

## Focus Management

```
Tab → Next Widget
Shift+Tab → Previous Widget
Arrow Keys → Directional Navigation
Enter/Space → Activate
Escape → Cancel/Back
```

**Focus state is local.** Server doesn't know or care which UI element is focused.

```rust
#[derive(Resource)]
struct UiFocus {
    current: Option<Entity>,
}

#[derive(Component)]
struct UiFocusable {
    nav_up: Option<Entity>,
    nav_down: Option<Entity>,
    nav_left: Option<Entity>,
    nav_right: Option<Entity>,
}
```

---

## Bevy Integration

**Components**

```rust
#[derive(Component)]
struct UiWidget {
    widget_type: WidgetType,
    style: UiStyle,
    focused: bool,
    disabled: bool,
}

#[derive(Component)]
struct UiButton {
    action: ButtonAction,
}

enum ButtonAction {
    OpenMenu(MenuId),
    CloseMenu,
    GameplayAction(GameplayInput), // Goes through Lightyear
    LocalAction(LocalAction),       // Stays client-side
}
```

**Button actions that affect gameplay:**

```rust
fn handle_button_activation(
    focus: Res<UiFocus>,
    buttons: Query<&UiButton>,
    mut gameplay_input: ResMut<InputBuffer>, // Lightyear input buffer
) {
    if let Some(focused) = focus.current {
        if let Ok(button) = buttons.get(focused) {
            match &button.action {
                ButtonAction::GameplayAction(input) => {
                    // Queue as gameplay input → goes through Lightyear
                    gameplay_input.queue(input.clone());
                }
                ButtonAction::LocalAction(action) => {
                    // Handle locally (settings, menu navigation)
                }
                // ...
            }
        }
    }
}
```

---

## Events

```rust
// Local UI events (not replicated)
enum UiEvent {
    FocusChanged { from: Option<Entity>, to: Option<Entity> },
    ButtonPressed { entity: Entity, action: ButtonAction },
    MenuOpened { menu_id: MenuId },
    MenuClosed { menu_id: MenuId },
}
```

---

## Accessibility

- All widgets have accessible names
- Keyboard fully navigable
- High contrast theme option
- Screen reader compatibility (stretch goal)

---

## Implementation Note

Consider using **bevy_ui** for layout and styling, or **bevy_egui** for debug/dev UI. The framework here describes patterns, not a specific library choice.

---

## What's Not Here

- Theming system details (add when implementing)
- Animation (UI transitions)
- Drag and drop

*See: architecture/localization.md (text keys), features/accessibility_*.md*
