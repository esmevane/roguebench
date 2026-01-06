//! Lua scripting runtime for behavior authoring.
//!
//! Uses mlua with Luau dialect for sandbox-friendly scripting.
//!
//! # Architecture
//!
//! Scripts are modules that export handler functions:
//!
//! ```lua
//! local module = {}
//!
//! function module.handle_item_used(event)
//!     -- React to item usage
//! end
//!
//! return module
//! ```
//!
//! The runtime loads these modules and dispatches game events to their handlers.

mod hooks;
mod loader;
mod runtime;
mod userdata;

pub use hooks::{
    dispatch_item_used_hooks, dispatch_state_changed_hooks, HookDispatchResult, HookDispatcher,
    ScriptHooksPlugin,
};
pub use loader::{LoadedModule, ScriptLoader, ScriptLoaderPlugin};
pub use runtime::{ScriptError, ScriptRuntime, ScriptingPlugin};
pub use userdata::{
    FromLuaExt, LuaEffect, LuaItemDefinition, LuaItemId, LuaItemType, LuaStateDefinition,
    LuaStateId, LuaStateTransition, ToLuaExt,
};
