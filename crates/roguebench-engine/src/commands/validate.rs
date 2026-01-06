//! Command validation layer.
//!
//! Validators run before command execution to reject invalid commands.

use bevy::prelude::*;
use roguebench_core::commands::{Command, ValidationError};
use std::marker::PhantomData;

/// Trait for command validators.
///
/// Validators check commands before execution and can reject them
/// with a validation error.
///
/// # Example
///
/// ```ignore
/// struct PositiveDamageValidator;
///
/// impl Validator<DealDamage> for PositiveDamageValidator {
///     fn validate(&self, command: &DealDamage) -> Result<(), ValidationError> {
///         if command.amount <= 0 {
///             return Err(ValidationError::field(
///                 "deal_damage",
///                 "amount",
///                 "must be positive"
///             ));
///         }
///         Ok(())
///     }
/// }
/// ```
pub trait Validator<C: Command>: Send + Sync + 'static {
    /// Validate a command.
    ///
    /// Returns Ok(()) if valid, or Err(ValidationError) if invalid.
    fn validate(&self, command: &C) -> Result<(), ValidationError>;
}

/// A boxed validator for dynamic dispatch.
type BoxedValidator<C> = Box<dyn Validator<C>>;

/// Resource that holds validators for a command type.
#[derive(Resource)]
pub struct Validators<C: Command> {
    validators: Vec<BoxedValidator<C>>,
    _marker: PhantomData<C>,
}

impl<C: Command> Default for Validators<C> {
    fn default() -> Self {
        Self {
            validators: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<C: Command> Validators<C> {
    /// Create a new empty validator collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a validator.
    pub fn add<V: Validator<C>>(&mut self, validator: V) {
        self.validators.push(Box::new(validator));
    }

    /// Validate a command against all registered validators.
    ///
    /// Returns Ok(()) if all validators pass, or the first error encountered.
    pub fn validate(&self, command: &C) -> Result<(), ValidationError> {
        for validator in &self.validators {
            validator.validate(command)?;
        }
        Ok(())
    }

    /// Validate a command and collect all errors.
    ///
    /// Unlike `validate`, this runs all validators and returns all errors.
    pub fn validate_all(&self, command: &C) -> Vec<ValidationError> {
        self.validators
            .iter()
            .filter_map(|v| v.validate(command).err())
            .collect()
    }

    /// Check if any validators are registered.
    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }

    /// Get the number of registered validators.
    pub fn len(&self) -> usize {
        self.validators.len()
    }
}

/// A validator implemented as a closure.
pub struct FnValidator<C, F>
where
    C: Command,
    F: Fn(&C) -> Result<(), ValidationError> + Send + Sync + 'static,
{
    func: F,
    _marker: PhantomData<C>,
}

impl<C, F> FnValidator<C, F>
where
    C: Command,
    F: Fn(&C) -> Result<(), ValidationError> + Send + Sync + 'static,
{
    /// Create a new function-based validator.
    pub fn new(func: F) -> Self {
        Self {
            func,
            _marker: PhantomData,
        }
    }
}

impl<C, F> Validator<C> for FnValidator<C, F>
where
    C: Command,
    F: Fn(&C) -> Result<(), ValidationError> + Send + Sync + 'static,
{
    fn validate(&self, command: &C) -> Result<(), ValidationError> {
        (self.func)(command)
    }
}

/// Extension trait for registering validators.
pub trait ValidatorAppExt {
    /// Register the validators resource for a command type.
    fn register_validators<C: Command>(&mut self) -> &mut Self;

    /// Add a validator for a command type.
    ///
    /// This also registers the Validators resource if not present.
    fn add_validator<C: Command, V: Validator<C>>(&mut self, validator: V) -> &mut Self;

    /// Add a function-based validator for a command type.
    fn add_fn_validator<C, F>(&mut self, func: F) -> &mut Self
    where
        C: Command,
        F: Fn(&C) -> Result<(), ValidationError> + Send + Sync + 'static;
}

impl ValidatorAppExt for App {
    fn register_validators<C: Command>(&mut self) -> &mut Self {
        if !self.world().contains_resource::<Validators<C>>() {
            self.init_resource::<Validators<C>>();
        }
        self
    }

    fn add_validator<C: Command, V: Validator<C>>(&mut self, validator: V) -> &mut Self {
        self.register_validators::<C>();

        // Add validator to resource
        self.world_mut()
            .resource_mut::<Validators<C>>()
            .add(validator);

        self
    }

    fn add_fn_validator<C, F>(&mut self, func: F) -> &mut Self
    where
        C: Command,
        F: Fn(&C) -> Result<(), ValidationError> + Send + Sync + 'static,
    {
        self.add_validator::<C, _>(FnValidator::new(func))
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
        type Error = ();

        fn name() -> &'static str {
            "test"
        }
    }

    struct PositiveValidator;

    impl Validator<TestCommand> for PositiveValidator {
        fn validate(&self, command: &TestCommand) -> Result<(), ValidationError> {
            if command.value <= 0 {
                return Err(ValidationError::field("test", "value", "must be positive"));
            }
            Ok(())
        }
    }

    struct MaxValueValidator {
        max: i32,
    }

    impl Validator<TestCommand> for MaxValueValidator {
        fn validate(&self, command: &TestCommand) -> Result<(), ValidationError> {
            if command.value > self.max {
                return Err(ValidationError::field(
                    "test",
                    "value",
                    format!("must be <= {}", self.max),
                ));
            }
            Ok(())
        }
    }

    #[test]
    fn validator_passes() {
        let mut validators = Validators::<TestCommand>::new();
        validators.add(PositiveValidator);

        let cmd = TestCommand { value: 10 };
        assert!(validators.validate(&cmd).is_ok());
    }

    #[test]
    fn validator_fails() {
        let mut validators = Validators::<TestCommand>::new();
        validators.add(PositiveValidator);

        let cmd = TestCommand { value: -5 };
        let result = validators.validate(&cmd);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.field, Some("value".to_string()));
    }

    #[test]
    fn multiple_validators() {
        let mut validators = Validators::<TestCommand>::new();
        validators.add(PositiveValidator);
        validators.add(MaxValueValidator { max: 100 });

        // Both pass
        assert!(validators.validate(&TestCommand { value: 50 }).is_ok());

        // First fails
        assert!(validators.validate(&TestCommand { value: -1 }).is_err());

        // Second fails
        assert!(validators.validate(&TestCommand { value: 150 }).is_err());
    }

    #[test]
    fn validate_all_collects_errors() {
        let mut validators = Validators::<TestCommand>::new();
        validators.add(PositiveValidator);
        validators.add(MaxValueValidator { max: 100 });

        // Value that fails both (if we had that case) - but -1 only fails positive
        let errors = validators.validate_all(&TestCommand { value: -1 });
        assert_eq!(errors.len(), 1);

        // Value that passes
        let errors = validators.validate_all(&TestCommand { value: 50 });
        assert!(errors.is_empty());
    }

    #[test]
    fn fn_validator() {
        let mut validators = Validators::<TestCommand>::new();
        validators.add(FnValidator::new(|cmd: &TestCommand| {
            if cmd.value % 2 != 0 {
                return Err(ValidationError::field("test", "value", "must be even"));
            }
            Ok(())
        }));

        assert!(validators.validate(&TestCommand { value: 4 }).is_ok());
        assert!(validators.validate(&TestCommand { value: 3 }).is_err());
    }

    #[test]
    fn app_extension() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_validator::<TestCommand, _>(PositiveValidator)
            .add_fn_validator::<TestCommand, _>(|cmd| {
                if cmd.value > 1000 {
                    return Err(ValidationError::new("test", "value too large"));
                }
                Ok(())
            });

        app.update();

        let validators = app.world().resource::<Validators<TestCommand>>();
        assert_eq!(validators.len(), 2);
    }
}
