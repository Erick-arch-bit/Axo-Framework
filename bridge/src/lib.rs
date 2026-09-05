pub mod api;
pub mod device_api;
pub mod serde;
pub mod taffy_conv;

// --- Nuevos módulos para el motor JS (Fase 0 — solo stubs) ---
pub mod runtime;
pub mod ui;
pub mod callbacks;
pub mod js_serde;

// --- API pública del motor JS (Fase 1 — bridge mínimo) ---
pub use runtime::create_js_runtime;
pub use js_serde::js_to_ui_node;

/// Crea una VM de QuickJS lista para usar (Fase 1).
pub fn create_js_vm() -> Result<rquickjs::Context, Box<dyn std::error::Error>> {
    // Fase 1: Runtime + contexto con console.log registrado.
    let rt = runtime::create_js_runtime()?;
    let ctx = rquickjs::Context::full(&rt)?;
    ctx.with(|ctx| runtime::register_console(ctx))?;
    Ok(ctx)
}

/// Carga un archivo .js, evalúa la función/objeto exportado y lo convierte a UiNode.
///
/// Contrato Fase 1:
/// - El archivo debe evaluar a una función que al llamarse devuelve el árbol UI,
///   o directamente a un objeto que representa el árbol.
/// - Ejemplo mínimo aceptado:
///   ```js
///   function App() {
///     return {
///       type: "View",
///       style: { width: "100%", height: "100%", backgroundColor: "#1a1a2e" },
///       children: [
///         { type: "Text", content: "Hola desde JS", style: { fontSize: 24, color: "#ffffff" } }
///       ]
///     };
///   }
///   App; // o export / return App
///   ```
// Fase 2: lee un archivo de la stdlib JS (state.js, components.js, index.js)
// resolviendo rutas relativas al working directory.
fn read_js_stdlib(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let candidates = [
        format!("app/axo/{name}"),
        format!("../app/axo/{name}"),
        format!("{}/../app/axo/{name}", env!("CARGO_MANIFEST_DIR")),
    ];
    for p in &candidates {
        if let Ok(code) = std::fs::read_to_string(p) {
            return Ok(code);
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("Fase 2: stdlib JS no encontrada: app/axo/{name}"),
    )
    .into())
}

pub fn load_js_app(path: &str) -> Result<crate::serde::UiNode, Box<dyn std::error::Error>> {
    // Fase 1: 1. Leer el archivo.
    let code = std::fs::read_to_string(path)?;
    // Fase 1: 2. Crear runtime + context.
    let ctx = create_js_vm()?;
    // Fase 1: 3-6. Evaluar, invocar si es función y convertir a UiNode (dentro del scope del contexto).
    let node: crate::serde::UiNode = ctx.with(|ctx| -> Result<crate::serde::UiNode, Box<dyn std::error::Error>> {
        // Fase 2: inyectar la stdlib (state.js, components.js, index.js) antes de la app.
        for name in ["state.js", "components.js", "index.js"] {
            let std_code = read_js_stdlib(name)?;
            let _: rquickjs::Value = ctx.eval(std_code)?;
        }
        // Fase 1: 3. Evaluar el código.
        let v: rquickjs::Value = ctx.eval(code.clone())?;
        // Fase 1: 4. Si el resultado es función → llamarla.
        let target: rquickjs::Value = if v.is_function() {
            let func = rquickjs::Function::from_value(v)?;
            func.call::<_, rquickjs::Value>(())?
        } else {
            v
        };
        // Fase 1: 5-6. Convertir el valor resultante con js_to_ui_node y devolver UiNode.
        js_to_ui_node(target).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e).into())
    })?;
    Ok(node)
}

// Fase 1: verificación mínima del flujo app.js → QuickJS → UiNode.
#[allow(dead_code)]
pub fn fase1_smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    // Fase 1: ruta relativa al workspace (con fallback según working directory).
    let candidates = [
        "app/examples/js-hello/app.js",
        "../app/examples/js-hello/app.js",
        concat!(env!("CARGO_MANIFEST_DIR"), "/../app/examples/js-hello/app.js"),
    ];
    let mut last_err: Option<Box<dyn std::error::Error>> = None;
    for path in candidates {
        match load_js_app(path) {
            Ok(root) => {
                assert_eq!(root.node_type, "View");
                assert!(!root.children.is_empty());
                assert_eq!(root.children[0].node_type, "Text");
                assert_eq!(root.children[0].content, "Hola desde QuickJS");
                println!("[Fase 1] OK: {path} → View con {} hijo(s)", root.children.len());
                return Ok(());
            }
            Err(e) if std::fs::metadata(path).is_err() => {
                last_err = Some(e);
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err(last_err.unwrap_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Fase 1: app.js no encontrado").into()))
}

// Fase 2: verificación mínima del flujo con stdlib (UI.View/Text/Button + useState + onClick).
#[allow(dead_code)]
pub fn fase2_smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    // Fase 2: ruta relativa al workspace (con fallback según working directory).
    let candidates = [
        "app/examples/js-counter/app.js",
        "../app/examples/js-counter/app.js",
        concat!(env!("CARGO_MANIFEST_DIR"), "/../app/examples/js-counter/app.js"),
    ];
    let mut last_err: Option<Box<dyn std::error::Error>> = None;
    for path in candidates {
        match load_js_app(path) {
            Ok(root) => {
                assert_eq!(root.node_type, "View");
                assert!(root.children.iter().any(|c| c.node_type == "Text"));
                let btn = root.children.iter().find(|c| c.node_type == "Button").expect("Fase 2: falta Button");
                assert!(!btn.style.on_click_id.is_empty());
                println!("[Fase 2] OK: {path} → View con {} hijo(s), on_click_id={}", root.children.len(), btn.style.on_click_id);
                return Ok(());
            }
            Err(e) if std::fs::metadata(path).is_err() => {
                last_err = Some(e);
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err(last_err.unwrap_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Fase 2: js-counter/app.js no encontrado").into()))
}

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
