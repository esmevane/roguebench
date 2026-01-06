//! Command trait and core types for the command bus.
//!
//! All game mutations flow through commands. This module defines the
//! platform-agnostic traits and types used by the command system.

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// A unique identifier for a command instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandId(pub u64);

impl CommandId {
    /// Create a new command ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Trait for types that can be sent through the command bus.
///
/// Commands represent intentions to mutate game state. They are:
/// - Immutable once created
/// - Serializable for logging and replay
/// - Validated before execution
///
/// # Example
///
/// ```ignore
/// #[derive(Clone, Debug, Serialize, Deserialize)]
/// pub struct DealDamage {
///     pub target: Entity,
///     pub amount: i32,
///     pub source: Option<Entity>,
/// }
///
/// impl Command for DealDamage {
///     type Output = DamageResult;
///     type Error = DamageError;
///
///     fn name() -> &'static str {
///         "deal_damage"
///     }
/// }
/// ```
pub trait Command: Clone + Debug + Send + Sync + Serialize + for<'de> Deserialize<'de> + 'static {
    /// The result type when the command succeeds.
    type Output: Clone + Debug + Send + Sync + 'static;

    /// The error type when the command fails.
    type Error: Clone + Debug + Send + Sync + 'static;

    /// A unique name for this command type, used in logging.
    fn name() -> &'static str;
}

/// The result of executing a command.
#[derive(Debug, Clone)]
pub enum CommandResult<C: Command> {
    /// Command executed successfully.
    Success(C::Output),
    /// Command execution failed.
    Failed(C::Error),
    /// Command was rejected by validation.
    Rejected(ValidationError),
}

impl<C: Command> CommandResult<C> {
    /// Returns true if the command succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, CommandResult::Success(_))
    }

    /// Returns true if the command failed.
    pub fn is_failed(&self) -> bool {
        matches!(self, CommandResult::Failed(_))
    }

    /// Returns true if the command was rejected.
    pub fn is_rejected(&self) -> bool {
        matches!(self, CommandResult::Rejected(_))
    }

    /// Converts to an Option containing the success value.
    pub fn ok(self) -> Option<C::Output> {
        match self {
            CommandResult::Success(output) => Some(output),
            _ => None,
        }
    }
}

/// An error from command validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// The command type that was rejected.
    pub command_type: String,
    /// Human-readable reason for rejection.
    pub reason: String,
    /// Optional field that caused the error.
    pub field: Option<String>,
}

impl ValidationError {
    /// Create a new validation error.
    pub fn new(command_type: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            command_type: command_type.into(),
            reason: reason.into(),
            field: None,
        }
    }

    /// Create a validation error for a specific field.
    pub fn field(
        command_type: impl Into<String>,
        field: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            command_type: command_type.into(),
            reason: reason.into(),
            field: Some(field.into()),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(field) = &self.field {
            write!(f, "{}.{}: {}", self.command_type, field, self.reason)
        } else {
            write!(f, "{}: {}", self.command_type, self.reason)
        }
    }
}

impl std::error::Error for ValidationError {}

/// Metadata attached to a command when it enters the bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMeta {
    /// Unique ID for this command instance.
    pub id: CommandId,
    /// Timestamp when the command was sent (milliseconds since epoch).
    pub timestamp_ms: u64,
    /// Optional frame number when the command was sent.
    pub frame: Option<u64>,
}

impl CommandMeta {
    /// Create new command metadata with the given ID.
    pub fn new(id: CommandId, timestamp_ms: u64) -> Self {
        Self {
            id,
            timestamp_ms,
            frame: None,
        }
    }

    /// Set the frame number.
    pub fn with_frame(mut self, frame: u64) -> Self {
        self.frame = Some(frame);
        self
    }
}

/// A command bundled with its metadata, ready for processing.
#[derive(Debug, Clone)]
pub struct Envelope<C: Command> {
    /// The command to execute.
    pub command: C,
    /// Metadata about when/how the command was sent.
    pub meta: CommandMeta,
}

impl<C: Command> Envelope<C> {
    /// Create a new envelope wrapping a command.
    pub fn new(command: C, meta: CommandMeta) -> Self {
        Self { command, meta }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct TestCommand {
        value: i32,
    }

    impl Command for TestCommand {
        type Output = i32;
        type Error = String;

        fn name() -> &'static str {
            "test_command"
        }
    }

    #[test]
    fn command_id_equality() {
        let id1 = CommandId::new(42);
        let id2 = CommandId::new(42);
        let id3 = CommandId::new(43);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn command_result_success() {
        let result: CommandResult<TestCommand> = CommandResult::Success(42);
        assert!(result.is_success());
        assert!(!result.is_failed());
        assert!(!result.is_rejected());
        assert_eq!(result.ok(), Some(42));
    }

    #[test]
    fn command_result_failed() {
        let result: CommandResult<TestCommand> = CommandResult::Failed("error".to_string());
        assert!(!result.is_success());
        assert!(result.is_failed());
        assert!(!result.is_rejected());
        assert_eq!(result.ok(), None);
    }

    #[test]
    fn command_result_rejected() {
        let error = ValidationError::new("test_command", "invalid state");
        let result: CommandResult<TestCommand> = CommandResult::Rejected(error);
        assert!(!result.is_success());
        assert!(!result.is_failed());
        assert!(result.is_rejected());
    }

    #[test]
    fn validation_error_display() {
        let error = ValidationError::new("deal_damage", "target not found");
        assert_eq!(error.to_string(), "deal_damage: target not found");

        let field_error = ValidationError::field("deal_damage", "amount", "must be positive");
        assert_eq!(
            field_error.to_string(),
            "deal_damage.amount: must be positive"
        );
    }

    #[test]
    fn envelope_creation() {
        let cmd = TestCommand { value: 100 };
        let meta = CommandMeta::new(CommandId::new(1), 1234567890);
        let envelope = Envelope::new(cmd.clone(), meta);

        assert_eq!(envelope.command.value, 100);
        assert_eq!(envelope.meta.id, CommandId::new(1));
    }

    #[test]
    fn command_serialization() {
        let cmd = TestCommand { value: 42 };
        let json = serde_json::to_string(&cmd).unwrap();
        let parsed: TestCommand = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.value, 42);
    }
}
