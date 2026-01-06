//! CommandBus resource for sending and processing commands.

use bevy::prelude::*;
use roguebench_core::commands::{Command, CommandId, CommandMeta, Envelope};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// A Bevy resource that queues commands for processing.
///
/// Each command type has its own bus instance. Systems send commands
/// through the bus, and other systems drain and process them.
///
/// # Example
///
/// ```ignore
/// fn send_damage(mut bus: ResMut<CommandBus<DealDamage>>) {
///     bus.send(DealDamage { target, amount: 10 });
/// }
///
/// fn process_damage(mut bus: ResMut<CommandBus<DealDamage>>) {
///     for envelope in bus.drain() {
///         // Process envelope.command
///     }
/// }
/// ```
#[derive(Resource)]
pub struct CommandBus<C: Command> {
    queue: VecDeque<Envelope<C>>,
    next_id: u64,
    current_frame: u64,
}

impl<C: Command> Default for CommandBus<C> {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
            next_id: 1,
            current_frame: 0,
        }
    }
}

impl<C: Command> CommandBus<C> {
    /// Create a new empty command bus.
    pub fn new() -> Self {
        Self::default()
    }

    /// Send a command to the bus.
    ///
    /// The command is wrapped in an envelope with auto-generated metadata
    /// including a unique ID and timestamp.
    pub fn send(&mut self, command: C) -> CommandId {
        let id = CommandId::new(self.next_id);
        self.next_id += 1;

        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let meta = CommandMeta::new(id, timestamp_ms).with_frame(self.current_frame);
        let envelope = Envelope::new(command, meta);

        self.queue.push_back(envelope);
        id
    }

    /// Send a command with custom metadata.
    ///
    /// Use this when replaying commands or when you need control over
    /// the command ID and timestamp.
    pub fn send_with_meta(&mut self, command: C, meta: CommandMeta) {
        let envelope = Envelope::new(command, meta);
        self.queue.push_back(envelope);
    }

    /// Drain all queued commands for processing.
    ///
    /// Returns an iterator that removes and yields each queued command.
    pub fn drain(&mut self) -> impl Iterator<Item = Envelope<C>> + '_ {
        self.queue.drain(..)
    }

    /// Check if there are any queued commands.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Get the number of queued commands.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Peek at the next command without removing it.
    pub fn peek(&self) -> Option<&Envelope<C>> {
        self.queue.front()
    }

    /// Set the current frame number for new commands.
    ///
    /// This should be called by a system that tracks frame count.
    pub fn set_frame(&mut self, frame: u64) {
        self.current_frame = frame;
    }

    /// Get the current frame number.
    pub fn frame(&self) -> u64 {
        self.current_frame
    }

    /// Clear all queued commands without processing them.
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct TestCommand {
        value: i32,
    }

    impl Command for TestCommand {
        type Output = ();
        type Error = String;

        fn name() -> &'static str {
            "test"
        }
    }

    #[test]
    fn send_and_drain() {
        let mut bus = CommandBus::<TestCommand>::new();

        assert!(bus.is_empty());
        assert_eq!(bus.len(), 0);

        let id1 = bus.send(TestCommand { value: 1 });
        let id2 = bus.send(TestCommand { value: 2 });

        assert!(!bus.is_empty());
        assert_eq!(bus.len(), 2);
        assert_ne!(id1, id2);

        let commands: Vec<_> = bus.drain().collect();
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].command.value, 1);
        assert_eq!(commands[1].command.value, 2);

        assert!(bus.is_empty());
    }

    #[test]
    fn command_ids_are_sequential() {
        let mut bus = CommandBus::<TestCommand>::new();

        let id1 = bus.send(TestCommand { value: 1 });
        let id2 = bus.send(TestCommand { value: 2 });
        let id3 = bus.send(TestCommand { value: 3 });

        assert_eq!(id1.0, 1);
        assert_eq!(id2.0, 2);
        assert_eq!(id3.0, 3);
    }

    #[test]
    fn frame_tracking() {
        let mut bus = CommandBus::<TestCommand>::new();

        bus.set_frame(100);
        bus.send(TestCommand { value: 1 });

        bus.set_frame(101);
        bus.send(TestCommand { value: 2 });

        let commands: Vec<_> = bus.drain().collect();
        assert_eq!(commands[0].meta.frame, Some(100));
        assert_eq!(commands[1].meta.frame, Some(101));
    }

    #[test]
    fn peek_without_remove() {
        let mut bus = CommandBus::<TestCommand>::new();
        bus.send(TestCommand { value: 42 });

        assert_eq!(bus.peek().unwrap().command.value, 42);
        assert_eq!(bus.len(), 1); // Still in queue
    }

    #[test]
    fn clear_queue() {
        let mut bus = CommandBus::<TestCommand>::new();
        bus.send(TestCommand { value: 1 });
        bus.send(TestCommand { value: 2 });

        assert_eq!(bus.len(), 2);
        bus.clear();
        assert!(bus.is_empty());
    }

    #[test]
    fn send_with_custom_meta() {
        let mut bus = CommandBus::<TestCommand>::new();

        let meta = CommandMeta::new(CommandId::new(999), 12345);
        bus.send_with_meta(TestCommand { value: 42 }, meta);

        let envelope = bus.drain().next().unwrap();
        assert_eq!(envelope.meta.id, CommandId::new(999));
        assert_eq!(envelope.meta.timestamp_ms, 12345);
    }
}
