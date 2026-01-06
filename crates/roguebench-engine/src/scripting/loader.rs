//! Script module loader for managing Lua modules.
//!
//! Loads script files from the filesystem and tracks them by name.

use bevy::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::{ScriptError, ScriptRuntime};

/// A loaded script module with metadata.
#[derive(Debug)]
pub struct LoadedModule {
    /// The module name (derived from filename or explicitly set)
    pub name: String,
    /// Path to the source file (if loaded from file)
    pub path: Option<PathBuf>,
    /// The Lua table returned by the module
    table: mlua::RegistryKey,
}

impl LoadedModule {
    /// Get the module's Lua table.
    pub fn table(&self, lua: &mlua::Lua) -> Result<mlua::Table, ScriptError> {
        Ok(lua.registry_value(&self.table)?)
    }
}

/// Manages loading and tracking of script modules.
#[derive(Resource)]
pub struct ScriptLoader {
    /// Directory to load scripts from
    scripts_dir: PathBuf,
    /// Loaded modules by name
    modules: HashMap<String, LoadedModule>,
}

impl ScriptLoader {
    /// Create a new script loader with the given scripts directory.
    pub fn new(scripts_dir: impl Into<PathBuf>) -> Self {
        Self {
            scripts_dir: scripts_dir.into(),
            modules: HashMap::new(),
        }
    }

    /// Get the scripts directory.
    pub fn scripts_dir(&self) -> &Path {
        &self.scripts_dir
    }

    /// Load a module from a file path relative to the scripts directory.
    pub fn load_file(
        &mut self,
        runtime: &ScriptRuntime,
        relative_path: impl AsRef<Path>,
    ) -> Result<&LoadedModule, ScriptError> {
        let relative_path = relative_path.as_ref();
        let full_path = self.scripts_dir.join(relative_path);

        // Derive module name from path (without extension)
        let name = relative_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| ScriptError::InvalidModule("Invalid path".to_string()))?
            .to_string();

        self.load_file_as(runtime.lua(), &full_path, &name)
    }

    /// Load a module from a file with an explicit name.
    fn load_file_as(
        &mut self,
        lua: &mlua::Lua,
        path: &Path,
        name: &str,
    ) -> Result<&LoadedModule, ScriptError> {
        if !path.exists() {
            return Err(ScriptError::NotFound(path.display().to_string()));
        }

        let script = std::fs::read_to_string(path)?;
        let chunk = lua.load(&script).set_name(path.to_string_lossy());
        let result: mlua::Value = chunk.eval()?;

        match result {
            mlua::Value::Table(table) => {
                let key = lua.create_registry_value(table)?;
                let module = LoadedModule {
                    name: name.to_string(),
                    path: Some(path.to_path_buf()),
                    table: key,
                };
                self.modules.insert(name.to_string(), module);
                Ok(self.modules.get(name).unwrap())
            }
            _ => Err(ScriptError::InvalidModule(
                "Script must return a table".to_string(),
            )),
        }
    }

    /// Load a module from a string with an explicit name.
    pub fn load_string(
        &mut self,
        runtime: &ScriptRuntime,
        name: &str,
        script: &str,
    ) -> Result<&LoadedModule, ScriptError> {
        let lua = runtime.lua();
        let chunk = lua.load(script).set_name(name);
        let result: mlua::Value = chunk.eval()?;

        match result {
            mlua::Value::Table(table) => {
                let key = lua.create_registry_value(table)?;
                let module = LoadedModule {
                    name: name.to_string(),
                    path: None,
                    table: key,
                };
                self.modules.insert(name.to_string(), module);
                Ok(self.modules.get(name).unwrap())
            }
            _ => Err(ScriptError::InvalidModule(
                "Script must return a table".to_string(),
            )),
        }
    }

    /// Get a loaded module by name.
    pub fn get(&self, name: &str) -> Option<&LoadedModule> {
        self.modules.get(name)
    }

    /// Check if a module is loaded.
    pub fn is_loaded(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    /// Get all loaded module names.
    pub fn loaded_modules(&self) -> impl Iterator<Item = &str> {
        self.modules.keys().map(|s| s.as_str())
    }

    /// Unload a module by name.
    pub fn unload(&mut self, name: &str) -> bool {
        self.modules.remove(name).is_some()
    }

    /// Reload a module from its original path.
    pub fn reload(
        &mut self,
        runtime: &ScriptRuntime,
        name: &str,
    ) -> Result<&LoadedModule, ScriptError> {
        let path = self
            .modules
            .get(name)
            .and_then(|m| m.path.clone())
            .ok_or_else(|| ScriptError::NotFound(format!("Module not loaded: {}", name)))?;

        self.load_file_as(runtime.lua(), &path, name)
    }

    /// Load all Lua files from the scripts directory.
    pub fn load_all(&mut self, runtime: &ScriptRuntime) -> Vec<Result<String, ScriptError>> {
        let mut results = Vec::new();

        if !self.scripts_dir.exists() {
            return results;
        }

        let entries = match std::fs::read_dir(&self.scripts_dir) {
            Ok(entries) => entries,
            Err(e) => {
                results.push(Err(ScriptError::Io(e)));
                return results;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "lua" || ext == "luau") {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                match self.load_file_as(runtime.lua(), &path, &name) {
                    Ok(_) => results.push(Ok(name)),
                    Err(e) => results.push(Err(e)),
                }
            }
        }

        results
    }

    /// Get modules that have a specific handler function.
    pub fn modules_with_handler<'a>(
        &'a self,
        runtime: &'a ScriptRuntime,
        handler_name: &'a str,
    ) -> impl Iterator<Item = &'a LoadedModule> + 'a {
        self.modules.values().filter(move |module| {
            module
                .table(runtime.lua())
                .map(|t| runtime.module_has_handler(&t, handler_name))
                .unwrap_or(false)
        })
    }
}

impl Default for ScriptLoader {
    fn default() -> Self {
        Self::new("scripts")
    }
}

/// Plugin that sets up the script loader.
pub struct ScriptLoaderPlugin {
    pub scripts_dir: PathBuf,
}

impl ScriptLoaderPlugin {
    pub fn new(scripts_dir: impl Into<PathBuf>) -> Self {
        Self {
            scripts_dir: scripts_dir.into(),
        }
    }
}

impl Default for ScriptLoaderPlugin {
    fn default() -> Self {
        Self::new("scripts")
    }
}

impl Plugin for ScriptLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScriptLoader::new(self.scripts_dir.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn setup_test_dir() -> (TempDir, PathBuf) {
        let temp = TempDir::new().unwrap();
        let scripts = temp.path().join("scripts");
        std::fs::create_dir_all(&scripts).unwrap();
        (temp, scripts)
    }

    fn write_script(dir: &Path, name: &str, content: &str) {
        let path = dir.join(name);
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn load_module_from_string() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");

        loader
            .load_string(
                &runtime,
                "test",
                r#"
                local m = {}
                m.value = 42
                return m
            "#,
            )
            .unwrap();

        assert!(loader.is_loaded("test"));
        let module = loader.get("test").unwrap();
        let table = module.table(runtime.lua()).unwrap();
        let value: i32 = table.get("value").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn load_module_from_file() {
        let (_temp, scripts) = setup_test_dir();
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new(&scripts);

        write_script(
            &scripts,
            "items.lua",
            r#"
            local m = {}
            m.name = "items_module"
            function m.get_name()
                return m.name
            end
            return m
        "#,
        );

        loader.load_file(&runtime, "items.lua").unwrap();

        assert!(loader.is_loaded("items"));
        let module = loader.get("items").unwrap();
        assert_eq!(module.path.as_ref().unwrap(), &scripts.join("items.lua"));
    }

    #[test]
    fn load_all_scripts() {
        let (_temp, scripts) = setup_test_dir();
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new(&scripts);

        write_script(&scripts, "mod_a.lua", "return { name = 'a' }");
        write_script(&scripts, "mod_b.lua", "return { name = 'b' }");
        write_script(&scripts, "mod_c.luau", "return { name = 'c' }");
        write_script(&scripts, "not_lua.txt", "ignore me");

        let results = loader.load_all(&runtime);

        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
        assert!(loader.is_loaded("mod_a"));
        assert!(loader.is_loaded("mod_b"));
        assert!(loader.is_loaded("mod_c"));
    }

    #[test]
    fn reload_module() {
        let (_temp, scripts) = setup_test_dir();
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new(&scripts);

        write_script(&scripts, "reloadable.lua", "return { version = 1 }");
        loader.load_file(&runtime, "reloadable.lua").unwrap();

        {
            let module = loader.get("reloadable").unwrap();
            let table = module.table(runtime.lua()).unwrap();
            let version: i32 = table.get("version").unwrap();
            assert_eq!(version, 1);
        }

        // Modify the file
        write_script(&scripts, "reloadable.lua", "return { version = 2 }");
        loader.reload(&runtime, "reloadable").unwrap();

        let module = loader.get("reloadable").unwrap();
        let table = module.table(runtime.lua()).unwrap();
        let version: i32 = table.get("version").unwrap();
        assert_eq!(version, 2);
    }

    #[test]
    fn find_modules_with_handler() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");

        loader
            .load_string(
                &runtime,
                "with_handler",
                r#"
                local m = {}
                function m.on_item_used() end
                return m
            "#,
            )
            .unwrap();

        loader
            .load_string(
                &runtime,
                "without_handler",
                r#"
                local m = {}
                function m.other_handler() end
                return m
            "#,
            )
            .unwrap();

        let with_handler: Vec<_> = loader
            .modules_with_handler(&runtime, "on_item_used")
            .collect();
        assert_eq!(with_handler.len(), 1);
        assert_eq!(with_handler[0].name, "with_handler");
    }

    #[test]
    fn unload_module() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");

        loader
            .load_string(&runtime, "temp", "return {}")
            .unwrap();
        assert!(loader.is_loaded("temp"));

        assert!(loader.unload("temp"));
        assert!(!loader.is_loaded("temp"));
        assert!(!loader.unload("temp")); // Second unload returns false
    }

    #[test]
    fn loaded_modules_iterator() {
        let runtime = ScriptRuntime::new().unwrap();
        let mut loader = ScriptLoader::new("scripts");

        loader.load_string(&runtime, "a", "return {}").unwrap();
        loader.load_string(&runtime, "b", "return {}").unwrap();
        loader.load_string(&runtime, "c", "return {}").unwrap();

        let mut names: Vec<_> = loader.loaded_modules().collect();
        names.sort();
        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn plugin_inserts_resource() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(ScriptLoaderPlugin::new("test_scripts"));

        app.update();

        let loader = app.world().get_resource::<ScriptLoader>();
        assert!(loader.is_some());
        assert_eq!(loader.unwrap().scripts_dir(), Path::new("test_scripts"));
    }
}
