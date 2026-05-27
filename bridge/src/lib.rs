pub mod api;
pub mod device_api;
pub mod serde;
pub mod taffy_conv;

use std::sync::{Arc, Mutex};
use mlua::prelude::*;
use crate::device_api::DeviceBridge;
use crate::serde::UiNode;

pub fn create_vm() -> LuaResult<Lua> {
    let lua = Lua::new();

    // Set Lua package path so `require("lumina")` finds app/lumina/init.lua
    let globals = lua.globals();
    let package: LuaTable = globals.get("package")?;
    let current_path: String = package.get("path")?;
    let cwd = std::env::current_dir().unwrap_or_default().display().to_string();
    let lumina_path = format!("{}/app/lumina/?.lua;{}/app/?/init.lua;{}/app/?.lua;{}",
        cwd, cwd, cwd, current_path);
    package.set("path", lumina_path)?;

    let device_bridge = Arc::new(Mutex::new(DeviceBridge::new()));

    // Initialize callback registry for event system
    {
        let globals = lua.globals();
        let callbacks = lua.create_table()?;
        globals.set("_LUMINA_CALLBACKS", callbacks)?;
    }

    api::register_functions(&lua)?;
    device_api::register_device_api(&lua, device_bridge)?;

    Ok(lua)
}

pub fn load_app(lua: &Lua, path: &str) -> LuaResult<UiNode> {
    let code = std::fs::read_to_string(path)
        .map_err(|e| mlua::Error::RuntimeError(format!("Failed to read {}: {}", path, e)))?;

    let app_fn: LuaFunction = lua.load(&code).eval()?;
    let ui_tree: LuaTable = app_fn.call(())?;
    let root = serde::table_to_ui_node(&ui_tree, lua)?;

    Ok(root)
}
