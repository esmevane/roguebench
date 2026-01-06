//! ContentType implementation for StateMachineDefinition.

use crate::data::{ContentType, Database, LoadError};
use roguebench_core::state_machine::StateMachineDefinition;

impl ContentType for StateMachineDefinition {
    type Id = String;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn load_all(db: &Database) -> Result<Vec<Self>, LoadError> {
        db.get_all_state_machines()
            .map_err(LoadError::from)
    }

    fn type_name() -> &'static str {
        "state_machine"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::DatabaseError;
    use roguebench_core::state_machine::{
        StateDefinition, TransitionCondition, TransitionDefinition,
    };

    #[test]
    fn state_machine_crud_operations() {
        let db = Database::in_memory().unwrap();

        // Create
        let machine = StateMachineDefinition::new("enemy_ai", "idle")
            .with_name("Enemy AI")
            .with_state(StateDefinition::new("idle").with_name("Idle"))
            .with_state(StateDefinition::new("patrol").with_name("Patrolling"))
            .with_state(StateDefinition::new("chase").with_name("Chasing"))
            .with_transition(
                TransitionDefinition::new("idle", "patrol")
                    .when(TransitionCondition::after(2.0))
                    .with_name("start_patrol"),
            )
            .with_transition(
                TransitionDefinition::new("patrol", "chase")
                    .when(TransitionCondition::flag("player_spotted", true))
                    .with_name("spot_player"),
            );

        db.upsert_state_machine(&machine).unwrap();
        assert_eq!(db.count_state_machines().unwrap(), 1);

        // Read
        let loaded = db.get_state_machine("enemy_ai").unwrap();
        assert_eq!(loaded.name, "Enemy AI");
        assert_eq!(loaded.initial_state.0, "idle");
        assert_eq!(loaded.states.len(), 3);
        assert_eq!(loaded.transitions.len(), 2);

        // Update
        let updated = StateMachineDefinition::new("enemy_ai", "patrol")
            .with_name("Updated Enemy AI")
            .with_state(StateDefinition::new("patrol"))
            .with_state(StateDefinition::new("attack"));

        db.upsert_state_machine(&updated).unwrap();
        assert_eq!(db.count_state_machines().unwrap(), 1);

        let reloaded = db.get_state_machine("enemy_ai").unwrap();
        assert_eq!(reloaded.name, "Updated Enemy AI");
        assert_eq!(reloaded.initial_state.0, "patrol");
        assert_eq!(reloaded.states.len(), 2);

        // Delete
        assert!(db.delete_state_machine("enemy_ai").unwrap());
        assert_eq!(db.count_state_machines().unwrap(), 0);
    }

    #[test]
    fn get_all_state_machines() {
        let db = Database::in_memory().unwrap();

        let machines = vec![
            StateMachineDefinition::new("ai_a", "idle").with_state(StateDefinition::new("idle")),
            StateMachineDefinition::new("ai_b", "start").with_state(StateDefinition::new("start")),
            StateMachineDefinition::new("ai_c", "waiting")
                .with_state(StateDefinition::new("waiting")),
        ];

        for m in &machines {
            db.upsert_state_machine(m).unwrap();
        }

        let all = db.get_all_state_machines().unwrap();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn state_machine_not_found() {
        let db = Database::in_memory().unwrap();
        let result = db.get_state_machine("nonexistent");
        assert!(matches!(result, Err(DatabaseError::ItemNotFound(_))));
    }

    #[test]
    fn content_type_implementation() {
        let db = Database::in_memory().unwrap();

        let machine =
            StateMachineDefinition::new("test_ai", "idle").with_state(StateDefinition::new("idle"));

        db.upsert_state_machine(&machine).unwrap();

        // Use ContentType trait
        let loaded = StateMachineDefinition::load_all(&db).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id(), "test_ai");
        assert_eq!(StateMachineDefinition::type_name(), "state_machine");
    }

    #[test]
    fn complex_state_machine_serialization() {
        let db = Database::in_memory().unwrap();

        // Create a complex state machine with various conditions
        let machine = StateMachineDefinition::new("boss_ai", "idle")
            .with_name("Boss AI")
            .with_state(
                StateDefinition::new("idle")
                    .with_name("Idle Phase")
                    .with_metadata("animation", serde_json::json!("boss_idle")),
            )
            .with_state(
                StateDefinition::new("phase1")
                    .with_name("Attack Phase 1")
                    .with_metadata("damage_multiplier", serde_json::json!(1.0)),
            )
            .with_state(
                StateDefinition::new("phase2")
                    .with_name("Attack Phase 2")
                    .with_metadata("damage_multiplier", serde_json::json!(1.5)),
            )
            .with_state(StateDefinition::new("enraged").with_name("Enraged"))
            .with_transition(
                TransitionDefinition::new("idle", "phase1")
                    .when(TransitionCondition::flag("combat_started", true))
                    .with_priority(10),
            )
            .with_transition(
                TransitionDefinition::new("phase1", "phase2").when(
                    TransitionCondition::threshold(
                        "health_percent",
                        roguebench_core::state_machine::CompareOp::Lt,
                        50.0,
                    ),
                ),
            )
            .with_transition(
                TransitionDefinition::new("phase2", "enraged").when(
                    TransitionCondition::threshold(
                        "health_percent",
                        roguebench_core::state_machine::CompareOp::Lt,
                        25.0,
                    )
                    .and(TransitionCondition::flag("enrage_ready", true)),
                ),
            );

        db.upsert_state_machine(&machine).unwrap();

        let loaded = db.get_state_machine("boss_ai").unwrap();
        assert_eq!(loaded.states.len(), 4);
        assert_eq!(loaded.transitions.len(), 3);

        // Verify metadata preserved
        let idle_state = loaded.get_state(&"idle".into()).unwrap();
        assert_eq!(
            idle_state.metadata.get("animation"),
            Some(&serde_json::json!("boss_idle"))
        );
    }
}
