//! UserData implementations for Lua interop.
//!
//! Implements mlua::UserData for core game types to enable
//! Lua scripts to interact with Rust data.

use mlua::{AnyUserData, IntoLua, Lua, Result as LuaResult, UserData, UserDataMethods, Value};
use roguebench_core::items::{Effect, ItemDefinition, ItemId, ItemType};
use roguebench_core::state_machine::{StateDefinition, StateId, StateTransition};

/// Wrapper for ItemId to implement UserData.
#[derive(Debug, Clone)]
pub struct LuaItemId(pub ItemId);

impl UserData for LuaItemId {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("id", |_, this, ()| Ok(this.0 .0.clone()));

        methods.add_meta_method("__tostring", |_, this, ()| Ok(this.0 .0.clone()));

        methods.add_meta_method("__eq", |_, this, other: AnyUserData| {
            if let Ok(other_id) = other.borrow::<LuaItemId>() {
                Ok(this.0 == other_id.0)
            } else {
                Ok(false)
            }
        });
    }
}

impl From<ItemId> for LuaItemId {
    fn from(id: ItemId) -> Self {
        Self(id)
    }
}

impl From<LuaItemId> for ItemId {
    fn from(lua_id: LuaItemId) -> Self {
        lua_id.0
    }
}

/// Wrapper for ItemType to implement UserData.
#[derive(Debug, Clone, Copy)]
pub struct LuaItemType(pub ItemType);

impl UserData for LuaItemType {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| {
            Ok(match this.0 {
                ItemType::Consumable => "consumable",
                ItemType::Equipment => "equipment",
                ItemType::Key => "key",
                ItemType::Currency => "currency",
                ItemType::Misc => "misc",
            })
        });

        methods.add_method("is_consumable", |_, this, ()| {
            Ok(matches!(this.0, ItemType::Consumable))
        });

        methods.add_method("is_equipment", |_, this, ()| {
            Ok(matches!(this.0, ItemType::Equipment))
        });

        methods.add_method("is_key", |_, this, ()| Ok(matches!(this.0, ItemType::Key)));

        methods.add_method("is_currency", |_, this, ()| {
            Ok(matches!(this.0, ItemType::Currency))
        });

        methods.add_meta_method("__tostring", |_, this, ()| {
            Ok(match this.0 {
                ItemType::Consumable => "consumable",
                ItemType::Equipment => "equipment",
                ItemType::Key => "key",
                ItemType::Currency => "currency",
                ItemType::Misc => "misc",
            })
        });

        methods.add_meta_method("__eq", |_, this, other: AnyUserData| {
            if let Ok(other_type) = other.borrow::<LuaItemType>() {
                Ok(this.0 == other_type.0)
            } else {
                Ok(false)
            }
        });
    }
}

impl From<ItemType> for LuaItemType {
    fn from(t: ItemType) -> Self {
        Self(t)
    }
}

impl From<LuaItemType> for ItemType {
    fn from(lua_type: LuaItemType) -> Self {
        lua_type.0
    }
}

/// Wrapper for Effect to implement UserData.
#[derive(Debug, Clone)]
pub struct LuaEffect(pub Effect);

impl UserData for LuaEffect {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("kind", |_, this, ()| {
            Ok(match &this.0 {
                Effect::ModifyStat { .. } => "modify_stat",
                Effect::ApplyStatus { .. } => "apply_status",
                Effect::TriggerEvent { .. } => "trigger_event",
            })
        });

        methods.add_method("stat", |_, this, ()| match &this.0 {
            Effect::ModifyStat { stat, .. } => Ok(Some(stat.clone())),
            _ => Ok(None),
        });

        methods.add_method("amount", |_, this, ()| match &this.0 {
            Effect::ModifyStat { amount, .. } => Ok(Some(*amount)),
            _ => Ok(None),
        });

        methods.add_method("status", |_, this, ()| match &this.0 {
            Effect::ApplyStatus { status, .. } => Ok(Some(status.clone())),
            _ => Ok(None),
        });

        methods.add_method("duration", |_, this, ()| match &this.0 {
            Effect::ApplyStatus { duration_secs, .. } => Ok(Some(*duration_secs)),
            _ => Ok(None),
        });

        methods.add_method("event", |_, this, ()| match &this.0 {
            Effect::TriggerEvent { event } => Ok(Some(event.clone())),
            _ => Ok(None),
        });

        methods.add_meta_method("__tostring", |_, this, ()| {
            Ok(match &this.0 {
                Effect::ModifyStat { stat, amount } => format!("ModifyStat({}, {})", stat, amount),
                Effect::ApplyStatus {
                    status,
                    duration_secs,
                } => {
                    format!("ApplyStatus({}, {}s)", status, duration_secs)
                }
                Effect::TriggerEvent { event } => format!("TriggerEvent({})", event),
            })
        });
    }
}

impl From<Effect> for LuaEffect {
    fn from(e: Effect) -> Self {
        Self(e)
    }
}

impl From<LuaEffect> for Effect {
    fn from(lua_effect: LuaEffect) -> Self {
        lua_effect.0
    }
}

/// Wrapper for ItemDefinition to implement UserData.
#[derive(Debug, Clone)]
pub struct LuaItemDefinition(pub ItemDefinition);

impl UserData for LuaItemDefinition {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("id", |_, this, ()| Ok(LuaItemId(this.0.id.clone())));

        methods.add_method("name", |_, this, ()| Ok(this.0.name.clone()));

        methods.add_method("item_type", |_, this, ()| Ok(LuaItemType(this.0.item_type)));

        methods.add_method("description", |_, this, ()| Ok(this.0.description.clone()));

        methods.add_method("is_stackable", |_, this, ()| Ok(this.0.stackable));

        methods.add_method("max_stack", |_, this, ()| Ok(this.0.max_stack));

        methods.add_method("value", |_, this, ()| Ok(this.0.value));

        methods.add_method("effects", |lua, this, ()| {
            let effects: Vec<LuaEffect> =
                this.0.effects.iter().cloned().map(LuaEffect).collect();
            lua.create_sequence_from(effects)
        });

        methods.add_method("effect_count", |_, this, ()| Ok(this.0.effects.len()));

        methods.add_meta_method("__tostring", |_, this, ()| {
            Ok(format!("Item({}: {})", this.0.id, this.0.name))
        });

        methods.add_meta_method("__eq", |_, this, other: AnyUserData| {
            if let Ok(other_def) = other.borrow::<LuaItemDefinition>() {
                Ok(this.0.id == other_def.0.id)
            } else {
                Ok(false)
            }
        });
    }
}

impl From<ItemDefinition> for LuaItemDefinition {
    fn from(def: ItemDefinition) -> Self {
        Self(def)
    }
}

impl From<LuaItemDefinition> for ItemDefinition {
    fn from(lua_def: LuaItemDefinition) -> Self {
        lua_def.0
    }
}

/// Helper to convert Rust types to Lua values.
pub trait ToLuaExt {
    fn to_lua_value(self, lua: &Lua) -> LuaResult<Value>;
}

impl ToLuaExt for ItemId {
    fn to_lua_value(self, lua: &Lua) -> LuaResult<Value> {
        LuaItemId(self).into_lua(lua)
    }
}

impl ToLuaExt for ItemType {
    fn to_lua_value(self, lua: &Lua) -> LuaResult<Value> {
        LuaItemType(self).into_lua(lua)
    }
}

impl ToLuaExt for Effect {
    fn to_lua_value(self, lua: &Lua) -> LuaResult<Value> {
        LuaEffect(self).into_lua(lua)
    }
}

impl ToLuaExt for ItemDefinition {
    fn to_lua_value(self, lua: &Lua) -> LuaResult<Value> {
        LuaItemDefinition(self).into_lua(lua)
    }
}

/// Helper to extract Rust types from Lua AnyUserData.
pub trait FromLuaExt: Sized {
    fn from_userdata(ud: &AnyUserData) -> LuaResult<Self>;
}

impl FromLuaExt for ItemId {
    fn from_userdata(ud: &AnyUserData) -> LuaResult<Self> {
        let lua_id = ud.borrow::<LuaItemId>()?;
        Ok(lua_id.0.clone())
    }
}

impl FromLuaExt for ItemType {
    fn from_userdata(ud: &AnyUserData) -> LuaResult<Self> {
        let lua_type = ud.borrow::<LuaItemType>()?;
        Ok(lua_type.0)
    }
}

impl FromLuaExt for Effect {
    fn from_userdata(ud: &AnyUserData) -> LuaResult<Self> {
        let lua_effect = ud.borrow::<LuaEffect>()?;
        Ok(lua_effect.0.clone())
    }
}

impl FromLuaExt for ItemDefinition {
    fn from_userdata(ud: &AnyUserData) -> LuaResult<Self> {
        let lua_def = ud.borrow::<LuaItemDefinition>()?;
        Ok(lua_def.0.clone())
    }
}

// --- State Machine UserData ---

/// Wrapper for StateId to implement UserData.
#[derive(Debug, Clone)]
pub struct LuaStateId(pub StateId);

impl UserData for LuaStateId {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("id", |_, this, ()| Ok(this.0 .0.clone()));

        methods.add_meta_method("__tostring", |_, this, ()| Ok(this.0 .0.clone()));

        methods.add_meta_method("__eq", |_, this, other: AnyUserData| {
            if let Ok(other_id) = other.borrow::<LuaStateId>() {
                Ok(this.0 == other_id.0)
            } else {
                Ok(false)
            }
        });
    }
}

impl From<StateId> for LuaStateId {
    fn from(id: StateId) -> Self {
        Self(id)
    }
}

impl From<LuaStateId> for StateId {
    fn from(lua_id: LuaStateId) -> Self {
        lua_id.0
    }
}

/// Wrapper for StateTransition to implement UserData.
#[derive(Debug, Clone)]
pub struct LuaStateTransition(pub StateTransition);

impl UserData for LuaStateTransition {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("from", |_, this, ()| Ok(LuaStateId(this.0.from.clone())));

        methods.add_method("to", |_, this, ()| Ok(LuaStateId(this.0.to.clone())));

        methods.add_method("from_id", |_, this, ()| Ok(this.0.from.0.clone()));

        methods.add_method("to_id", |_, this, ()| Ok(this.0.to.0.clone()));

        methods.add_method("time_in_previous", |_, this, ()| {
            Ok(this.0.time_in_previous)
        });

        methods.add_meta_method("__tostring", |_, this, ()| {
            Ok(format!("Transition({} -> {})", this.0.from, this.0.to))
        });
    }
}

impl From<StateTransition> for LuaStateTransition {
    fn from(transition: StateTransition) -> Self {
        Self(transition)
    }
}

impl From<LuaStateTransition> for StateTransition {
    fn from(lua_transition: LuaStateTransition) -> Self {
        lua_transition.0
    }
}

/// Wrapper for StateDefinition to implement UserData.
#[derive(Debug, Clone)]
pub struct LuaStateDefinition(pub StateDefinition);

impl UserData for LuaStateDefinition {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("id", |_, this, ()| Ok(LuaStateId(this.0.id.clone())));

        methods.add_method("id_str", |_, this, ()| Ok(this.0.id.0.clone()));

        methods.add_method("name", |_, this, ()| Ok(this.0.name.clone()));

        methods.add_method("description", |_, this, ()| {
            Ok(this.0.description.clone())
        });

        methods.add_method("metadata", |lua, this, key: String| {
            if let Some(value) = this.0.metadata.get(&key) {
                // Convert serde_json::Value to Lua value
                json_to_lua(lua, value)
            } else {
                Ok(Value::Nil)
            }
        });

        methods.add_method("has_metadata", |_, this, key: String| {
            Ok(this.0.metadata.contains_key(&key))
        });

        methods.add_meta_method("__tostring", |_, this, ()| {
            Ok(format!("State({})", this.0.id))
        });
    }
}

/// Convert a serde_json::Value to a Lua Value.
fn json_to_lua(lua: &Lua, value: &serde_json::Value) -> LuaResult<Value> {
    match value {
        serde_json::Value::Null => Ok(Value::Nil),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                // mlua Integer is i32, so try to fit it
                if let Ok(i32_val) = i32::try_from(i) {
                    Ok(Value::Integer(i32_val))
                } else {
                    // Too large for i32, use f64
                    Ok(Value::Number(i as f64))
                }
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Ok(Value::Nil)
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(lua.create_string(s)?)),
        serde_json::Value::Array(arr) => {
            let table = lua.create_table()?;
            for (i, v) in arr.iter().enumerate() {
                table.set(i + 1, json_to_lua(lua, v)?)?;
            }
            Ok(Value::Table(table))
        }
        serde_json::Value::Object(obj) => {
            let table = lua.create_table()?;
            for (k, v) in obj {
                table.set(k.clone(), json_to_lua(lua, v)?)?;
            }
            Ok(Value::Table(table))
        }
    }
}

impl From<StateDefinition> for LuaStateDefinition {
    fn from(def: StateDefinition) -> Self {
        Self(def)
    }
}

impl From<LuaStateDefinition> for StateDefinition {
    fn from(lua_def: LuaStateDefinition) -> Self {
        lua_def.0
    }
}

impl ToLuaExt for StateId {
    fn to_lua_value(self, lua: &Lua) -> LuaResult<Value> {
        LuaStateId(self).into_lua(lua)
    }
}

impl ToLuaExt for StateTransition {
    fn to_lua_value(self, lua: &Lua) -> LuaResult<Value> {
        LuaStateTransition(self).into_lua(lua)
    }
}

impl ToLuaExt for StateDefinition {
    fn to_lua_value(self, lua: &Lua) -> LuaResult<Value> {
        LuaStateDefinition(self).into_lua(lua)
    }
}

impl FromLuaExt for StateId {
    fn from_userdata(ud: &AnyUserData) -> LuaResult<Self> {
        let lua_id = ud.borrow::<LuaStateId>()?;
        Ok(lua_id.0.clone())
    }
}

impl FromLuaExt for StateTransition {
    fn from_userdata(ud: &AnyUserData) -> LuaResult<Self> {
        let lua_transition = ud.borrow::<LuaStateTransition>()?;
        Ok(lua_transition.0.clone())
    }
}

impl FromLuaExt for StateDefinition {
    fn from_userdata(ud: &AnyUserData) -> LuaResult<Self> {
        let lua_def = ud.borrow::<LuaStateDefinition>()?;
        Ok(lua_def.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scripting::ScriptRuntime;

    #[test]
    fn item_id_userdata() {
        let runtime = ScriptRuntime::new().unwrap();
        let id = LuaItemId(ItemId::new("test_item"));

        runtime.set_global("item_id", id).unwrap();

        let result: String = runtime.eval("return item_id:id()").unwrap();
        assert_eq!(result, "test_item");

        let str_result: String = runtime.eval("return tostring(item_id)").unwrap();
        assert_eq!(str_result, "test_item");
    }

    #[test]
    fn item_type_userdata() {
        let runtime = ScriptRuntime::new().unwrap();
        runtime
            .set_global("item_type", LuaItemType(ItemType::Consumable))
            .unwrap();

        let name: String = runtime.eval("return item_type:name()").unwrap();
        assert_eq!(name, "consumable");

        let is_consumable: bool = runtime.eval("return item_type:is_consumable()").unwrap();
        assert!(is_consumable);

        let is_equipment: bool = runtime.eval("return item_type:is_equipment()").unwrap();
        assert!(!is_equipment);
    }

    #[test]
    fn effect_userdata() {
        let runtime = ScriptRuntime::new().unwrap();
        let effect = LuaEffect(Effect::ModifyStat {
            stat: "health".into(),
            amount: 50,
        });

        runtime.set_global("effect", effect).unwrap();

        let kind: String = runtime.eval("return effect:kind()").unwrap();
        assert_eq!(kind, "modify_stat");

        let stat: String = runtime.eval("return effect:stat()").unwrap();
        assert_eq!(stat, "health");

        let amount: i32 = runtime.eval("return effect:amount()").unwrap();
        assert_eq!(amount, 50);
    }

    #[test]
    fn item_definition_userdata() {
        let runtime = ScriptRuntime::new().unwrap();
        let item = ItemDefinition::new("health_potion", "Health Potion", ItemType::Consumable)
            .with_description("Restores health")
            .with_effect(Effect::ModifyStat {
                stat: "health".into(),
                amount: 50,
            })
            .stackable(99)
            .with_value(25);

        runtime
            .set_global("item", LuaItemDefinition(item))
            .unwrap();

        let name: String = runtime.eval("return item:name()").unwrap();
        assert_eq!(name, "Health Potion");

        let stackable: bool = runtime.eval("return item:is_stackable()").unwrap();
        assert!(stackable);

        let max_stack: u32 = runtime.eval("return item:max_stack()").unwrap();
        assert_eq!(max_stack, 99);

        let value: u32 = runtime.eval("return item:value()").unwrap();
        assert_eq!(value, 25);

        let effect_count: usize = runtime.eval("return item:effect_count()").unwrap();
        assert_eq!(effect_count, 1);

        // Access effect through effects array
        let effect_kind: String = runtime.eval("return item:effects()[1]:kind()").unwrap();
        assert_eq!(effect_kind, "modify_stat");
    }

    #[test]
    fn item_type_comparison() {
        let runtime = ScriptRuntime::new().unwrap();
        runtime
            .set_global("type_a", LuaItemType(ItemType::Consumable))
            .unwrap();
        runtime
            .set_global("type_b", LuaItemType(ItemType::Consumable))
            .unwrap();
        runtime
            .set_global("type_c", LuaItemType(ItemType::Equipment))
            .unwrap();

        let same: bool = runtime.eval("return type_a == type_b").unwrap();
        assert!(same);

        let different: bool = runtime.eval("return type_a == type_c").unwrap();
        assert!(!different);
    }

    #[test]
    fn item_definition_in_script() {
        let runtime = ScriptRuntime::new().unwrap();
        let potion = ItemDefinition::new("healing_potion", "Healing Potion", ItemType::Consumable)
            .with_effect(Effect::ModifyStat {
                stat: "health".into(),
                amount: 100,
            });

        runtime
            .set_global("potion", LuaItemDefinition(potion))
            .unwrap();

        // Script that processes the item
        let module = runtime
            .load_module_str(
                "test",
                r#"
                local m = {}

                function m.calculate_heal_value(item)
                    local base = 0
                    for _, effect in ipairs(item:effects()) do
                        if effect:kind() == "modify_stat" and effect:stat() == "health" then
                            base = base + effect:amount()
                        end
                    end
                    return base
                end

                return m
            "#,
            )
            .unwrap();

        let heal_value: i32 = runtime
            .call_module_function(
                &module,
                "calculate_heal_value",
                runtime.get_global::<mlua::Value>("potion").unwrap(),
            )
            .unwrap();
        assert_eq!(heal_value, 100);
    }

    // --- State Machine UserData Tests ---

    #[test]
    fn state_id_userdata() {
        let runtime = ScriptRuntime::new().unwrap();
        let state_id = LuaStateId(StateId::new("idle"));

        runtime.set_global("state_id", state_id).unwrap();

        let result: String = runtime.eval("return state_id:id()").unwrap();
        assert_eq!(result, "idle");

        let str_result: String = runtime.eval("return tostring(state_id)").unwrap();
        assert_eq!(str_result, "idle");
    }

    #[test]
    fn state_id_comparison() {
        let runtime = ScriptRuntime::new().unwrap();
        runtime
            .set_global("state_a", LuaStateId(StateId::new("idle")))
            .unwrap();
        runtime
            .set_global("state_b", LuaStateId(StateId::new("idle")))
            .unwrap();
        runtime
            .set_global("state_c", LuaStateId(StateId::new("active")))
            .unwrap();

        let same: bool = runtime.eval("return state_a == state_b").unwrap();
        assert!(same);

        let different: bool = runtime.eval("return state_a == state_c").unwrap();
        assert!(!different);
    }

    #[test]
    fn state_transition_userdata() {
        let runtime = ScriptRuntime::new().unwrap();
        let transition = LuaStateTransition(StateTransition {
            from: StateId::new("idle"),
            to: StateId::new("active"),
            time_in_previous: 2.5,
        });

        runtime.set_global("transition", transition).unwrap();

        let from: String = runtime.eval("return transition:from_id()").unwrap();
        assert_eq!(from, "idle");

        let to: String = runtime.eval("return transition:to_id()").unwrap();
        assert_eq!(to, "active");

        let time: f32 = runtime.eval("return transition:time_in_previous()").unwrap();
        assert!((time - 2.5).abs() < 0.01);

        let str_result: String = runtime.eval("return tostring(transition)").unwrap();
        assert_eq!(str_result, "Transition(idle -> active)");
    }

    #[test]
    fn state_definition_userdata() {
        let runtime = ScriptRuntime::new().unwrap();
        let state = StateDefinition::new("idle")
            .with_name("Idle State")
            .with_description("Character is standing still")
            .with_metadata("animation", serde_json::json!("idle_anim"))
            .with_metadata("speed", serde_json::json!(0.0));

        runtime
            .set_global("state", LuaStateDefinition(state))
            .unwrap();

        let id: String = runtime.eval("return state:id_str()").unwrap();
        assert_eq!(id, "idle");

        let name: String = runtime.eval("return state:name()").unwrap();
        assert_eq!(name, "Idle State");

        let desc: String = runtime.eval("return state:description()").unwrap();
        assert_eq!(desc, "Character is standing still");

        let has_anim: bool = runtime.eval("return state:has_metadata('animation')").unwrap();
        assert!(has_anim);

        let has_missing: bool = runtime.eval("return state:has_metadata('missing')").unwrap();
        assert!(!has_missing);

        let anim: String = runtime.eval("return state:metadata('animation')").unwrap();
        assert_eq!(anim, "idle_anim");

        let speed: f64 = runtime.eval("return state:metadata('speed')").unwrap();
        assert!((speed - 0.0).abs() < 0.01);
    }

    #[test]
    fn state_definition_complex_metadata() {
        let runtime = ScriptRuntime::new().unwrap();
        let state = StateDefinition::new("attack")
            .with_metadata("damage_types", serde_json::json!(["physical", "fire"]))
            .with_metadata("stats", serde_json::json!({
                "damage": 50,
                "cooldown": 1.5
            }));

        runtime
            .set_global("state", LuaStateDefinition(state))
            .unwrap();

        // Access array
        let first_damage: String = runtime
            .eval("return state:metadata('damage_types')[1]")
            .unwrap();
        assert_eq!(first_damage, "physical");

        // Access nested object
        let damage: i64 = runtime.eval("return state:metadata('stats').damage").unwrap();
        assert_eq!(damage, 50);

        let cooldown: f64 = runtime
            .eval("return state:metadata('stats').cooldown")
            .unwrap();
        assert!((cooldown - 1.5).abs() < 0.01);
    }
}
