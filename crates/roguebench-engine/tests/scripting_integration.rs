//! Integration tests for the scripting framework.
//!
//! Tests the full flow from script loading through event dispatching.

use bevy::prelude::*;
use roguebench_core::items::{Effect, ItemDefinition, ItemId, ItemType};
use roguebench_engine::items::{ItemRegistry, ItemUsed};
use roguebench_engine::scripting::{
    HookDispatcher, LuaItemDefinition, ScriptLoader, ScriptRuntime,
};
use std::io::Write;
use tempfile::TempDir;

fn setup_scripts_dir() -> (TempDir, std::path::PathBuf) {
    let temp = TempDir::new().unwrap();
    let scripts = temp.path().join("scripts");
    std::fs::create_dir_all(&scripts).unwrap();
    (temp, scripts)
}

fn write_script(dir: &std::path::Path, name: &str, content: &str) {
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

#[test]
fn full_scripting_pipeline() {
    // Setup
    let (_temp, scripts) = setup_scripts_dir();
    let runtime = ScriptRuntime::new().unwrap();
    let mut loader = ScriptLoader::new(&scripts);
    let mut registry = ItemRegistry::new();

    // Create an item
    let healing_potion =
        ItemDefinition::new("healing_potion", "Healing Potion", ItemType::Consumable)
            .with_description("Restores health")
            .with_effect(Effect::ModifyStat {
                stat: "health".into(),
                amount: 50,
            })
            .with_value(25);
    registry.insert(healing_potion);

    // Write a script that tracks item usage
    write_script(
        &scripts,
        "item_tracker.lua",
        r#"
        local m = {}
        m.usage_log = {}

        function m.on_item_used(event)
            table.insert(m.usage_log, {
                item_id = event.item_id,
                item_name = event.item:name(),
                item_value = event.item:value()
            })
        end

        function m.get_usage_count()
            return #m.usage_log
        end

        function m.get_last_used_item()
            return m.usage_log[#m.usage_log]
        end

        return m
    "#,
    );

    // Load the script
    loader.load_file(&runtime, "item_tracker.lua").unwrap();

    // Simulate item usage
    let event = ItemUsed {
        item_id: ItemId::new("healing_potion"),
        user_entity: Entity::PLACEHOLDER,
    };

    // Dispatch the event
    let result = HookDispatcher::dispatch_item_used(&runtime, &loader, &registry, &event);
    assert_eq!(result.handlers_called, 1);
    assert!(result.errors.is_empty());

    // Verify the script tracked the usage
    let module = loader.get("item_tracker").unwrap();
    let table = module.table(runtime.lua()).unwrap();

    let count: i32 = runtime
        .call_module_function(&table, "get_usage_count", ())
        .unwrap();
    assert_eq!(count, 1);

    let last: mlua::Table = runtime
        .call_module_function(&table, "get_last_used_item", ())
        .unwrap();
    assert_eq!(last.get::<String>("item_id").unwrap(), "healing_potion");
    assert_eq!(last.get::<String>("item_name").unwrap(), "Healing Potion");
    assert_eq!(last.get::<u32>("item_value").unwrap(), 25);
}

#[test]
fn multiple_scripts_handle_same_event() {
    let runtime = ScriptRuntime::new().unwrap();
    let mut loader = ScriptLoader::new("scripts");
    let mut registry = ItemRegistry::new();

    let item = ItemDefinition::new("test_item", "Test Item", ItemType::Misc);
    registry.insert(item);

    // Load two scripts that both handle item usage
    loader
        .load_string(
            &runtime,
            "counter_a",
            r#"
            local m = { count = 0 }
            function m.on_item_used(event)
                m.count = m.count + 1
            end
            return m
        "#,
        )
        .unwrap();

    loader
        .load_string(
            &runtime,
            "counter_b",
            r#"
            local m = { count = 0 }
            function m.on_item_used(event)
                m.count = m.count + 10
            end
            return m
        "#,
        )
        .unwrap();

    let event = ItemUsed {
        item_id: ItemId::new("test_item"),
        user_entity: Entity::PLACEHOLDER,
    };

    // Dispatch should call both handlers
    let result = HookDispatcher::dispatch_item_used(&runtime, &loader, &registry, &event);
    assert_eq!(result.handlers_called, 2);

    // Verify both scripts were updated
    let module_a = loader.get("counter_a").unwrap();
    let table_a = module_a.table(runtime.lua()).unwrap();
    let count_a: i32 = table_a.get("count").unwrap();
    assert_eq!(count_a, 1);

    let module_b = loader.get("counter_b").unwrap();
    let table_b = module_b.table(runtime.lua()).unwrap();
    let count_b: i32 = table_b.get("count").unwrap();
    assert_eq!(count_b, 10);
}

#[test]
fn script_can_access_item_effects() {
    let runtime = ScriptRuntime::new().unwrap();
    let mut loader = ScriptLoader::new("scripts");
    let mut registry = ItemRegistry::new();

    let potion = ItemDefinition::new("super_potion", "Super Potion", ItemType::Consumable)
        .with_effect(Effect::ModifyStat {
            stat: "health".into(),
            amount: 100,
        })
        .with_effect(Effect::ApplyStatus {
            status: "regeneration".into(),
            duration_secs: 10.0,
        });
    registry.insert(potion);

    loader
        .load_string(
            &runtime,
            "effect_analyzer",
            r#"
            local m = { total_healing = 0, statuses = {} }

            function m.on_item_used(event)
                for _, effect in ipairs(event.item:effects()) do
                    if effect:kind() == "modify_stat" and effect:stat() == "health" then
                        m.total_healing = m.total_healing + effect:amount()
                    elseif effect:kind() == "apply_status" then
                        table.insert(m.statuses, {
                            name = effect:status(),
                            duration = effect:duration()
                        })
                    end
                end
            end

            return m
        "#,
        )
        .unwrap();

    let event = ItemUsed {
        item_id: ItemId::new("super_potion"),
        user_entity: Entity::PLACEHOLDER,
    };

    HookDispatcher::dispatch_item_used(&runtime, &loader, &registry, &event);

    let module = loader.get("effect_analyzer").unwrap();
    let table = module.table(runtime.lua()).unwrap();

    let total_healing: i32 = table.get("total_healing").unwrap();
    assert_eq!(total_healing, 100);

    let statuses: mlua::Table = table.get("statuses").unwrap();
    let first_status: mlua::Table = statuses.get(1).unwrap();
    assert_eq!(first_status.get::<String>("name").unwrap(), "regeneration");
    assert_eq!(first_status.get::<f32>("duration").unwrap(), 10.0);
}

#[test]
fn hot_reload_script() {
    let (_temp, scripts) = setup_scripts_dir();
    let runtime = ScriptRuntime::new().unwrap();
    let mut loader = ScriptLoader::new(&scripts);

    // Initial version
    write_script(&scripts, "version_test.lua", "return { version = 1 }");

    loader.load_file(&runtime, "version_test.lua").unwrap();
    let module = loader.get("version_test").unwrap();
    let table = module.table(runtime.lua()).unwrap();
    let version: i32 = table.get("version").unwrap();
    assert_eq!(version, 1);

    // Update the script
    write_script(&scripts, "version_test.lua", "return { version = 2 }");

    // Reload it
    loader.reload(&runtime, "version_test").unwrap();
    let module = loader.get("version_test").unwrap();
    let table = module.table(runtime.lua()).unwrap();
    let version: i32 = table.get("version").unwrap();
    assert_eq!(version, 2);
}

#[test]
fn script_error_handling() {
    let runtime = ScriptRuntime::new().unwrap();
    let mut loader = ScriptLoader::new("scripts");
    let mut registry = ItemRegistry::new();

    let item = ItemDefinition::new("error_item", "Error Item", ItemType::Misc);
    registry.insert(item);

    // Script that throws an error
    loader
        .load_string(
            &runtime,
            "error_script",
            r#"
            local m = {}
            function m.on_item_used(event)
                error("Script error!")
            end
            return m
        "#,
        )
        .unwrap();

    // Script that works correctly
    loader
        .load_string(
            &runtime,
            "good_script",
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
        item_id: ItemId::new("error_item"),
        user_entity: Entity::PLACEHOLDER,
    };

    let result = HookDispatcher::dispatch_item_used(&runtime, &loader, &registry, &event);

    // One handler succeeded, one failed
    assert_eq!(result.handlers_called, 1);
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].0, "error_script");

    // Good script should still have been called
    let module = loader.get("good_script").unwrap();
    let table = module.table(runtime.lua()).unwrap();
    let called: bool = table.get("called").unwrap();
    assert!(called);
}

#[test]
fn rust_function_exposed_to_lua() {
    let runtime = ScriptRuntime::new().unwrap();

    // Expose a Rust function to Lua
    let double_fn = runtime
        .create_function(|_, n: i32| Ok(n * 2))
        .unwrap();
    runtime.set_global("double", double_fn).unwrap();

    // Use it in a script
    let result: i32 = runtime.eval("return double(21)").unwrap();
    assert_eq!(result, 42);
}

#[test]
fn userdata_roundtrip() {
    let runtime = ScriptRuntime::new().unwrap();

    let item = ItemDefinition::new("test_item", "Test Item", ItemType::Equipment)
        .with_description("A test item")
        .with_value(100);

    runtime
        .set_global("item", LuaItemDefinition(item.clone()))
        .unwrap();

    // Access the item data from Lua
    let name: String = runtime.eval("return item:name()").unwrap();
    assert_eq!(name, "Test Item");

    let value: u32 = runtime.eval("return item:value()").unwrap();
    assert_eq!(value, 100);

    let desc: String = runtime.eval("return item:description()").unwrap();
    assert_eq!(desc, "A test item");

    let type_name: String = runtime.eval("return item:item_type():name()").unwrap();
    assert_eq!(type_name, "equipment");
}

#[test]
fn sandbox_prevents_dangerous_operations() {
    let runtime = ScriptRuntime::new().unwrap();

    // These should fail in sandbox mode
    let io_result = runtime.eval::<mlua::Value>("return io");
    assert!(io_result.is_err() || matches!(io_result, Ok(mlua::Value::Nil)));

    let os_result = runtime.eval::<mlua::Value>("return os.execute");
    assert!(os_result.is_err() || matches!(os_result, Ok(mlua::Value::Nil)));

    let loadfile_result = runtime.eval::<mlua::Value>("return loadfile");
    assert!(loadfile_result.is_err() || matches!(loadfile_result, Ok(mlua::Value::Nil)));
}

#[test]
fn load_all_scripts_from_directory() {
    let (_temp, scripts) = setup_scripts_dir();
    let runtime = ScriptRuntime::new().unwrap();
    let mut loader = ScriptLoader::new(&scripts);

    write_script(&scripts, "mod_a.lua", "return { name = 'a' }");
    write_script(&scripts, "mod_b.lua", "return { name = 'b' }");
    write_script(&scripts, "mod_c.luau", "return { name = 'c' }");

    let results = loader.load_all(&runtime);

    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|r| r.is_ok()));

    let names: Vec<_> = results.iter().filter_map(|r| r.as_ref().ok()).collect();
    assert!(names.contains(&&"mod_a".to_string()));
    assert!(names.contains(&&"mod_b".to_string()));
    assert!(names.contains(&&"mod_c".to_string()));
}
