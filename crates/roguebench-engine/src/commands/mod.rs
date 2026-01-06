//! Command bus infrastructure for Roguebench.
//!
//! All game mutations flow through the command bus. This provides:
//! - Centralized mutation tracking
//! - Command logging for replay and debugging
//! - Hook points for scripting integration
//! - Validation before execution
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────┐    ┌──────────────┐    ┌─────────────────┐
//! │ Game System │───▶│ CommandBus<C>│───▶│ Processor System│
//! │  (sender)   │    │   (queue)    │    │   (executor)    │
//! └─────────────┘    └──────────────┘    └────────┬────────┘
//!                                                 │
//!                                                 ▼
//!                                        ┌───────────────────┐
//!                                        │CommandExecuted<C> │
//!                                        │    (event)        │
//!                                        └───────────────────┘
//! ```
//!
//! # Usage
//!
//! 1. Define a command type implementing `Command`
//! 2. Register the bus with `app.init_resource::<CommandBus<YourCommand>>()`
//! 3. Send commands: `bus.send(YourCommand { ... })`
//! 4. Process in a system: `for envelope in bus.drain() { ... }`
//! 5. Emit completion event: `events.send(CommandExecuted::success(...))`

mod bus;
mod events;
mod log;

pub use bus::CommandBus;
pub use events::{CommandEventAppExt, CommandExecuted, ExecutionTimer};
pub use log::{CommandLog, CommandLogAppExt, LogEntry, ReplayIterator};

// Re-export core types for convenience
pub use roguebench_core::commands::{
    Command, CommandId, CommandMeta, CommandResult, Envelope, ValidationError,
};

use bevy::prelude::*;

/// Plugin that sets up the command bus infrastructure.
///
/// This adds the frame counter system but does NOT register any specific
/// command buses. Each command type should be registered by the system
/// that uses it.
pub struct CommandBusPlugin;

impl Plugin for CommandBusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameCount>()
            .add_systems(First, increment_frame_count);
    }
}

/// Resource tracking the current frame number.
#[derive(Resource, Default)]
pub struct FrameCount(pub u64);

/// System to increment frame count each tick.
fn increment_frame_count(mut frame_count: ResMut<FrameCount>) {
    frame_count.0 += 1;
}

/// Extension trait for registering command buses.
pub trait CommandBusAppExt {
    /// Register a command bus for a specific command type.
    ///
    /// This initializes the `CommandBus<C>` resource.
    fn register_command<C: Command>(&mut self) -> &mut Self;
}

impl CommandBusAppExt for App {
    fn register_command<C: Command>(&mut self) -> &mut Self {
        self.init_resource::<CommandBus<C>>();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct MockCommand {
        data: String,
    }

    impl Command for MockCommand {
        type Output = ();
        type Error = ();

        fn name() -> &'static str {
            "mock"
        }
    }

    #[test]
    fn frame_count_increments() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CommandBusPlugin);

        // Initial frame
        app.update();
        assert_eq!(app.world().resource::<FrameCount>().0, 1);

        // Second frame
        app.update();
        assert_eq!(app.world().resource::<FrameCount>().0, 2);
    }

    #[test]
    fn register_command_extension() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(CommandBusPlugin)
            .register_command::<MockCommand>();

        app.update();

        // Verify the bus exists
        let bus = app.world().resource::<CommandBus<MockCommand>>();
        assert!(bus.is_empty());
    }
}
