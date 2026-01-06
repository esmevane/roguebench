//! Events emitted when commands are executed.

use bevy::prelude::*;
use roguebench_core::commands::{Command, CommandId, CommandMeta, CommandResult};
use std::time::{Duration, Instant};

/// Event emitted when a command has been executed.
///
/// This event is fired after a command processor handles a command,
/// allowing other systems to react to state changes.
///
/// # Type Parameters
///
/// - `C`: The command type that was executed
///
/// # Example
///
/// ```ignore
/// fn react_to_damage(mut events: EventReader<CommandExecuted<DealDamage>>) {
///     for event in events.read() {
///         if event.result.is_success() {
///             // Play damage sound, show effects, etc.
///         }
///     }
/// }
/// ```
#[derive(Event)]
pub struct CommandExecuted<C: Command> {
    /// The command that was executed.
    pub command: C,
    /// The result of execution.
    pub result: CommandResult<C>,
    /// Metadata from when the command was sent.
    pub meta: CommandMeta,
    /// How long execution took.
    pub execution_time: Duration,
}

impl<C: Command> CommandExecuted<C> {
    /// Create a new CommandExecuted event.
    pub fn new(
        command: C,
        result: CommandResult<C>,
        meta: CommandMeta,
        execution_time: Duration,
    ) -> Self {
        Self {
            command,
            result,
            meta,
            execution_time,
        }
    }

    /// Create a success event with the given output.
    pub fn success(command: C, output: C::Output, meta: CommandMeta) -> Self {
        Self {
            command,
            result: CommandResult::Success(output),
            meta,
            execution_time: Duration::ZERO,
        }
    }

    /// Create a failure event with the given error.
    pub fn failed(command: C, error: C::Error, meta: CommandMeta) -> Self {
        Self {
            command,
            result: CommandResult::Failed(error),
            meta,
            execution_time: Duration::ZERO,
        }
    }

    /// Set the execution time.
    pub fn with_execution_time(mut self, time: Duration) -> Self {
        self.execution_time = time;
        self
    }

    /// Returns true if the command succeeded.
    pub fn is_success(&self) -> bool {
        self.result.is_success()
    }

    /// Returns true if the command failed.
    pub fn is_failed(&self) -> bool {
        self.result.is_failed()
    }

    /// Get the command ID.
    pub fn id(&self) -> CommandId {
        self.meta.id
    }
}

/// Helper for timing command execution.
///
/// Use this in processor systems to measure how long commands take.
///
/// # Example
///
/// ```ignore
/// fn process_command(mut bus: ResMut<CommandBus<MyCommand>>) {
///     for envelope in bus.drain() {
///         let timer = ExecutionTimer::start();
///         // ... execute command ...
///         let duration = timer.elapsed();
///     }
/// }
/// ```
pub struct ExecutionTimer {
    start: Instant,
}

impl ExecutionTimer {
    /// Start timing.
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Get elapsed time since start.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

/// Extension trait for apps to register command events.
pub trait CommandEventAppExt {
    /// Register the CommandExecuted event for a command type.
    fn register_command_event<C: Command>(&mut self) -> &mut Self;

    /// Register both the bus and event for a command type.
    ///
    /// This is a convenience method that calls both
    /// `register_command` and `register_command_event`.
    fn register_command_full<C: Command>(&mut self) -> &mut Self;
}

impl CommandEventAppExt for App {
    fn register_command_event<C: Command>(&mut self) -> &mut Self {
        self.add_event::<CommandExecuted<C>>();
        self
    }

    fn register_command_full<C: Command>(&mut self) -> &mut Self {
        use super::CommandBusAppExt;
        self.register_command::<C>().register_command_event::<C>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roguebench_core::commands::CommandMeta;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct TestCommand {
        value: i32,
    }

    impl Command for TestCommand {
        type Output = i32;
        type Error = String;

        fn name() -> &'static str {
            "test"
        }
    }

    #[test]
    fn create_success_event() {
        let cmd = TestCommand { value: 42 };
        let meta = CommandMeta::new(CommandId::new(1), 1000);

        let event = CommandExecuted::success(cmd, 100, meta);

        assert!(event.is_success());
        assert!(!event.is_failed());
        assert_eq!(event.command.value, 42);
        assert_eq!(event.id(), CommandId::new(1));
    }

    #[test]
    fn create_failed_event() {
        let cmd = TestCommand { value: 42 };
        let meta = CommandMeta::new(CommandId::new(2), 2000);

        let event = CommandExecuted::failed(cmd, "oops".to_string(), meta);

        assert!(!event.is_success());
        assert!(event.is_failed());
    }

    #[test]
    fn execution_timer() {
        let timer = ExecutionTimer::start();
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = timer.elapsed();

        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn register_command_events() {
        use bevy::app::App;

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .register_command_event::<TestCommand>();

        // Verify event is registered by sending one
        app.world_mut()
            .send_event(CommandExecuted::success(
                TestCommand { value: 1 },
                42,
                CommandMeta::new(CommandId::new(1), 0),
            ));

        app.update();

        // Event should have been processed
    }

    #[test]
    fn register_command_full() {
        use super::super::CommandBus;
        use bevy::app::App;

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(super::super::CommandBusPlugin)
            .register_command_full::<TestCommand>();

        app.update();

        // Both bus and event should exist
        assert!(app.world().get_resource::<CommandBus<TestCommand>>().is_some());
    }
}
