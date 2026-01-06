//! State machine abstraction for data-driven behaviors.
//!
//! # Architecture
//!
//! A state machine consists of:
//! - **States**: Named nodes with enter/exit/update logic
//! - **Transitions**: Edges between states with conditions
//! - **Context**: Shared data available to all states
//!
//! # Example
//!
//! ```ignore
//! // Define states
//! let mut machine = StateMachine::new("idle");
//! machine.add_state(State::new("idle"));
//! machine.add_state(State::new("attacking"));
//!
//! // Define transitions
//! machine.add_transition(Transition::new("idle", "attacking")
//!     .when(|ctx| ctx.target_in_range()));
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

/// Unique identifier for a state.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StateId(pub String);

impl StateId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for StateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for StateId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for StateId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Definition of a state within a state machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDefinition {
    /// Unique identifier for this state.
    pub id: StateId,
    /// Display name for debugging/editing.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Custom data attached to this state (for scripting).
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl StateDefinition {
    /// Create a new state definition.
    pub fn new(id: impl Into<StateId>) -> Self {
        let id = id.into();
        Self {
            name: id.0.clone(),
            id,
            description: None,
            metadata: HashMap::new(),
        }
    }

    /// Set the display name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}

/// Condition for a transition to fire.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransitionCondition {
    /// Always transitions.
    Always,
    /// Never transitions (must be triggered manually).
    Manual,
    /// Transitions after a duration (in seconds).
    After { seconds: f32 },
    /// Transitions when a flag is set.
    Flag { name: String, value: bool },
    /// Transitions when a numeric value meets a threshold.
    Threshold {
        name: String,
        op: CompareOp,
        value: f32,
    },
    /// Custom condition evaluated by script.
    Script { handler: String },
    /// All conditions must be true.
    All { conditions: Vec<TransitionCondition> },
    /// Any condition must be true.
    Any { conditions: Vec<TransitionCondition> },
    /// Negates a condition.
    Not { condition: Box<TransitionCondition> },
}

impl TransitionCondition {
    /// Create an "always" condition.
    pub fn always() -> Self {
        Self::Always
    }

    /// Create a "manual" condition.
    pub fn manual() -> Self {
        Self::Manual
    }

    /// Create a timer condition.
    pub fn after(seconds: f32) -> Self {
        Self::After { seconds }
    }

    /// Create a flag condition.
    pub fn flag(name: impl Into<String>, value: bool) -> Self {
        Self::Flag {
            name: name.into(),
            value,
        }
    }

    /// Create a threshold condition.
    pub fn threshold(name: impl Into<String>, op: CompareOp, value: f32) -> Self {
        Self::Threshold {
            name: name.into(),
            op,
            value,
        }
    }

    /// Create a script condition.
    pub fn script(handler: impl Into<String>) -> Self {
        Self::Script {
            handler: handler.into(),
        }
    }

    /// Combine with AND.
    pub fn and(self, other: Self) -> Self {
        match self {
            Self::All { mut conditions } => {
                conditions.push(other);
                Self::All { conditions }
            }
            _ => Self::All {
                conditions: vec![self, other],
            },
        }
    }

    /// Combine with OR.
    pub fn or(self, other: Self) -> Self {
        match self {
            Self::Any { mut conditions } => {
                conditions.push(other);
                Self::Any { conditions }
            }
            _ => Self::Any {
                conditions: vec![self, other],
            },
        }
    }

    /// Negate the condition.
    pub fn negate(self) -> Self {
        Self::Not {
            condition: Box::new(self),
        }
    }
}

/// Comparison operators for threshold conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl CompareOp {
    /// Evaluate the comparison.
    pub fn eval(self, left: f32, right: f32) -> bool {
        match self {
            Self::Eq => (left - right).abs() < f32::EPSILON,
            Self::Ne => (left - right).abs() >= f32::EPSILON,
            Self::Lt => left < right,
            Self::Le => left <= right,
            Self::Gt => left > right,
            Self::Ge => left >= right,
        }
    }
}

/// Definition of a transition between states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionDefinition {
    /// Source state ID.
    pub from: StateId,
    /// Target state ID.
    pub to: StateId,
    /// Condition that triggers this transition.
    pub condition: TransitionCondition,
    /// Priority (higher values checked first).
    pub priority: i32,
    /// Optional name for debugging.
    pub name: Option<String>,
}

impl TransitionDefinition {
    /// Create a new transition.
    pub fn new(from: impl Into<StateId>, to: impl Into<StateId>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            condition: TransitionCondition::Manual,
            priority: 0,
            name: None,
        }
    }

    /// Set the transition condition.
    pub fn when(mut self, condition: TransitionCondition) -> Self {
        self.condition = condition;
        self
    }

    /// Set the priority.
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Set the name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// Complete definition of a state machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachineDefinition {
    /// Unique identifier for this state machine type.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Initial state when the machine starts.
    pub initial_state: StateId,
    /// All states in the machine.
    pub states: Vec<StateDefinition>,
    /// All transitions between states.
    pub transitions: Vec<TransitionDefinition>,
}

impl StateMachineDefinition {
    /// Create a new state machine definition.
    pub fn new(id: impl Into<String>, initial_state: impl Into<StateId>) -> Self {
        Self {
            id: id.into(),
            name: String::new(),
            initial_state: initial_state.into(),
            states: Vec::new(),
            transitions: Vec::new(),
        }
    }

    /// Set the display name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Add a state.
    pub fn with_state(mut self, state: StateDefinition) -> Self {
        self.states.push(state);
        self
    }

    /// Add a transition.
    pub fn with_transition(mut self, transition: TransitionDefinition) -> Self {
        self.transitions.push(transition);
        self
    }

    /// Get a state by ID.
    pub fn get_state(&self, id: &StateId) -> Option<&StateDefinition> {
        self.states.iter().find(|s| &s.id == id)
    }

    /// Get all transitions from a state, sorted by priority.
    pub fn get_transitions_from(&self, from: &StateId) -> Vec<&TransitionDefinition> {
        let mut transitions: Vec<_> = self.transitions.iter().filter(|t| &t.from == from).collect();
        transitions.sort_by(|a, b| b.priority.cmp(&a.priority));
        transitions
    }
}

/// Context data for condition evaluation.
#[derive(Debug, Clone, Default)]
pub struct StateContext {
    /// Boolean flags.
    pub flags: HashMap<String, bool>,
    /// Numeric values.
    pub values: HashMap<String, f32>,
    /// Time spent in current state (seconds).
    pub time_in_state: f32,
}

impl StateContext {
    /// Create a new empty context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a flag.
    pub fn set_flag(&mut self, name: impl Into<String>, value: bool) {
        self.flags.insert(name.into(), value);
    }

    /// Get a flag (defaults to false).
    pub fn get_flag(&self, name: &str) -> bool {
        self.flags.get(name).copied().unwrap_or(false)
    }

    /// Set a value.
    pub fn set_value(&mut self, name: impl Into<String>, value: f32) {
        self.values.insert(name.into(), value);
    }

    /// Get a value (defaults to 0.0).
    pub fn get_value(&self, name: &str) -> f32 {
        self.values.get(name).copied().unwrap_or(0.0)
    }

    /// Reset time in state (called on state change).
    pub fn reset_time(&mut self) {
        self.time_in_state = 0.0;
    }

    /// Add elapsed time (called each update).
    pub fn add_time(&mut self, delta: f32) {
        self.time_in_state += delta;
    }
}

/// Event emitted when a state transition occurs.
#[derive(Debug, Clone)]
pub struct StateTransition {
    /// The state being exited.
    pub from: StateId,
    /// The state being entered.
    pub to: StateId,
    /// Time spent in the previous state.
    pub time_in_previous: f32,
}

/// Result of evaluating a state machine update.
#[derive(Debug, Clone)]
pub enum StateMachineResult {
    /// No transition occurred, stay in current state.
    Stay,
    /// Transition to a new state.
    Transition(StateTransition),
}

/// Evaluates conditions against a context.
pub struct ConditionEvaluator;

impl ConditionEvaluator {
    /// Evaluate a condition against the given context.
    ///
    /// Returns true if the condition is met, false otherwise.
    /// Script conditions always return false (must be handled externally).
    pub fn evaluate(condition: &TransitionCondition, context: &StateContext) -> bool {
        match condition {
            TransitionCondition::Always => true,
            TransitionCondition::Manual => false,
            TransitionCondition::After { seconds } => context.time_in_state >= *seconds,
            TransitionCondition::Flag { name, value } => context.get_flag(name) == *value,
            TransitionCondition::Threshold { name, op, value } => {
                op.eval(context.get_value(name), *value)
            }
            TransitionCondition::Script { .. } => {
                // Script conditions must be evaluated externally
                false
            }
            TransitionCondition::All { conditions } => {
                conditions.iter().all(|c| Self::evaluate(c, context))
            }
            TransitionCondition::Any { conditions } => {
                conditions.iter().any(|c| Self::evaluate(c, context))
            }
            TransitionCondition::Not { condition } => !Self::evaluate(condition, context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_definition_builder() {
        let state = StateDefinition::new("idle")
            .with_name("Idle State")
            .with_description("Character is standing still")
            .with_metadata("animation", serde_json::json!("idle_anim"));

        assert_eq!(state.id.0, "idle");
        assert_eq!(state.name, "Idle State");
        assert_eq!(
            state.description,
            Some("Character is standing still".to_string())
        );
        assert!(state.metadata.contains_key("animation"));
    }

    #[test]
    fn transition_definition_builder() {
        let transition = TransitionDefinition::new("idle", "walking")
            .when(TransitionCondition::flag("is_moving", true))
            .with_priority(10)
            .with_name("start_walking");

        assert_eq!(transition.from.0, "idle");
        assert_eq!(transition.to.0, "walking");
        assert_eq!(transition.priority, 10);
        assert_eq!(transition.name, Some("start_walking".to_string()));
    }

    #[test]
    fn state_machine_definition_builder() {
        let machine = StateMachineDefinition::new("enemy_ai", "idle")
            .with_name("Enemy AI")
            .with_state(StateDefinition::new("idle"))
            .with_state(StateDefinition::new("patrol"))
            .with_state(StateDefinition::new("chase"))
            .with_transition(
                TransitionDefinition::new("idle", "patrol")
                    .when(TransitionCondition::after(2.0)),
            )
            .with_transition(
                TransitionDefinition::new("patrol", "chase")
                    .when(TransitionCondition::flag("player_spotted", true)),
            );

        assert_eq!(machine.id, "enemy_ai");
        assert_eq!(machine.initial_state.0, "idle");
        assert_eq!(machine.states.len(), 3);
        assert_eq!(machine.transitions.len(), 2);
    }

    #[test]
    fn get_transitions_from_state() {
        let machine = StateMachineDefinition::new("test", "a")
            .with_state(StateDefinition::new("a"))
            .with_state(StateDefinition::new("b"))
            .with_state(StateDefinition::new("c"))
            .with_transition(
                TransitionDefinition::new("a", "b")
                    .when(TransitionCondition::always())
                    .with_priority(1),
            )
            .with_transition(
                TransitionDefinition::new("a", "c")
                    .when(TransitionCondition::always())
                    .with_priority(10),
            )
            .with_transition(TransitionDefinition::new("b", "a"));

        let transitions = machine.get_transitions_from(&StateId::new("a"));
        assert_eq!(transitions.len(), 2);
        // Higher priority first
        assert_eq!(transitions[0].to.0, "c");
        assert_eq!(transitions[1].to.0, "b");
    }

    #[test]
    fn condition_evaluation_always() {
        let ctx = StateContext::new();
        assert!(ConditionEvaluator::evaluate(
            &TransitionCondition::always(),
            &ctx
        ));
    }

    #[test]
    fn condition_evaluation_manual() {
        let ctx = StateContext::new();
        assert!(!ConditionEvaluator::evaluate(
            &TransitionCondition::manual(),
            &ctx
        ));
    }

    #[test]
    fn condition_evaluation_after() {
        let mut ctx = StateContext::new();
        ctx.time_in_state = 1.0;

        assert!(!ConditionEvaluator::evaluate(
            &TransitionCondition::after(2.0),
            &ctx
        ));

        ctx.time_in_state = 2.5;
        assert!(ConditionEvaluator::evaluate(
            &TransitionCondition::after(2.0),
            &ctx
        ));
    }

    #[test]
    fn condition_evaluation_flag() {
        let mut ctx = StateContext::new();
        ctx.set_flag("ready", true);

        assert!(ConditionEvaluator::evaluate(
            &TransitionCondition::flag("ready", true),
            &ctx
        ));
        assert!(!ConditionEvaluator::evaluate(
            &TransitionCondition::flag("ready", false),
            &ctx
        ));
        assert!(!ConditionEvaluator::evaluate(
            &TransitionCondition::flag("other", true),
            &ctx
        ));
    }

    #[test]
    fn condition_evaluation_threshold() {
        let mut ctx = StateContext::new();
        ctx.set_value("health", 50.0);

        assert!(ConditionEvaluator::evaluate(
            &TransitionCondition::threshold("health", CompareOp::Lt, 100.0),
            &ctx
        ));
        assert!(!ConditionEvaluator::evaluate(
            &TransitionCondition::threshold("health", CompareOp::Lt, 25.0),
            &ctx
        ));
        assert!(ConditionEvaluator::evaluate(
            &TransitionCondition::threshold("health", CompareOp::Ge, 50.0),
            &ctx
        ));
    }

    #[test]
    fn condition_evaluation_combined() {
        let mut ctx = StateContext::new();
        ctx.set_flag("player_spotted", true);
        ctx.time_in_state = 3.0;

        // AND: both must be true
        let and_cond = TransitionCondition::flag("player_spotted", true)
            .and(TransitionCondition::after(2.0));
        assert!(ConditionEvaluator::evaluate(&and_cond, &ctx));

        // OR: one must be true
        let or_cond = TransitionCondition::flag("wrong_flag", true)
            .or(TransitionCondition::after(2.0));
        assert!(ConditionEvaluator::evaluate(&or_cond, &ctx));

        // NOT: negate
        let not_cond = TransitionCondition::flag("player_spotted", false).negate();
        assert!(ConditionEvaluator::evaluate(&not_cond, &ctx));
    }

    #[test]
    fn context_operations() {
        let mut ctx = StateContext::new();

        // Flags
        assert!(!ctx.get_flag("test"));
        ctx.set_flag("test", true);
        assert!(ctx.get_flag("test"));

        // Values
        assert_eq!(ctx.get_value("score"), 0.0);
        ctx.set_value("score", 100.0);
        assert_eq!(ctx.get_value("score"), 100.0);

        // Time
        ctx.add_time(0.5);
        ctx.add_time(0.3);
        assert!((ctx.time_in_state - 0.8).abs() < f32::EPSILON);

        ctx.reset_time();
        assert_eq!(ctx.time_in_state, 0.0);
    }

    #[test]
    fn serialization_roundtrip() {
        let machine = StateMachineDefinition::new("test", "idle")
            .with_name("Test Machine")
            .with_state(StateDefinition::new("idle"))
            .with_state(StateDefinition::new("active"))
            .with_transition(
                TransitionDefinition::new("idle", "active")
                    .when(TransitionCondition::flag("activate", true).and(
                        TransitionCondition::threshold("power", CompareOp::Ge, 10.0),
                    ))
                    .with_priority(5),
            );

        let json = serde_json::to_string(&machine).unwrap();
        let parsed: StateMachineDefinition = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, machine.id);
        assert_eq!(parsed.name, machine.name);
        assert_eq!(parsed.states.len(), machine.states.len());
        assert_eq!(parsed.transitions.len(), machine.transitions.len());
    }

    #[test]
    fn compare_op_evaluation() {
        assert!(CompareOp::Eq.eval(5.0, 5.0));
        assert!(!CompareOp::Eq.eval(5.0, 6.0));

        assert!(CompareOp::Ne.eval(5.0, 6.0));
        assert!(!CompareOp::Ne.eval(5.0, 5.0));

        assert!(CompareOp::Lt.eval(5.0, 10.0));
        assert!(!CompareOp::Lt.eval(10.0, 5.0));

        assert!(CompareOp::Le.eval(5.0, 5.0));
        assert!(CompareOp::Le.eval(5.0, 10.0));

        assert!(CompareOp::Gt.eval(10.0, 5.0));
        assert!(!CompareOp::Gt.eval(5.0, 10.0));

        assert!(CompareOp::Ge.eval(5.0, 5.0));
        assert!(CompareOp::Ge.eval(10.0, 5.0));
    }
}
