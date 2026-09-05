// Fase 4 — Invocación real de callbacks JS (onClick) desde Rust.
use rquickjs::{Function, Object, Value};

/// Invoca un callback JS previamente registrado por id.
/// Devuelve Ok(true) si existía y se llamó, Ok(false) si no existía.
pub fn invoke_js_callback(callback_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if callback_id.is_empty() {
        return Ok(false);
    }
    // Fase 4: se ejecuta en el runtime retenido donde se cargó la app.
    let called = crate::app_runtime::with_ctx(
        |ctx| -> Result<bool, Box<dyn std::error::Error>> {
            let globals = ctx.globals();
            let cbs: Option<Object> = globals.get("__AXO_CALLBACKS")?;
            let cbs = match cbs {
                Some(o) => o,
                None => return Ok(false),
            };
            let val: Option<Value> = cbs.get(callback_id)?;
            match val {
                None => Ok(false),
                Some(v) if v.is_undefined() || v.is_null() => Ok(false),
                Some(v) => {
                    let func = Function::from_value(v).map_err(|e| {
                        format!("Fase 4: '{callback_id}' no es una función JS: {e}")
                    })?;
                    // Fase 4: llamada sin argumentos.
                    let _: Value = func.call::<_, Value>(())?;
                    Ok(true)
                }
            }
        },
    )??;
    Ok(called)
}

/// Adaptador semántico para clicks: el core/CLI puede llamar con el `on_click_id`
/// del `UiNode` y se invoca la función JS correspondiente.
pub fn handle_click(callback_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    match invoke_js_callback(callback_id)? {
        true => Ok(()),
        false => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Fase 4: sin callback JS registrado para id '{callback_id}'"),
        )
        .into()),
    }
}

/// Handler `Fn(String)` listo para el core/CLI (los errores se loguean, no se propagan).
pub fn click_handler_from_js() -> impl Fn(String) + Send + Sync + 'static {
    move |id: String| {
        if let Err(e) = handle_click(&id) {
            eprintln!("[Axo JS] click '{id}': {e}");
        }
    }
}
