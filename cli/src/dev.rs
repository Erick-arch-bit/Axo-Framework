use std::sync::{Arc, Mutex};

use clap::Args;
use mlua::prelude::*;

use axo_core::window::ClickHandler;

static LUA_VM: std::sync::LazyLock<Mutex<Option<Lua>>> =
    std::sync::LazyLock::new(|| Mutex::new(None));

#[derive(Args)]
pub struct DevArgs {
    #[arg(long, default_value = "app/app.lua")]
    pub entry: String,

    #[arg(long, default_value_t = 9876)]
    pub port: u16,

    #[arg(long, default_value_t = false)]
    pub no_watch: bool,
}

fn load_and_build(entry: &str, viewport_w: f32, viewport_h: f32) -> Option<Vec<axo_core::Rect>> {
    match axo_bridge::create_vm() {
        Ok(lua) => {
            match axo_bridge::load_app(&lua, entry) {
                Ok(root) => {
                    // Move Lua VM to global storage for click callbacks
                    *LUA_VM.lock().unwrap() = Some(lua);

                    let mut engine = axo_core::layout::Engine::new();
                    let rects = axo_bridge::taffy_conv::build_rects(
                        &mut engine, &root, viewport_w, viewport_h,
                    );
                    println!("[CLI] Loaded {} rectangles", rects.len());
                    Some(rects)
                }
                Err(e) => {
                    eprintln!("[Lua] Failed to load app: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            eprintln!("[Lua] Failed to create VM: {}", e);
            None
        }
    }
}

fn make_click_handler() -> ClickHandler {
    Arc::new(|_node_id: u64, cb_id: &str, _x: f64, _y: f64| {
        if cb_id.is_empty() {
            return;
        }
        let state = LUA_VM.lock().unwrap();
        if let Some(ref lua) = *state {
            if cb_id.starts_with("__axo_cb_") {
                if let Ok(callbacks) = lua.globals().get::<LuaTable>("_AXO_CALLBACKS") {
                    if let Ok(func) = callbacks.get::<LuaFunction>(cb_id) {
                        let _ = func.call::<()>(());
                    }
                }
            } else {
                if let Ok(func) = lua.globals().get::<LuaFunction>(cb_id) {
                    let _ = func.call::<()>(());
                }
            }
        }
    })
}

pub fn run(args: DevArgs) {
    println!("[CLI] Loading Lua app from {}", args.entry);

    let shared_rects = Arc::new(Mutex::new(Vec::new()));

    // Initial load
    if let Some(rects) = load_and_build(&args.entry, 1024.0, 768.0) {
        *shared_rects.lock().unwrap() = rects;
    }

    let click_handler = make_click_handler();

    // Start file watcher for hot reload
    if !args.no_watch {
        let entry = args.entry.clone();
        let shared = shared_rects.clone();
        let app_dir = std::path::Path::new(&entry)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());

        axo_core::hot_reload::watcher::watch(&app_dir, move || {
            if let Some(rects) = load_and_build(&entry, 1024.0, 768.0) {
                *shared.lock().unwrap() = rects;
                println!("[HotReload] UI updated!");
            }
        });
    }

    println!("[CLI] Launching Axo window...");
    axo_core::window::run_with_shared_rects_and_handler(shared_rects, Some(click_handler));
}
