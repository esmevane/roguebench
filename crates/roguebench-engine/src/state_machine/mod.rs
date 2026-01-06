//! State machine Bevy integration.
//!
//! Provides components and systems for running state machines within the ECS.

mod component;
mod data;

pub use component::{
    StateMachine, StateMachineBundle, StateMachinePlugin, StateChanged,
};
