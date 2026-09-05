// Fase 4 — Runtime JS retenido: un contexto QuickJS compartido entre load_* e invoke.
use std::sync::{Mutex, OnceLock};
use rquickjs::{Context, Ctx, Runtime};

struct Retained {
    ctx: Context,
}

static APP: OnceLock<Mutex<Option<Retained>>> = OnceLock::new();

fn slot() -> &'static Mutex<Option<Retained>> {
    APP.get_or_init(|| Mutex::new(None))
}

// Fase 4: crea o reutiliza el contexto retenido (console + Device se registran una sola vez).
fn ensure() -> Result<Context, Box<dyn std::error::Error>> {
    {
        let guard = slot().lock().expect("Fase 4: lock del runtime JS");
        if let Some(r) = guard.as_ref() {
            return Ok(r.ctx.clone());
        }
    }
    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;
    ctx.with(|ctx| -> Result<(), rquickjs::Error> {
        crate::runtime::register_console(ctx.clone())?;
        crate::js_device::register_js_device(ctx)?;
        Ok(())
    })?;
    slot()
        .lock()
        .expect("Fase 4: lock del runtime JS")
        .replace(Retained { ctx: ctx.clone() });
    Ok(ctx)
}

// Fase 4: ejecuta f con el contexto retenido (el lock solo se usa para crear/clonar).
pub(crate) fn with_ctx<R>(f: impl FnOnce(Ctx) -> R) -> Result<R, Box<dyn std::error::Error>> {
    let ctx = ensure()?;
    Ok(ctx.with(f))
}

// Fase 4: descarta el runtime retenido (hot-reload); los ids de callbacks anteriores se invalidan.
pub fn reset_js_app_runtime() {
    *slot().lock().expect("Fase 4: lock del runtime JS") = None;
}
