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

// --- Motor JS/TS (Fase 3 — transpile + hot-reload mínimo) ---
pub mod transpile;
pub mod watch;

// --- Motor JS/TS (Fase 4 — runtime retenido + Device API) ---
pub mod app_runtime;
pub mod js_device;

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

// Fase 3: lee un archivo de la stdlib para apps .ts, prefiriendo la versión .ts
// (transpilada en memoria) y usando la .js como fallback.
fn read_stdlib_js_for_ts(basename: &str) -> Result<String, Box<dyn std::error::Error>> {
    for ext in ["ts", "js"] {
        let name = format!("{basename}.{ext}");
        let candidates = [
            format!("app/axo/{name}"),
            format!("../app/axo/{name}"),
            format!("{}/../app/axo/{name}", env!("CARGO_MANIFEST_DIR")),
        ];
        for p in &candidates {
            if let Ok(code) = std::fs::read_to_string(p) {
                if ext == "ts" {
                    return transpile::transpile_ts_to_js(&code, &name);
                }
                return Ok(code);
            }
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("Fase 3: stdlib no encontrada: app/axo/{basename}.{{ts,js}}"),
    )
    .into())
}

// Fase 3: evalúa código JS de app con la stdlib ya preparada (inyectar, invocar si es función, convertir).
// Fase 4: usa el runtime retenido (los callbacks quedan disponibles para invoke_js_callback).
fn eval_js_with_stdlib(
    app_js: String,
    stdlib: Vec<String>,
) -> Result<crate::serde::UiNode, Box<dyn std::error::Error>> {
    let node: crate::serde::UiNode = app_runtime::with_ctx(
        |ctx| -> Result<crate::serde::UiNode, Box<dyn std::error::Error>> {
            for std_code in &stdlib {
                let _: rquickjs::Value = ctx.eval(std_code.clone())?;
            }
            let v: rquickjs::Value = ctx.eval(app_js.clone())?;
            let target: rquickjs::Value = if v.is_function() {
                let func = rquickjs::Function::from_value(v)?;
                func.call::<_, rquickjs::Value>(())?
            } else {
                v
            };
            js_to_ui_node(target).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e).into())
        },
    )??;
    Ok(node)
}

/// Carga una app `.js` o `.ts`.
/// - .js  → flujo actual (Fase 2)
/// - .ts  → transpile + flujo actual
pub fn load_app_auto(path: &str) -> Result<crate::serde::UiNode, Box<dyn std::error::Error>> {
    if path.ends_with(".ts") {
        // Fase 3: leer, transpilar y evaluar con stdlib .ts (fallback .js).
        let ts = std::fs::read_to_string(path)?;
        let app_js = transpile::transpile_ts_to_js(&ts, path)?;
        let stdlib = vec![
            read_stdlib_js_for_ts("state")?,
            read_stdlib_js_for_ts("components")?,
            read_stdlib_js_for_ts("index")?,
        ];
        eval_js_with_stdlib(app_js, stdlib)
    } else {
        // Fase 3: .js → flujo Fase 2 sin cambios.
        load_js_app(path)
    }
}

pub fn load_js_app(path: &str) -> Result<crate::serde::UiNode, Box<dyn std::error::Error>> {
    // Fase 1: 1. Leer el archivo.
    let code = std::fs::read_to_string(path)?;
    // Fase 2: stdlib .js (state.js, components.js, index.js).
    let stdlib = vec![
        read_js_stdlib("state.js")?,
        read_js_stdlib("components.js")?,
        read_js_stdlib("index.js")?,
    ];
    // Fase 4: evaluar en el runtime retenido (console + Device registrados ahí).
    eval_js_with_stdlib(code, stdlib)
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

// Fase 3: verificación mínima de TypeScript + regresión JS.
#[allow(dead_code)]
pub fn fase3_smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    // Fase 3: ts-hello debe devolver View con hijo Text que contiene "TypeScript".
    let ts_candidates = [
        "app/examples/ts-hello/app.ts",
        "../app/examples/ts-hello/app.ts",
        concat!(env!("CARGO_MANIFEST_DIR"), "/../app/examples/ts-hello/app.ts"),
    ];
    let mut loaded_ts = false;
    let mut last_err: Option<Box<dyn std::error::Error>> = None;
    for path in ts_candidates {
        match load_app_auto(path) {
            Ok(root) => {
                assert_eq!(root.node_type, "View");
                assert!(root.children.iter().any(|c| c.node_type == "Text"
                    && c.content.contains("TypeScript")));
                println!("[Fase 3] OK: {path} → View con Text TypeScript");
                loaded_ts = true;
                break;
            }
            Err(e) if std::fs::metadata(path).is_err() => {
                last_err = Some(e);
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    if !loaded_ts {
        return Err(last_err.unwrap_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Fase 3: ts-hello/app.ts no encontrado").into()
        }));
    }
    // Fase 3: regresión Fase 2 (js-counter sigue funcionando).
    let js_candidates = [
        "app/examples/js-counter/app.js",
        "../app/examples/js-counter/app.js",
        concat!(env!("CARGO_MANIFEST_DIR"), "/../app/examples/js-counter/app.js"),
    ];
    let mut last_js_err: Option<Box<dyn std::error::Error>> = None;
    for path in js_candidates {
        match load_app_auto(path) {
            Ok(root) => {
                assert_eq!(root.node_type, "View");
                println!("[Fase 3] OK regresión: {path} → {}", root.node_type);
                return Ok(());
            }
            Err(e) if std::fs::metadata(path).is_err() => {
                last_js_err = Some(e);
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err(last_js_err.unwrap_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Fase 3: js-counter/app.js no encontrado").into()
    }))
}

// Fase 4: verificación de Device API + onClick invocable + regresión.
#[allow(dead_code)]
pub fn fase4_smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    // Fase 4: ts-device-click produce View con Button y on_click_id no vacío.
    let ts_candidates = [
        "app/examples/ts-device-click/app.ts",
        "../app/examples/ts-device-click/app.ts",
        concat!(env!("CARGO_MANIFEST_DIR"), "/../app/examples/ts-device-click/app.ts"),
    ];
    let mut btn_id: Option<String> = None;
    let mut last_err: Option<Box<dyn std::error::Error>> = None;
    for path in ts_candidates {
        match load_app_auto(path) {
            Ok(root) => {
                assert_eq!(root.node_type, "View");
                let btn = root
                    .children
                    .iter()
                    .find(|c| c.node_type == "Button")
                    .expect("Fase 4: falta Button");
                assert!(!btn.style.on_click_id.is_empty());
                println!("[Fase 4] OK: {path} → View con Button on_click_id={}", btn.style.on_click_id);
                btn_id = Some(btn.style.on_click_id.clone());
                break;
            }
            Err(e) if std::fs::metadata(path).is_err() => {
                last_err = Some(e);
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    let btn_id = btn_id.ok_or_else(|| {
        last_err.unwrap_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Fase 4: ts-device-click/app.ts no encontrado").into()
        })
    })?;
    // Fase 4: Device.info() accesible en el runtime retenido.
    let os: String = app_runtime::with_ctx(|ctx| -> Result<String, Box<dyn std::error::Error>> {
        let s: String = ctx.eval("Device.info().os_name")?;
        Ok(s)
    })??;
    assert!(!os.is_empty());
    println!("[Fase 4] OK: Device.info().os_name={os}");
    // Fase 4: invoke_js_callback ejecuta la función registrada (imprime click + notificación).
    assert!(callbacks::invoke_js_callback(&btn_id)?);
    println!("[Fase 4] OK: invoke_js_callback({btn_id})=true");
    // Fase 4: id inexistente → Ok(false), sin pánico.
    assert!(!callbacks::invoke_js_callback("cb_inexistente_fase4")?);
    // Fase 4: regresión Fase 2 (js-counter sigue funcionando).
    let js_candidates = [
        "app/examples/js-counter/app.js",
        "../app/examples/js-counter/app.js",
        concat!(env!("CARGO_MANIFEST_DIR"), "/../app/examples/js-counter/app.js"),
    ];
    let mut last_js_err: Option<Box<dyn std::error::Error>> = None;
    for path in js_candidates {
        match load_app_auto(path) {
            Ok(root) => {
                assert_eq!(root.node_type, "View");
                println!("[Fase 4] OK regresión: {path} → {}", root.node_type);
                return Ok(());
            }
            Err(e) if std::fs::metadata(path).is_err() => {
                last_js_err = Some(e);
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err(last_js_err.unwrap_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Fase 4: js-counter/app.js no encontrado").into()
    }))
}

// Fase 5: batería mínima de humo post-limpieza (sin Lua).
#[allow(dead_code)]
pub fn fase5_smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    // Fase 5: los tres ejemplos TS cargan y devuelven View.
    for (name, want) in [
        ("app/examples/ts-hello/app.ts", "Hola desde TypeScript"),
        ("app/examples/ts-counter/app.ts", "Contador:"),
        ("app/examples/ts-device-click/app.ts", "OS:"),
    ] {
        let candidates = [
            name.to_string(),
            format!("../{name}"),
            format!("{}/../{name}", env!("CARGO_MANIFEST_DIR")),
        ];
        let mut loaded = false;
        for path in &candidates {
            if std::fs::metadata(path).is_err() {
                continue;
            }
            let root = load_app_auto(path)?;
            assert_eq!(root.node_type, "View", "Fase 5: {path} no es View");
            assert!(
                root.children.iter().any(|c| c.content.contains(want)),
                "Fase 5: {path} sin contenido '{want}'"
            );
            println!("[Fase 5] OK: {path} → View ('{want}')");
            loaded = true;
            break;
        }
        if !loaded {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Fase 5: ejemplo no encontrado: {name}"),
            )
            .into());
        }
    }
    // Fase 5: el botón de ts-device-click sigue siendo invocable.
    let root = load_app_auto("app/examples/ts-device-click/app.ts")
        .or_else(|_| load_app_auto("../app/examples/ts-device-click/app.ts"))?;
    let btn = root
        .children
        .iter()
        .find(|c| c.node_type == "Button")
        .expect("Fase 5: falta Button");
    assert!(callbacks::invoke_js_callback(&btn.style.on_click_id)?);
    println!("[Fase 5] OK: invoke on_click_id={}", btn.style.on_click_id);
    Ok(())
}
