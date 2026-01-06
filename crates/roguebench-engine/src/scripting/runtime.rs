//! Core Lua runtime integration.
//!
//! Provides a Bevy resource wrapping mlua's Lua instance with Luau dialect.

use bevy::prelude::*;
use mlua::{Lua, Result as LuaResult, StdLib, Value};
use std::path::Path;
use thiserror::Error;

/// Error type for scripting operations.
#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),

    #[error("Script not found: {0}")]
    NotFound(String),

    #[error("Invalid module: {0}")]
    InvalidModule(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Bevy resource containing the Lua runtime.
///
/// This wraps mlua's Lua instance configured for Luau dialect with
/// sandbox-friendly defaults.
#[derive(Resource)]
pub struct ScriptRuntime {
    lua: Lua,
}

impl ScriptRuntime {
    /// Create a new script runtime with default sandbox settings.
    pub fn new() -> Result<Self, ScriptError> {
        // Create Lua with safe subset of standard library
        // Luau mode is enabled via the mlua feature flag
        let lua = Lua::new();

        // Configure for sandboxed execution
        lua.sandbox(true)?;

        // Load safe standard libraries
        lua.globals().set("_G", lua.globals().clone())?;

        Ok(Self { lua })
    }

    /// Create a runtime with full standard library (for testing/development).
    pub fn new_unsafe() -> Result<Self, ScriptError> {
        let lua = unsafe { Lua::unsafe_new_with(StdLib::ALL, mlua::LuaOptions::default()) };
        Ok(Self { lua })
    }

    /// Execute a Lua script string.
    pub fn exec(&self, script: &str) -> Result<(), ScriptError> {
        self.lua.load(script).exec()?;
        Ok(())
    }

    /// Execute a script and return the result.
    pub fn eval<T: mlua::FromLua>(&self, script: &str) -> Result<T, ScriptError> {
        let result = self.lua.load(script).eval()?;
        Ok(result)
    }

    /// Load a script file and execute it, returning the module table.
    pub fn load_module(&self, path: impl AsRef<Path>) -> Result<mlua::Table, ScriptError> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(ScriptError::NotFound(path.display().to_string()));
        }

        let script = std::fs::read_to_string(path)?;
        let chunk = self.lua.load(&script).set_name(path.to_string_lossy());

        let result: Value = chunk.eval()?;

        match result {
            Value::Table(table) => Ok(table),
            _ => Err(ScriptError::InvalidModule(
                "Script must return a table".to_string(),
            )),
        }
    }

    /// Load a script from a string, returning the module table.
    pub fn load_module_str(&self, name: &str, script: &str) -> Result<mlua::Table, ScriptError> {
        let chunk = self.lua.load(script).set_name(name);
        let result: Value = chunk.eval()?;

        match result {
            Value::Table(table) => Ok(table),
            _ => Err(ScriptError::InvalidModule(
                "Script must return a table".to_string(),
            )),
        }
    }

    /// Set a global value.
    pub fn set_global<V: mlua::IntoLua>(&self, name: &str, value: V) -> Result<(), ScriptError> {
        self.lua.globals().set(name, value)?;
        Ok(())
    }

    /// Get a global value.
    pub fn get_global<V: mlua::FromLua>(&self, name: &str) -> Result<V, ScriptError> {
        let value = self.lua.globals().get(name)?;
        Ok(value)
    }

    /// Create a new Lua table.
    pub fn create_table(&self) -> Result<mlua::Table, ScriptError> {
        Ok(self.lua.create_table()?)
    }

    /// Create a Lua function from a Rust closure.
    pub fn create_function<F, A, R>(&self, func: F) -> Result<mlua::Function, ScriptError>
    where
        F: Fn(&Lua, A) -> LuaResult<R> + Send + 'static,
        A: mlua::FromLuaMulti,
        R: mlua::IntoLuaMulti,
    {
        Ok(self.lua.create_function(func)?)
    }

    /// Get direct access to the Lua instance (for advanced use).
    pub fn lua(&self) -> &Lua {
        &self.lua
    }

    /// Call a function in a module table.
    pub fn call_module_function<A, R>(
        &self,
        module: &mlua::Table,
        func_name: &str,
        args: A,
    ) -> Result<R, ScriptError>
    where
        A: mlua::IntoLuaMulti,
        R: mlua::FromLuaMulti,
    {
        let func: mlua::Function = module.get(func_name)?;
        let result = func.call(args)?;
        Ok(result)
    }

    /// Check if a module has a specific handler function.
    pub fn module_has_handler(&self, module: &mlua::Table, handler_name: &str) -> bool {
        module.get::<mlua::Function>(handler_name).is_ok()
    }
}

impl Default for ScriptRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create ScriptRuntime")
    }
}

/// Plugin that sets up the scripting runtime.
pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        match ScriptRuntime::new() {
            Ok(runtime) => {
                app.insert_resource(runtime);
                info!("Scripting runtime initialized");
            }
            Err(e) => {
                error!("Failed to initialize scripting runtime: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_runtime() {
        let runtime = ScriptRuntime::new();
        assert!(runtime.is_ok());
    }

    #[test]
    fn execute_simple_script() {
        let runtime = ScriptRuntime::new().unwrap();
        let result = runtime.exec("local x = 1 + 1");
        assert!(result.is_ok());
    }

    #[test]
    fn eval_expression() {
        let runtime = ScriptRuntime::new().unwrap();
        let result: i32 = runtime.eval("return 2 + 2").unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn load_module_from_string() {
        let runtime = ScriptRuntime::new().unwrap();
        let module = runtime
            .load_module_str(
                "test",
                r#"
                local module = {}
                module.value = 42
                function module.get_value()
                    return module.value
                end
                return module
            "#,
            )
            .unwrap();

        let value: i32 = module.get("value").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn call_module_function() {
        let runtime = ScriptRuntime::new().unwrap();
        let module = runtime
            .load_module_str(
                "test",
                r#"
                local m = {}
                function m.add(a, b)
                    return a + b
                end
                return m
            "#,
            )
            .unwrap();

        let result: i32 = runtime.call_module_function(&module, "add", (3, 4)).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn module_handler_check() {
        let runtime = ScriptRuntime::new().unwrap();
        let module = runtime
            .load_module_str(
                "test",
                r#"
                local m = {}
                function m.handle_event() end
                return m
            "#,
            )
            .unwrap();

        assert!(runtime.module_has_handler(&module, "handle_event"));
        assert!(!runtime.module_has_handler(&module, "handle_other"));
    }

    #[test]
    fn global_values() {
        let runtime = ScriptRuntime::new().unwrap();

        runtime.set_global("test_value", 123).unwrap();
        let value: i32 = runtime.get_global("test_value").unwrap();
        assert_eq!(value, 123);

        // Use in script
        let result: i32 = runtime.eval("return test_value * 2").unwrap();
        assert_eq!(result, 246);
    }

    #[test]
    fn create_rust_function() {
        let runtime = ScriptRuntime::new().unwrap();

        let add_func = runtime
            .create_function(|_, (a, b): (i32, i32)| Ok(a + b))
            .unwrap();

        runtime.set_global("rust_add", add_func).unwrap();

        let result: i32 = runtime.eval("return rust_add(10, 20)").unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    fn invalid_module_returns_error() {
        let runtime = ScriptRuntime::new().unwrap();
        let result = runtime.load_module_str("test", "return 42"); // Not a table
        assert!(matches!(result, Err(ScriptError::InvalidModule(_))));
    }

    #[test]
    fn plugin_initializes() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(ScriptingPlugin);

        app.update();

        assert!(app.world().get_resource::<ScriptRuntime>().is_some());
    }
}
