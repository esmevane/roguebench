//! Platform-agnostic types for Roguebench.
//!
//! This crate contains pure data structures with no Bevy dependency,
//! enabling use in tooling, serialization, and non-game contexts.

pub mod commands;
pub mod items;
pub mod state_machine;

pub mod prelude {
    pub use crate::commands::{
        Command, CommandId, CommandMeta, CommandResult, Envelope, ValidationError,
    };
    pub use crate::items::{Effect, ItemDefinition, ItemId, ItemType};
    pub use crate::state_machine::{
        CompareOp, ConditionEvaluator, StateContext, StateDefinition, StateId,
        StateMachineDefinition, StateMachineResult, StateTransition, TransitionCondition,
        TransitionDefinition,
    };
}
