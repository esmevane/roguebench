//! Integration tests for the state machine framework.
//!
//! Tests the full flow from definition loading through state transitions
//! and scripting hooks.

use bevy::prelude::*;
use roguebench_core::state_machine::{
    CompareOp, StateDefinition, StateId, StateMachineDefinition, StateTransition,
    TransitionCondition, TransitionDefinition,
};
use roguebench_engine::data::{ContentRegistry, Database};
use roguebench_engine::scripting::{HookDispatcher, ScriptLoader, ScriptRuntime};
use roguebench_engine::state_machine::{StateChanged, StateMachine, StateMachineBundle};

/// Create a simple two-state machine for testing.
fn simple_machine() -> StateMachineDefinition {
    StateMachineDefinition::new("simple_ai", "idle")
        .with_name("Simple AI")
        .with_state(StateDefinition::new("idle").with_name("Idle"))
        .with_state(StateDefinition::new("active").with_name("Active"))
        .with_transition(
            TransitionDefinition::new("idle", "active")
                .when(TransitionCondition::flag("activate", true))
                .with_name("activate"),
        )
}

/// Create an enemy AI state machine with complex transitions.
fn enemy_ai_machine() -> StateMachineDefinition {
    StateMachineDefinition::new("enemy_ai", "idle")
        .with_name("Enemy AI")
        .with_state(
            StateDefinition::new("idle")
                .with_name("Idle")
                .with_metadata("animation", serde_json::json!("idle_anim")),
        )
        .with_state(
            StateDefinition::new("patrol")
                .with_name("Patrolling")
                .with_metadata("speed", serde_json::json!(1.0)),
        )
        .with_state(
            StateDefinition::new("chase")
                .with_name("Chasing")
                .with_metadata("speed", serde_json::json!(2.0)),
        )
        .with_state(
            StateDefinition::new("attack")
                .with_name("Attacking")
                .with_metadata("damage", serde_json::json!(10)),
        )
        .with_transition(
            TransitionDefinition::new("idle", "patrol")
                .when(TransitionCondition::after(2.0))
                .with_name("start_patrol"),
        )
        .with_transition(
            TransitionDefinition::new("patrol", "chase")
                .when(TransitionCondition::flag("player_spotted", true))
                .with_priority(10)
                .with_name("spot_player"),
        )
        .with_transition(
            TransitionDefinition::new("chase", "attack")
                .when(TransitionCondition::threshold(
                    "distance",
                    CompareOp::Le,
                    1.0,
                ))
                .with_name("in_range"),
        )
        .with_transition(
            TransitionDefinition::new("chase", "patrol")
                .when(TransitionCondition::flag("player_spotted", false))
                .with_name("lost_player"),
        )
        .with_transition(
            TransitionDefinition::new("attack", "chase")
                .when(TransitionCondition::threshold(
                    "distance",
                    CompareOp::Gt,
                    1.0,
                ))
                .with_name("out_of_range"),
        )
}

#[test]
fn load_state_machine_from_database() {
    let db = Database::in_memory().unwrap();

    // Store a state machine definition
    let machine = simple_machine();
    db.upsert_state_machine(&machine).unwrap();

    // Load it back
    let loaded = db.get_state_machine("simple_ai").unwrap();
    assert_eq!(loaded.id, "simple_ai");
    assert_eq!(loaded.name, "Simple AI");
    assert_eq!(loaded.initial_state.0, "idle");
    assert_eq!(loaded.states.len(), 2);
    assert_eq!(loaded.transitions.len(), 1);
}

#[test]
fn content_registry_with_state_machines() {
    let db = Database::in_memory().unwrap();

    // Store multiple state machines
    db.upsert_state_machine(&simple_machine()).unwrap();
    db.upsert_state_machine(&enemy_ai_machine()).unwrap();

    // Load via ContentRegistry
    let registry = ContentRegistry::<StateMachineDefinition>::load_from_db(&db).unwrap();
    assert_eq!(registry.len(), 2);
    assert!(registry.contains(&"simple_ai".to_string()));
    assert!(registry.contains(&"enemy_ai".to_string()));

    // Access individual machines
    let enemy = registry.get(&"enemy_ai".to_string()).unwrap();
    assert_eq!(enemy.states.len(), 4);
}

#[test]
fn state_machine_entity_lifecycle() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_event::<StateChanged>();

    // Spawn entity with state machine
    let machine = simple_machine();
    let entity = app
        .world_mut()
        .spawn(StateMachineBundle::new(machine))
        .id();

    // Initially in idle state
    {
        let sm = app.world().get::<StateMachine>(entity).unwrap();
        assert!(sm.is_in_state("idle"));
    }

    // Trigger transition
    {
        let mut sm = app.world_mut().get_mut::<StateMachine>(entity).unwrap();
        sm.set_flag("activate", true);
    }

    // Manually update (simulating system run)
    {
        let mut sm = app.world_mut().get_mut::<StateMachine>(entity).unwrap();
        let transition = sm.update(0.016);
        assert!(transition.is_some());
    }

    // Now in active state
    {
        let sm = app.world().get::<StateMachine>(entity).unwrap();
        assert!(sm.is_in_state("active"));
    }
}

#[test]
fn complex_ai_state_transitions() {
    let machine = enemy_ai_machine();
    let mut sm = StateMachine::new(machine);

    // Start idle
    assert!(sm.is_in_state("idle"));

    // Wait for patrol transition (2 seconds)
    sm.update(1.0);
    assert!(sm.is_in_state("idle")); // Still idle

    sm.force_update();
    sm.update(1.5);
    assert!(sm.is_in_state("patrol")); // Now patrolling

    // Spot player
    sm.set_flag("player_spotted", true);
    sm.update(0.016);
    assert!(sm.is_in_state("chase")); // Now chasing

    // Get in range
    sm.set_value("distance", 0.5);
    sm.update(0.016);
    assert!(sm.is_in_state("attack")); // Now attacking

    // Back out of range
    sm.set_value("distance", 2.0);
    sm.update(0.016);
    assert!(sm.is_in_state("chase")); // Back to chasing

    // Lose sight of player
    sm.set_flag("player_spotted", false);
    sm.update(0.016);
    assert!(sm.is_in_state("patrol")); // Back to patrolling
}

#[test]
fn state_changed_hooks_integration() {
    let runtime = ScriptRuntime::new().unwrap();
    let mut loader = ScriptLoader::new("scripts");

    // Load a script that tracks state changes
    loader
        .load_string(
            &runtime,
            "state_tracker",
            r#"
            local m = {}
            m.transitions = {}
            m.enter_count = 0
            m.exit_count = 0

            function m.on_state_enter(event)
                m.enter_count = m.enter_count + 1
                table.insert(m.transitions, {
                    type = "enter",
                    from = event.from,
                    to = event.to,
                    time = event.time_in_previous
                })
            end

            function m.on_state_exit(event)
                m.exit_count = m.exit_count + 1
            end

            function m.get_transition_count()
                return #m.transitions
            end

            function m.get_last_transition()
                return m.transitions[#m.transitions]
            end

            return m
        "#,
        )
        .unwrap();

    // Create a state change event
    let event = StateChanged {
        entity: Entity::PLACEHOLDER,
        transition: StateTransition {
            from: StateId::new("idle"),
            to: StateId::new("active"),
            time_in_previous: 2.5,
        },
    };

    // Dispatch the event
    let (exit_result, enter_result) =
        HookDispatcher::dispatch_state_changed(&runtime, &loader, &event);

    assert_eq!(exit_result.handlers_called, 1);
    assert_eq!(enter_result.handlers_called, 1);

    // Verify the script tracked the transition
    let module = loader.get("state_tracker").unwrap();
    let table = module.table(runtime.lua()).unwrap();

    let enter_count: i32 = table.get("enter_count").unwrap();
    let exit_count: i32 = table.get("exit_count").unwrap();
    assert_eq!(enter_count, 1);
    assert_eq!(exit_count, 1);

    let count: i32 = runtime
        .call_module_function(&table, "get_transition_count", ())
        .unwrap();
    assert_eq!(count, 1);

    let last: mlua::Table = runtime
        .call_module_function(&table, "get_last_transition", ())
        .unwrap();
    assert_eq!(last.get::<String>("from").unwrap(), "idle");
    assert_eq!(last.get::<String>("to").unwrap(), "active");
    assert!((last.get::<f32>("time").unwrap() - 2.5).abs() < 0.01);
}

#[test]
fn state_metadata_accessible_from_lua() {
    let runtime = ScriptRuntime::new().unwrap();

    // Create a state definition with metadata
    let state = StateDefinition::new("attack")
        .with_name("Attack State")
        .with_metadata("damage", serde_json::json!(25))
        .with_metadata("cooldown", serde_json::json!(1.5))
        .with_metadata("effects", serde_json::json!(["fire", "stun"]));

    // Expose it to Lua
    use roguebench_engine::scripting::LuaStateDefinition;
    runtime
        .set_global("attack_state", LuaStateDefinition(state))
        .unwrap();

    // Access from Lua
    let name: String = runtime.eval("return attack_state:name()").unwrap();
    assert_eq!(name, "Attack State");

    let damage: i32 = runtime.eval("return attack_state:metadata('damage')").unwrap();
    assert_eq!(damage, 25);

    let cooldown: f64 = runtime
        .eval("return attack_state:metadata('cooldown')")
        .unwrap();
    assert!((cooldown - 1.5).abs() < 0.01);

    let first_effect: String = runtime
        .eval("return attack_state:metadata('effects')[1]")
        .unwrap();
    assert_eq!(first_effect, "fire");
}

#[test]
fn full_state_machine_pipeline() {
    // This test exercises the complete flow:
    // 1. Define state machine
    // 2. Store in database
    // 3. Load into registry
    // 4. Create entity with state machine
    // 5. Run transitions
    // 6. Dispatch hooks to scripts

    // Setup database
    let db = Database::in_memory().unwrap();
    let machine = enemy_ai_machine();
    db.upsert_state_machine(&machine).unwrap();

    // Load registry
    let registry = ContentRegistry::<StateMachineDefinition>::load_from_db(&db).unwrap();

    // Setup scripting
    let runtime = ScriptRuntime::new().unwrap();
    let mut loader = ScriptLoader::new("scripts");

    loader
        .load_string(
            &runtime,
            "ai_monitor",
            r#"
            local m = {}
            m.state_log = {}

            function m.on_state_enter(event)
                table.insert(m.state_log, event.to)
            end

            function m.get_states_visited()
                return m.state_log
            end

            return m
        "#,
        )
        .unwrap();

    // Get the definition from registry
    let def = registry.get(&"enemy_ai".to_string()).unwrap();
    let mut sm = StateMachine::new(def.clone());

    // Collect state changes
    let mut transitions = Vec::new();

    // Simulate AI behavior
    // 1. Wait for patrol
    sm.force_update();
    sm.update(2.5);
    if let Some(t) = sm.update(0.016) {
        transitions.push(t);
    }
    // Check if we transitioned - we might need to force update
    if sm.is_in_state("idle") {
        sm.force_update();
        if let Some(t) = sm.update(0.016) {
            transitions.push(t);
        }
    }

    // 2. Spot player -> chase
    sm.set_flag("player_spotted", true);
    if let Some(t) = sm.update(0.016) {
        transitions.push(t);
    }

    // 3. Get in range -> attack
    sm.set_value("distance", 0.5);
    if let Some(t) = sm.update(0.016) {
        transitions.push(t);
    }

    // Dispatch all transitions to scripts
    for transition in &transitions {
        let event = StateChanged {
            entity: Entity::PLACEHOLDER,
            transition: transition.clone(),
        };
        HookDispatcher::dispatch_state_changed(&runtime, &loader, &event);
    }

    // Verify script tracked the states
    let module = loader.get("ai_monitor").unwrap();
    let table = module.table(runtime.lua()).unwrap();

    let states: mlua::Table = runtime
        .call_module_function(&table, "get_states_visited", ())
        .unwrap();

    let state_count = states.len().unwrap();
    assert!(state_count >= 1, "Should have visited at least one state");

    // Verify final state
    assert!(
        sm.is_in_state("attack"),
        "Should end in attack state, was in {}",
        sm.current_state()
    );
}

#[test]
fn database_crud_with_complex_machine() {
    let db = Database::in_memory().unwrap();

    // Create a complex machine
    let machine = enemy_ai_machine();
    db.upsert_state_machine(&machine).unwrap();

    // Verify count
    assert_eq!(db.count_state_machines().unwrap(), 1);

    // Load and verify all details
    let loaded = db.get_state_machine("enemy_ai").unwrap();
    assert_eq!(loaded.states.len(), 4);
    assert_eq!(loaded.transitions.len(), 5);

    // Verify state metadata was preserved
    let patrol_state = loaded.get_state(&StateId::new("patrol")).unwrap();
    assert_eq!(
        patrol_state.metadata.get("speed"),
        Some(&serde_json::json!(1.0))
    );

    // Update the machine
    let updated = StateMachineDefinition::new("enemy_ai", "patrol")
        .with_name("Updated Enemy AI")
        .with_state(StateDefinition::new("patrol"))
        .with_state(StateDefinition::new("flee"));

    db.upsert_state_machine(&updated).unwrap();

    // Still one machine
    assert_eq!(db.count_state_machines().unwrap(), 1);

    // Verify update
    let reloaded = db.get_state_machine("enemy_ai").unwrap();
    assert_eq!(reloaded.name, "Updated Enemy AI");
    assert_eq!(reloaded.initial_state.0, "patrol");
    assert_eq!(reloaded.states.len(), 2);

    // Delete
    assert!(db.delete_state_machine("enemy_ai").unwrap());
    assert_eq!(db.count_state_machines().unwrap(), 0);
}

#[test]
fn multiple_entities_with_same_definition() {
    let machine = simple_machine();

    let mut sm1 = StateMachine::new(machine.clone());
    let mut sm2 = StateMachine::new(machine);

    // Both start in idle
    assert!(sm1.is_in_state("idle"));
    assert!(sm2.is_in_state("idle"));

    // Activate only sm1
    sm1.set_flag("activate", true);
    sm1.update(0.016);
    sm2.update(0.016);

    // sm1 transitioned, sm2 didn't
    assert!(sm1.is_in_state("active"));
    assert!(sm2.is_in_state("idle"));

    // Now activate sm2
    sm2.set_flag("activate", true);
    sm2.update(0.016);

    // Both in active
    assert!(sm1.is_in_state("active"));
    assert!(sm2.is_in_state("active"));
}

#[test]
fn state_transition_timing() {
    let machine = StateMachineDefinition::new("timed", "waiting")
        .with_state(StateDefinition::new("waiting"))
        .with_state(StateDefinition::new("ready"))
        .with_transition(
            TransitionDefinition::new("waiting", "ready").when(TransitionCondition::after(5.0)),
        );

    let mut sm = StateMachine::new(machine);

    // Time accumulates
    sm.update(2.0);
    assert!(sm.is_in_state("waiting"));
    assert!((sm.context().time_in_state - 2.0).abs() < 0.01);

    sm.update(2.0);
    assert!(sm.is_in_state("waiting"));
    assert!((sm.context().time_in_state - 4.0).abs() < 0.01);

    sm.force_update();
    sm.update(1.5);
    assert!(sm.is_in_state("ready"));

    // Time reset after transition
    assert!(sm.context().time_in_state < 2.0);
}
