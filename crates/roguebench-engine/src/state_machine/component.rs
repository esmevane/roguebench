//! State machine Bevy component.

use bevy::prelude::*;
use roguebench_core::state_machine::{
    ConditionEvaluator, StateContext, StateId, StateMachineDefinition, StateTransition,
    TransitionCondition,
};

/// Event emitted when a state machine changes state.
#[derive(Event, Debug, Clone)]
pub struct StateChanged {
    /// Entity whose state changed.
    pub entity: Entity,
    /// The transition that occurred.
    pub transition: StateTransition,
}

/// Component that runs a state machine for an entity.
#[derive(Component)]
pub struct StateMachine {
    /// The state machine definition.
    definition: StateMachineDefinition,
    /// Current state ID.
    current_state: StateId,
    /// Context for condition evaluation.
    context: StateContext,
    /// Whether an update is needed this frame.
    needs_update: bool,
    /// Manually triggered transition target.
    manual_transition: Option<StateId>,
}

impl StateMachine {
    /// Create a new state machine from a definition.
    pub fn new(definition: StateMachineDefinition) -> Self {
        let initial = definition.initial_state.clone();
        Self {
            definition,
            current_state: initial,
            context: StateContext::new(),
            needs_update: true,
            manual_transition: None,
        }
    }

    /// Get the current state ID.
    pub fn current_state(&self) -> &StateId {
        &self.current_state
    }

    /// Get the state machine definition.
    pub fn definition(&self) -> &StateMachineDefinition {
        &self.definition
    }

    /// Get mutable access to the context.
    pub fn context_mut(&mut self) -> &mut StateContext {
        &mut self.context
    }

    /// Get the context.
    pub fn context(&self) -> &StateContext {
        &self.context
    }

    /// Set a flag in the context.
    pub fn set_flag(&mut self, name: impl Into<String>, value: bool) {
        self.context.set_flag(name, value);
        self.needs_update = true;
    }

    /// Get a flag from the context.
    pub fn get_flag(&self, name: &str) -> bool {
        self.context.get_flag(name)
    }

    /// Set a value in the context.
    pub fn set_value(&mut self, name: impl Into<String>, value: f32) {
        self.context.set_value(name, value);
        self.needs_update = true;
    }

    /// Get a value from the context.
    pub fn get_value(&self, name: &str) -> f32 {
        self.context.get_value(name)
    }

    /// Trigger a manual transition to a state.
    ///
    /// The transition will occur on the next update if the target state exists.
    pub fn trigger_transition(&mut self, to: impl Into<StateId>) {
        self.manual_transition = Some(to.into());
        self.needs_update = true;
    }

    /// Check if a specific state is active.
    pub fn is_in_state(&self, state: &str) -> bool {
        self.current_state.0 == state
    }

    /// Update the state machine, returning a transition if one occurred.
    ///
    /// This should be called each frame with the delta time.
    pub fn update(&mut self, delta: f32) -> Option<StateTransition> {
        // Update time in state
        self.context.add_time(delta);

        // Check for manual transition first
        if let Some(target) = self.manual_transition.take()
            && self.definition.get_state(&target).is_some()
        {
            let transition = StateTransition {
                from: self.current_state.clone(),
                to: target.clone(),
                time_in_previous: self.context.time_in_state,
            };
            self.current_state = target;
            self.context.reset_time();
            return Some(transition);
        }

        // Check automatic transitions
        if self.needs_update || self.context.time_in_state < delta * 2.0 {
            self.needs_update = false;

            for trans_def in self.definition.get_transitions_from(&self.current_state) {
                if self.evaluate_condition(&trans_def.condition) {
                    let transition = StateTransition {
                        from: self.current_state.clone(),
                        to: trans_def.to.clone(),
                        time_in_previous: self.context.time_in_state,
                    };
                    self.current_state = trans_def.to.clone();
                    self.context.reset_time();
                    return Some(transition);
                }
            }
        }

        None
    }

    /// Evaluate a transition condition.
    fn evaluate_condition(&self, condition: &TransitionCondition) -> bool {
        ConditionEvaluator::evaluate(condition, &self.context)
    }

    /// Force immediate evaluation of transitions.
    pub fn force_update(&mut self) {
        self.needs_update = true;
    }
}

/// Bundle for spawning entities with a state machine.
#[derive(Bundle)]
pub struct StateMachineBundle {
    /// The state machine component.
    pub state_machine: StateMachine,
}

impl StateMachineBundle {
    /// Create a new bundle from a definition.
    pub fn new(definition: StateMachineDefinition) -> Self {
        Self {
            state_machine: StateMachine::new(definition),
        }
    }
}

/// System that updates all state machines and emits state change events.
pub fn update_state_machines(
    time: Res<Time>,
    mut query: Query<(Entity, &mut StateMachine)>,
    mut events: EventWriter<StateChanged>,
) {
    let delta = time.delta_secs();

    for (entity, mut machine) in query.iter_mut() {
        if let Some(transition) = machine.update(delta) {
            events.send(StateChanged { entity, transition });
        }
    }
}

/// Plugin that sets up state machine systems.
pub struct StateMachinePlugin;

impl Plugin for StateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StateChanged>()
            .add_systems(Update, update_state_machines);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roguebench_core::state_machine::{StateDefinition, TransitionCondition, TransitionDefinition};

    fn simple_machine() -> StateMachineDefinition {
        StateMachineDefinition::new("test", "idle")
            .with_state(StateDefinition::new("idle"))
            .with_state(StateDefinition::new("active"))
            .with_state(StateDefinition::new("done"))
    }

    #[test]
    fn create_state_machine() {
        let def = simple_machine();
        let machine = StateMachine::new(def);

        assert_eq!(machine.current_state().0, "idle");
        assert!(machine.is_in_state("idle"));
        assert!(!machine.is_in_state("active"));
    }

    #[test]
    fn manual_transition() {
        let def = simple_machine();
        let mut machine = StateMachine::new(def);

        machine.trigger_transition("active");
        let result = machine.update(0.016);

        assert!(result.is_some());
        let transition = result.unwrap();
        assert_eq!(transition.from.0, "idle");
        assert_eq!(transition.to.0, "active");
        assert!(machine.is_in_state("active"));
    }

    #[test]
    fn manual_transition_invalid_state() {
        let def = simple_machine();
        let mut machine = StateMachine::new(def);

        machine.trigger_transition("nonexistent");
        let result = machine.update(0.016);

        assert!(result.is_none());
        assert!(machine.is_in_state("idle"));
    }

    #[test]
    fn flag_triggered_transition() {
        let def = StateMachineDefinition::new("test", "idle")
            .with_state(StateDefinition::new("idle"))
            .with_state(StateDefinition::new("active"))
            .with_transition(
                TransitionDefinition::new("idle", "active")
                    .when(TransitionCondition::flag("go", true)),
            );

        let mut machine = StateMachine::new(def);

        // No transition yet
        let result = machine.update(0.016);
        assert!(result.is_none());
        assert!(machine.is_in_state("idle"));

        // Set flag and update
        machine.set_flag("go", true);
        let result = machine.update(0.016);

        assert!(result.is_some());
        assert!(machine.is_in_state("active"));
    }

    #[test]
    fn timed_transition() {
        let def = StateMachineDefinition::new("test", "idle")
            .with_state(StateDefinition::new("idle"))
            .with_state(StateDefinition::new("active"))
            .with_transition(
                TransitionDefinition::new("idle", "active")
                    .when(TransitionCondition::after(1.0)),
            );

        let mut machine = StateMachine::new(def);

        // Not enough time
        machine.update(0.5);
        assert!(machine.is_in_state("idle"));

        // More time
        machine.update(0.5);
        assert!(machine.is_in_state("idle")); // Still checking

        // Force update to re-evaluate
        machine.force_update();
        let result = machine.update(0.1);
        assert!(result.is_some());
        assert!(machine.is_in_state("active"));
    }

    #[test]
    fn transition_priority() {
        let def = StateMachineDefinition::new("test", "idle")
            .with_state(StateDefinition::new("idle"))
            .with_state(StateDefinition::new("low_priority"))
            .with_state(StateDefinition::new("high_priority"))
            .with_transition(
                TransitionDefinition::new("idle", "low_priority")
                    .when(TransitionCondition::always())
                    .with_priority(1),
            )
            .with_transition(
                TransitionDefinition::new("idle", "high_priority")
                    .when(TransitionCondition::always())
                    .with_priority(10),
            );

        let mut machine = StateMachine::new(def);
        machine.update(0.016);

        // Should take high priority transition
        assert!(machine.is_in_state("high_priority"));
    }

    #[test]
    fn context_values() {
        let def = StateMachineDefinition::new("test", "idle")
            .with_state(StateDefinition::new("idle"))
            .with_state(StateDefinition::new("low_health"))
            .with_transition(
                TransitionDefinition::new("idle", "low_health")
                    .when(TransitionCondition::threshold(
                        "health",
                        roguebench_core::state_machine::CompareOp::Lt,
                        25.0,
                    )),
            );

        let mut machine = StateMachine::new(def);
        machine.set_value("health", 100.0);

        machine.update(0.016);
        assert!(machine.is_in_state("idle"));

        machine.set_value("health", 20.0);
        machine.update(0.016);
        assert!(machine.is_in_state("low_health"));
    }

    #[test]
    fn transition_time_tracking() {
        let def = simple_machine();
        let mut machine = StateMachine::new(def);

        // Accumulate time
        machine.update(0.5);
        machine.update(0.3);
        assert!((machine.context().time_in_state - 0.8).abs() < 0.01);

        // Transition resets time
        machine.trigger_transition("active");
        let transition = machine.update(0.016).unwrap();

        assert!((transition.time_in_previous - 0.816).abs() < 0.02);
        assert!(machine.context().time_in_state < 0.1);
    }

    #[test]
    fn plugin_adds_event_and_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(StateMachinePlugin);

        // Spawn an entity with a state machine
        let def = StateMachineDefinition::new("test", "a")
            .with_state(StateDefinition::new("a"))
            .with_state(StateDefinition::new("b"))
            .with_transition(
                TransitionDefinition::new("a", "b").when(TransitionCondition::always()),
            );

        let entity = app.world_mut().spawn(StateMachineBundle::new(def)).id();

        // Run a frame
        app.update();

        // Check that the state machine transitioned
        let machine = app.world().get::<StateMachine>(entity).unwrap();
        assert!(machine.is_in_state("b"));
    }

    #[test]
    fn state_changed_events_emitted() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(StateMachinePlugin);

        let def = StateMachineDefinition::new("test", "a")
            .with_state(StateDefinition::new("a"))
            .with_state(StateDefinition::new("b"))
            .with_transition(
                TransitionDefinition::new("a", "b").when(TransitionCondition::always()),
            );

        let entity = app.world_mut().spawn(StateMachineBundle::new(def)).id();

        app.update();

        // Check events
        let events = app.world().resource::<Events<StateChanged>>();
        let mut reader = events.get_cursor();
        let transitions: Vec<_> = reader.read(events).collect();

        assert_eq!(transitions.len(), 1);
        assert_eq!(transitions[0].entity, entity);
        assert_eq!(transitions[0].transition.from.0, "a");
        assert_eq!(transitions[0].transition.to.0, "b");
    }
}
