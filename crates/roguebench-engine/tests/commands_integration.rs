//! Integration tests for the Command Bus framework.
//!
//! These tests verify the complete command flow:
//! Send → Process → Log → Event

use bevy::prelude::*;
use roguebench_core::commands::{Command, CommandId, CommandMeta};
use roguebench_engine::commands::{
    CommandBus, CommandBusAppExt, CommandBusPlugin, CommandEventAppExt, CommandExecuted,
    CommandLog, CommandLogAppExt, ExecutionTimer, FrameCount,
};
use serde::{Deserialize, Serialize};

// Test command types

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DealDamage {
    target_id: u32,
    amount: i32,
}

impl Command for DealDamage {
    type Output = DamageResult;
    type Error = DamageError;

    fn name() -> &'static str {
        "deal_damage"
    }
}

#[derive(Clone, Debug)]
struct DamageResult {
    final_health: i32,
    overkill: bool,
}

#[derive(Clone, Debug)]
struct DamageError {
    reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HealTarget {
    target_id: u32,
    amount: i32,
}

impl Command for HealTarget {
    type Output = i32; // New health
    type Error = String;

    fn name() -> &'static str {
        "heal_target"
    }
}

// Mock health component for testing
#[derive(Component)]
struct Health {
    current: i32,
    max: i32,
}

/// System that processes DealDamage commands.
fn process_damage_commands(
    mut bus: ResMut<CommandBus<DealDamage>>,
    mut events: EventWriter<CommandExecuted<DealDamage>>,
    mut log: ResMut<CommandLog<DealDamage>>,
    mut query: Query<&mut Health>,
) {
    for envelope in bus.drain() {
        let timer = ExecutionTimer::start();
        let cmd = &envelope.command;

        // Find entity with matching ID (simplified - in real code use a lookup)
        let result = if let Some(mut health) = query.iter_mut().next() {
            health.current -= cmd.amount;
            let overkill = health.current < 0;
            if overkill {
                health.current = 0;
            }

            Ok(DamageResult {
                final_health: health.current,
                overkill,
            })
        } else {
            Err(DamageError {
                reason: "Target not found".to_string(),
            })
        };

        // Log the command
        match &result {
            Ok(_) => log.log_success(envelope.command.clone(), envelope.meta.clone()),
            Err(_) => log.log_failure(envelope.command.clone(), envelope.meta.clone()),
        }

        // Emit event
        let event = match result {
            Ok(output) => {
                CommandExecuted::success(envelope.command, output, envelope.meta)
                    .with_execution_time(timer.elapsed())
            }
            Err(error) => {
                CommandExecuted::failed(envelope.command, error, envelope.meta)
                    .with_execution_time(timer.elapsed())
            }
        };
        events.send(event);
    }
}

/// Test: Full command flow - send, process, event.
#[test]
fn full_command_flow() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(CommandBusPlugin)
        .register_command::<DealDamage>()
        .register_command_event::<DealDamage>()
        .register_command_log::<DealDamage>()
        .add_systems(Update, process_damage_commands);

    // Spawn entity with health
    app.world_mut().spawn(Health { current: 100, max: 100 });

    // Send command
    {
        let mut bus = app.world_mut().resource_mut::<CommandBus<DealDamage>>();
        bus.send(DealDamage {
            target_id: 1,
            amount: 30,
        });
    }

    // Process
    app.update();

    // Verify log
    {
        let log = app.world().resource::<CommandLog<DealDamage>>();
        assert_eq!(log.len(), 1);
        assert!(log.entries()[0].succeeded);
        assert_eq!(log.entries()[0].command.amount, 30);
    }

    // Verify health was reduced
    {
        let mut query = app.world_mut().query::<&Health>();
        let health = query.iter(app.world()).next().unwrap();
        assert_eq!(health.current, 70);
    }
}

/// Test: Multiple command types work independently.
#[test]
fn multiple_command_types() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(CommandBusPlugin)
        .register_command::<DealDamage>()
        .register_command::<HealTarget>();

    // Send commands of different types
    {
        let mut damage_bus = app.world_mut().resource_mut::<CommandBus<DealDamage>>();
        damage_bus.send(DealDamage {
            target_id: 1,
            amount: 10,
        });

        let mut heal_bus = app.world_mut().resource_mut::<CommandBus<HealTarget>>();
        heal_bus.send(HealTarget {
            target_id: 1,
            amount: 20,
        });
    }

    // Verify they're in separate queues
    {
        let damage_bus = app.world().resource::<CommandBus<DealDamage>>();
        let heal_bus = app.world().resource::<CommandBus<HealTarget>>();

        assert_eq!(damage_bus.len(), 1);
        assert_eq!(heal_bus.len(), 1);
    }
}

/// Test: Frame tracking works correctly.
#[test]
fn frame_tracking() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_plugins(CommandBusPlugin);

    // First update
    app.update();
    assert_eq!(app.world().resource::<FrameCount>().0, 1);

    // Second update
    app.update();
    assert_eq!(app.world().resource::<FrameCount>().0, 2);
}

/// Test: Command IDs are unique and sequential.
#[test]
fn command_ids_unique() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(CommandBusPlugin)
        .register_command::<DealDamage>();

    let mut ids = Vec::new();
    {
        let mut bus = app.world_mut().resource_mut::<CommandBus<DealDamage>>();
        for i in 0..5 {
            ids.push(bus.send(DealDamage {
                target_id: 1,
                amount: i,
            }));
        }
    }

    // All IDs should be unique
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            assert_ne!(ids[i], ids[j]);
        }
    }

    // IDs should be sequential
    for i in 1..ids.len() {
        assert_eq!(ids[i].0, ids[i - 1].0 + 1);
    }
}

/// Test: Log persistence and reload.
#[test]
fn log_persistence() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir()
        .join(format!(
            "roguebench_cmd_test_{}_{}.jsonl",
            std::process::id(),
            timestamp
        ))
        .to_string_lossy()
        .to_string();

    // Create and populate log
    {
        let mut log = CommandLog::<DealDamage>::new();
        log.log_success(
            DealDamage {
                target_id: 1,
                amount: 10,
            },
            CommandMeta::new(CommandId::new(1), 1000).with_frame(1),
        );
        log.log_success(
            DealDamage {
                target_id: 2,
                amount: 20,
            },
            CommandMeta::new(CommandId::new(2), 2000).with_frame(2),
        );
        log.save_to_file(&path).unwrap();
    }

    // Load and verify
    {
        let log = CommandLog::<DealDamage>::load_from_file(&path).unwrap();
        assert_eq!(log.len(), 2);

        let entries: Vec<_> = log.iter().collect();
        assert_eq!(entries[0].command.amount, 10);
        assert_eq!(entries[1].command.amount, 20);
    }

    // Cleanup
    std::fs::remove_file(path).ok();
}

/// Test: Replay from log.
#[test]
fn replay_from_log() {
    let mut log = CommandLog::<DealDamage>::new();

    // Populate with mixed success/failure
    log.log_success(
        DealDamage {
            target_id: 1,
            amount: 10,
        },
        CommandMeta::new(CommandId::new(1), 1000),
    );
    log.log_failure(
        DealDamage {
            target_id: 2,
            amount: 99,
        },
        CommandMeta::new(CommandId::new(2), 2000),
    );
    log.log_success(
        DealDamage {
            target_id: 3,
            amount: 30,
        },
        CommandMeta::new(CommandId::new(3), 3000),
    );

    // Replay all
    let all: Vec<_> = log.replay().collect();
    assert_eq!(all.len(), 3);

    // Replay only successes
    let successes: Vec<_> = log.replay().successes_only().collect();
    assert_eq!(successes.len(), 2);
    assert_eq!(successes[0].0.amount, 10);
    assert_eq!(successes[1].0.amount, 30);
}

/// Test: Command events are received by systems.
#[test]
fn command_events_received() {
    #[derive(Resource, Default)]
    struct EventTracker {
        received: Vec<i32>,
    }

    fn track_events(
        mut tracker: ResMut<EventTracker>,
        mut events: EventReader<CommandExecuted<DealDamage>>,
    ) {
        for event in events.read() {
            tracker.received.push(event.command.amount);
        }
    }

    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(CommandBusPlugin)
        .register_command::<DealDamage>()
        .register_command_event::<DealDamage>()
        .register_command_log::<DealDamage>()
        .init_resource::<EventTracker>()
        .add_systems(Update, (process_damage_commands, track_events).chain());

    // Spawn entity
    app.world_mut().spawn(Health { current: 100, max: 100 });

    // Send commands
    {
        let mut bus = app.world_mut().resource_mut::<CommandBus<DealDamage>>();
        bus.send(DealDamage {
            target_id: 1,
            amount: 10,
        });
        bus.send(DealDamage {
            target_id: 1,
            amount: 20,
        });
    }

    // Process
    app.update();

    // Verify events were received
    {
        let tracker = app.world().resource::<EventTracker>();
        assert_eq!(tracker.received, vec![10, 20]);
    }
}
