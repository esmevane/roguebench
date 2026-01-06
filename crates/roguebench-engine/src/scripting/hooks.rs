//! Hook dispatcher for game events.
//!
//! Dispatches Bevy events to Lua script handlers, allowing scripts
//! to react to and modify game behavior.

use bevy::prelude::*;
use mlua::IntoLua;

use super::{LuaItemDefinition, LuaStateTransition, ScriptError, ScriptLoader, ScriptRuntime};
use crate::items::{ItemRegistry, ItemUsed};
use crate::state_machine::StateChanged;

/// Result of dispatching a hook to scripts.
#[derive(Debug)]
pub struct HookDispatchResult {
    /// Number of modules that handled the event
    pub handlers_called: usize,
    /// Errors from handlers (module name, error)
    pub errors: Vec<(String, ScriptError)>,
}

impl HookDispatchResult {
    fn new() -> Self {
        Self {
            handlers_called: 0,
            errors: Vec::new(),
        }
    }
}

/// Dispatches game events to Lua script handlers.
pub struct HookDispatcher;

impl HookDispatcher {
    /// Dispatch an ItemUsed event to script handlers.
    pub fn dispatch_item_used(
        runtime: &ScriptRuntime,
        loader: &ScriptLoader,
        item_registry: &ItemRegistry,
        event: &ItemUsed,
    ) -> HookDispatchResult {
        let mut result = HookDispatchResult::new();
        let lua = runtime.lua();

        // Get the item definition to pass to scripts
        let item_def = match item_registry.get(&event.item_id) {
            Some(def) => def.clone(),
            None => return result,
        };

        // Create the event table for Lua
        let event_table = match create_item_used_event(lua, event, &item_def) {
            Ok(table) => table,
            Err(e) => {
                result.errors.push(("_system".to_string(), e));
                return result;
            }
        };

        // Find all modules with the handler and call them
        for module in loader.modules_with_handler(runtime, "on_item_used") {
            match call_handler(runtime, lua, module, "on_item_used", event_table.clone()) {
                Ok(_) => result.handlers_called += 1,
                Err(e) => result.errors.push((module.name.clone(), e)),
            }
        }

        result
    }

    /// Dispatch a generic event by name with a Lua table argument.
    pub fn dispatch_event(
        runtime: &ScriptRuntime,
        loader: &ScriptLoader,
        handler_name: &str,
        event_table: mlua::Table,
    ) -> HookDispatchResult {
        let mut result = HookDispatchResult::new();
        let lua = runtime.lua();

        for module in loader.modules_with_handler(runtime, handler_name) {
            match call_handler(runtime, lua, module, handler_name, event_table.clone()) {
                Ok(_) => result.handlers_called += 1,
                Err(e) => result.errors.push((module.name.clone(), e)),
            }
        }

        result
    }
}

fn create_item_used_event(
    lua: &mlua::Lua,
    event: &ItemUsed,
    item_def: &roguebench_core::items::ItemDefinition,
) -> Result<mlua::Table, ScriptError> {
    let table = lua.create_table()?;

    table.set("item_id", event.item_id.0.clone())?;
    table.set("user_entity", format!("{:?}", event.user_entity))?;
    table.set("item", LuaItemDefinition(item_def.clone()))?;

    Ok(table)
}

fn call_handler(
    runtime: &ScriptRuntime,
    lua: &mlua::Lua,
    module: &super::LoadedModule,
    handler_name: &str,
    args: impl IntoLua,
) -> Result<(), ScriptError> {
    let table = module.table(lua)?;

    if !runtime.module_has_handler(&table, handler_name) {
        return Ok(());
    }

    let func: mlua::Function = table.get(handler_name)?;
    func.call::<()>(args)?;
    Ok(())
}

/// Bevy system that dispatches ItemUsed events to scripts.
pub fn dispatch_item_used_hooks(
    runtime: Res<ScriptRuntime>,
    loader: Res<ScriptLoader>,
    item_registry: Res<ItemRegistry>,
    mut events: EventReader<ItemUsed>,
) {
    for event in events.read() {
        let result =
            HookDispatcher::dispatch_item_used(&runtime, &loader, &item_registry, event);

        if !result.errors.is_empty() {
            for (module, error) in &result.errors {
                warn!("Script hook error in '{}': {}", module, error);
            }
        }

        if result.handlers_called > 0 {
            debug!(
                "ItemUsed hook dispatched to {} handlers",
                result.handlers_called
            );
        }
    }
}

// --- State Machine Hooks ---

impl HookDispatcher {
    /// Dispatch a StateChanged event to script handlers.
    ///
    /// This calls two hooks:
    /// - `on_state_exit(event)` - called with the state being left
    /// - `on_state_enter(event)` - called with the state being entered
    ///
    /// The event table contains:
    /// - `entity` - string representation of the entity
    /// - `from` - the state ID being exited
    /// - `to` - the state ID being entered
    /// - `transition` - the full transition userdata
    /// - `time_in_previous` - time spent in the previous state
    pub fn dispatch_state_changed(
        runtime: &ScriptRuntime,
        loader: &ScriptLoader,
        event: &StateChanged,
    ) -> (HookDispatchResult, HookDispatchResult) {
        let lua = runtime.lua();

        // Create the event table for Lua
        let event_table = match create_state_changed_event(lua, event) {
            Ok(table) => table,
            Err(e) => {
                let mut exit_result = HookDispatchResult::new();
                let enter_result = HookDispatchResult::new();
                // Store error message since ScriptError isn't Clone
                exit_result
                    .errors
                    .push(("_system".to_string(), ScriptError::InvalidModule(e.to_string())));
                return (exit_result, enter_result);
            }
        };

        // Dispatch exit hook
        let exit_result =
            Self::dispatch_event(runtime, loader, "on_state_exit", event_table.clone());

        // Dispatch enter hook
        let enter_result = Self::dispatch_event(runtime, loader, "on_state_enter", event_table);

        (exit_result, enter_result)
    }
}

fn create_state_changed_event(
    lua: &mlua::Lua,
    event: &StateChanged,
) -> Result<mlua::Table, ScriptError> {
    let table = lua.create_table()?;

    table.set("entity", format!("{:?}", event.entity))?;
    table.set("from", event.transition.from.0.clone())?;
    table.set("to", event.transition.to.0.clone())?;
    table.set("time_in_previous", event.transition.time_in_previous)?;
    table.set(
        "transition",
        LuaStateTransition(event.transition.clone()),
    )?;

    Ok(table)
}

/// Bevy system that dispatches StateChanged events to scripts.
pub fn dispatch_state_changed_hooks(
    runtime: Res<ScriptRuntime>,
    loader: Res<ScriptLoader>,
    mut events: EventReader<StateChanged>,
) {
    for event in events.read() {
        let (exit_result, enter_result) =
            HookDispatcher::dispatch_state_changed(&runtime, &loader, event);

        // Log exit errors
        for (module, error) in &exit_result.errors {
            warn!("Script on_state_exit error in '{}': {}", module, error);
        }

        // Log enter errors
        for (module, error) in &enter_result.errors {
            warn!("Script on_state_enter error in '{}': {}", module, error);
        }

        let total_handlers = exit_result.handlers_called + enter_result.handlers_called;
        if total_handlers > 0 {
            debug!(
                "StateChanged ({} -> {}) dispatched to {} handlers",
                event.transition.from, event.transition.to, total_handlers
            );
        }
    }
}

/// Plugin that sets up script hook dispatching.
pub struct ScriptHooksPlugin;

impl Plugin for ScriptHooksPlugin {
    fn build(&self, app: &mut App) {
        // Add item used hooks if resources exist
        app.add_systems(
            Update,
            dispatch_item_used_hooks.run_if(
                resource_exists::<ScriptRuntime>
                    .and(resource_exists::<ScriptLoader>)
                    .and(resource_exists::<ItemRegistry>),
            ),
        );

        // Add state changed hooks if resources exist
        app.add_systems(
            Update,
            dispatch_state_changed_hooks.run_if(
                resource_exists::<ScriptRuntime>.and(resource_exists::<ScriptLoader>),
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roguebench_core::items::{ItemDefinition, ItemId, ItemType};
    use roguebench_core::state_machine::{StateId, StateTransition};

    fn setup_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_event::<ItemUsed>()
            .add_event::<StateChanged>()
            .insert_resource(ScriptRuntime::new().unwrap())
            .insert_resource(ScriptLoader::new("scripts"))
            .insert_resource(ItemRegistry::new());
        app
    }

    #[test]
    fn dispatch_item_used_no_handlers() {
        let runtime = ScriptRuntime::new().unwrap();
        let loader = ScriptLoader::new("scripts");
        let registry = ItemRegistry::new();

        let event = ItemUsed {
            item_id: ItemId::new("test"),
            user_entity: Entity::PLACEHOLDER,
        };

        let result = HookDispatcher::dispatch_item_used(&runtime, &loader, &registry, &event);
        assert_eq!(result.handlers_called, 0);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn dispatch_item_used_with_handler() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");
        let mut registry = ItemRegistry::new();

        // Register an item
        let potion = ItemDefinition::new("health_potion", "Health Potion", ItemType::Consumable);
        registry.insert(potion);

        // Load a script with the handler
        loader
            .load_string(
                &runtime,
                "item_handler",
                r#"
                local m = {}
                m.items_used = 0

                function m.on_item_used(event)
                    m.items_used = m.items_used + 1
                end

                return m
            "#,
            )
            .unwrap();

        let event = ItemUsed {
            item_id: ItemId::new("health_potion"),
            user_entity: Entity::PLACEHOLDER,
        };

        let result = HookDispatcher::dispatch_item_used(&runtime, &loader, &registry, &event);
        assert_eq!(result.handlers_called, 1);
        assert!(result.errors.is_empty());

        // Verify the handler was actually called
        let module = loader.get("item_handler").unwrap();
        let table = module.table(runtime.lua()).unwrap();
        let count: i32 = table.get("items_used").unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn dispatch_item_used_handler_error() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");
        let mut registry = ItemRegistry::new();

        let potion = ItemDefinition::new("health_potion", "Health Potion", ItemType::Consumable);
        registry.insert(potion);

        // Load a script with a broken handler
        loader
            .load_string(
                &runtime,
                "broken_handler",
                r#"
                local m = {}

                function m.on_item_used(event)
                    error("intentional error")
                end

                return m
            "#,
            )
            .unwrap();

        let event = ItemUsed {
            item_id: ItemId::new("health_potion"),
            user_entity: Entity::PLACEHOLDER,
        };

        let result = HookDispatcher::dispatch_item_used(&runtime, &loader, &registry, &event);
        assert_eq!(result.handlers_called, 0);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].0, "broken_handler");
    }

    #[test]
    fn dispatch_multiple_handlers() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");
        let mut registry = ItemRegistry::new();

        let potion = ItemDefinition::new("health_potion", "Health Potion", ItemType::Consumable);
        registry.insert(potion);

        // Load multiple scripts with handlers
        loader
            .load_string(
                &runtime,
                "handler_a",
                r#"
                local m = { called = false }
                function m.on_item_used(event)
                    m.called = true
                end
                return m
            "#,
            )
            .unwrap();

        loader
            .load_string(
                &runtime,
                "handler_b",
                r#"
                local m = { called = false }
                function m.on_item_used(event)
                    m.called = true
                end
                return m
            "#,
            )
            .unwrap();

        let event = ItemUsed {
            item_id: ItemId::new("health_potion"),
            user_entity: Entity::PLACEHOLDER,
        };

        let result = HookDispatcher::dispatch_item_used(&runtime, &loader, &registry, &event);
        assert_eq!(result.handlers_called, 2);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn dispatch_generic_event() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");

        loader
            .load_string(
                &runtime,
                "custom_handler",
                r#"
                local m = { data = nil }
                function m.on_custom_event(event)
                    m.data = event.value
                end
                return m
            "#,
            )
            .unwrap();

        let event_table = runtime.lua().create_table().unwrap();
        event_table.set("value", 42).unwrap();

        let result =
            HookDispatcher::dispatch_event(&runtime, &loader, "on_custom_event", event_table);
        assert_eq!(result.handlers_called, 1);

        // Verify handler received data
        let module = loader.get("custom_handler").unwrap();
        let table = module.table(runtime.lua()).unwrap();
        let data: i32 = table.get("data").unwrap();
        assert_eq!(data, 42);
    }

    #[test]
    fn item_used_event_contains_item_data() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");
        let mut registry = ItemRegistry::new();

        let potion = ItemDefinition::new("health_potion", "Health Potion", ItemType::Consumable)
            .with_value(50);
        registry.insert(potion);

        loader
            .load_string(
                &runtime,
                "data_checker",
                r#"
                local m = { item_name = nil, item_value = nil }
                function m.on_item_used(event)
                    m.item_name = event.item:name()
                    m.item_value = event.item:value()
                end
                return m
            "#,
            )
            .unwrap();

        let event = ItemUsed {
            item_id: ItemId::new("health_potion"),
            user_entity: Entity::PLACEHOLDER,
        };

        HookDispatcher::dispatch_item_used(&runtime, &loader, &registry, &event);

        let module = loader.get("data_checker").unwrap();
        let table = module.table(runtime.lua()).unwrap();
        let name: String = table.get("item_name").unwrap();
        let value: u32 = table.get("item_value").unwrap();
        assert_eq!(name, "Health Potion");
        assert_eq!(value, 50);
    }

    #[test]
    fn hooks_plugin_adds_system() {
        let mut app = setup_test_app();
        app.add_plugins(ScriptHooksPlugin);

        // Should not panic
        app.update();
    }

    // --- State Machine Hook Tests ---

    #[test]
    fn dispatch_state_changed_no_handlers() {
        let runtime = ScriptRuntime::new().unwrap();
        let loader = ScriptLoader::new("scripts");

        let event = StateChanged {
            entity: Entity::PLACEHOLDER,
            transition: StateTransition {
                from: StateId::new("idle"),
                to: StateId::new("active"),
                time_in_previous: 1.0,
            },
        };

        let (exit_result, enter_result) =
            HookDispatcher::dispatch_state_changed(&runtime, &loader, &event);

        assert_eq!(exit_result.handlers_called, 0);
        assert!(exit_result.errors.is_empty());
        assert_eq!(enter_result.handlers_called, 0);
        assert!(enter_result.errors.is_empty());
    }

    #[test]
    fn dispatch_state_changed_enter_handler() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");

        loader
            .load_string(
                &runtime,
                "enter_handler",
                r#"
                local m = {}
                m.entered_state = nil
                m.exited_state = nil

                function m.on_state_enter(event)
                    m.entered_state = event.to
                end

                function m.on_state_exit(event)
                    m.exited_state = event.from
                end

                return m
            "#,
            )
            .unwrap();

        let event = StateChanged {
            entity: Entity::PLACEHOLDER,
            transition: StateTransition {
                from: StateId::new("idle"),
                to: StateId::new("active"),
                time_in_previous: 1.5,
            },
        };

        let (exit_result, enter_result) =
            HookDispatcher::dispatch_state_changed(&runtime, &loader, &event);

        assert_eq!(exit_result.handlers_called, 1);
        assert_eq!(enter_result.handlers_called, 1);

        // Verify handler received data
        let module = loader.get("enter_handler").unwrap();
        let table = module.table(runtime.lua()).unwrap();
        let entered: String = table.get("entered_state").unwrap();
        let exited: String = table.get("exited_state").unwrap();
        assert_eq!(entered, "active");
        assert_eq!(exited, "idle");
    }

    #[test]
    fn dispatch_state_changed_handler_error() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");

        loader
            .load_string(
                &runtime,
                "broken_handler",
                r#"
                local m = {}

                function m.on_state_enter(event)
                    error("intentional error")
                end

                return m
            "#,
            )
            .unwrap();

        let event = StateChanged {
            entity: Entity::PLACEHOLDER,
            transition: StateTransition {
                from: StateId::new("idle"),
                to: StateId::new("active"),
                time_in_previous: 1.0,
            },
        };

        let (_exit_result, enter_result) =
            HookDispatcher::dispatch_state_changed(&runtime, &loader, &event);

        assert_eq!(enter_result.handlers_called, 0);
        assert_eq!(enter_result.errors.len(), 1);
        assert_eq!(enter_result.errors[0].0, "broken_handler");
    }

    #[test]
    fn dispatch_state_changed_multiple_handlers() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");

        loader
            .load_string(
                &runtime,
                "handler_a",
                r#"
                local m = { enter_called = false, exit_called = false }
                function m.on_state_enter(event) m.enter_called = true end
                function m.on_state_exit(event) m.exit_called = true end
                return m
            "#,
            )
            .unwrap();

        loader
            .load_string(
                &runtime,
                "handler_b",
                r#"
                local m = { enter_called = false, exit_called = false }
                function m.on_state_enter(event) m.enter_called = true end
                function m.on_state_exit(event) m.exit_called = true end
                return m
            "#,
            )
            .unwrap();

        let event = StateChanged {
            entity: Entity::PLACEHOLDER,
            transition: StateTransition {
                from: StateId::new("idle"),
                to: StateId::new("active"),
                time_in_previous: 1.0,
            },
        };

        let (exit_result, enter_result) =
            HookDispatcher::dispatch_state_changed(&runtime, &loader, &event);

        assert_eq!(exit_result.handlers_called, 2);
        assert_eq!(enter_result.handlers_called, 2);
    }

    #[test]
    fn state_changed_event_contains_transition_data() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");

        loader
            .load_string(
                &runtime,
                "data_checker",
                r#"
                local m = {
                    from = nil,
                    to = nil,
                    time = nil,
                    transition_from = nil,
                    transition_to = nil,
                }

                function m.on_state_enter(event)
                    m.from = event.from
                    m.to = event.to
                    m.time = event.time_in_previous
                    m.transition_from = event.transition:from_id()
                    m.transition_to = event.transition:to_id()
                end

                return m
            "#,
            )
            .unwrap();

        let event = StateChanged {
            entity: Entity::PLACEHOLDER,
            transition: StateTransition {
                from: StateId::new("idle"),
                to: StateId::new("active"),
                time_in_previous: 2.5,
            },
        };

        HookDispatcher::dispatch_state_changed(&runtime, &loader, &event);

        let module = loader.get("data_checker").unwrap();
        let table = module.table(runtime.lua()).unwrap();

        let from: String = table.get("from").unwrap();
        let to: String = table.get("to").unwrap();
        let time: f32 = table.get("time").unwrap();
        let transition_from: String = table.get("transition_from").unwrap();
        let transition_to: String = table.get("transition_to").unwrap();

        assert_eq!(from, "idle");
        assert_eq!(to, "active");
        assert!((time - 2.5).abs() < 0.01);
        assert_eq!(transition_from, "idle");
        assert_eq!(transition_to, "active");
    }

    #[test]
    fn state_changed_only_enter_handler() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");

        // Only has enter handler, no exit
        loader
            .load_string(
                &runtime,
                "enter_only",
                r#"
                local m = { entered = false }
                function m.on_state_enter(event)
                    m.entered = true
                end
                return m
            "#,
            )
            .unwrap();

        let event = StateChanged {
            entity: Entity::PLACEHOLDER,
            transition: StateTransition {
                from: StateId::new("idle"),
                to: StateId::new("active"),
                time_in_previous: 1.0,
            },
        };

        let (exit_result, enter_result) =
            HookDispatcher::dispatch_state_changed(&runtime, &loader, &event);

        assert_eq!(exit_result.handlers_called, 0);
        assert_eq!(enter_result.handlers_called, 1);
    }
}
