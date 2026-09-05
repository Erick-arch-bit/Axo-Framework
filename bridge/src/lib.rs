pub mod api;
pub mod device_api;
pub mod serde;
pub mod taffy_conv;

// --- Nuevos módulos para el motor JS (Fase 0 — solo stubs) ---
pub mod runtime;
pub mod ui;
pub mod callbacks;
pub mod js_serde;

use std::sync::{Arc, Mutex};
use mlua::prelude::*;
use crate::device_api::DeviceBridge;
use crate::serde::UiNode;

pub fn create_vm() -> LuaResult<Lua> {
    let lua = Lua::new();

    // Set Lua package path so `require("axo")` finds app/axo/init.lua
    let globals = lua.globals();
    let package: LuaTable = globals.get("package")?;
    let current_path: String = package.get("path")?;
    let cwd = std::env::current_dir().unwrap_or_default().display().to_string();
    let axo_path = format!("{}/app/axo/?.lua;{}/app/components/?.lua;{}/app/?/init.lua;{}/app/?.lua;{}",
        cwd, cwd, cwd, cwd, current_path);
    package.set("path", axo_path)?;

    let device_bridge = Arc::new(Mutex::new(DeviceBridge::new()));

    // Initialize callback registry for event system
    {
        let globals = lua.globals();
        let callbacks = lua.create_table()?;
        globals.set("_AXO_CALLBACKS", callbacks)?;
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
