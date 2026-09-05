// Fase 1 — Runtime QuickJS (create_js_runtime, context management básico).
use rquickjs::{Context, Ctx, Function, Object, Runtime, convert::Coerced, function::Rest};

// Fase 1: registra `console.log` en el contexto dado.
// Imprime a stdout con prefijo [JS].
pub(crate) fn register_console(ctx: Ctx) -> Result<(), rquickjs::Error> {
    let globals = ctx.globals();
    let console = Object::new(ctx.clone())?;
    let log_fn = Function::new(
        ctx.clone(),
        |args: Rest<Coerced<std::string::String>>| {
            let parts: Vec<std::string::String> =
                args.iter().map(|c| c.0.clone()).collect();
            println!("[JS] {}", parts.join(" "));
        },
    )?;
    console.set("log", log_fn)?;
    globals.set("console", console)?;
    Ok(())
}

// Fase 1: crea un Runtime QuickJS y valida un contexto básico con console.log.
pub fn create_js_runtime() -> Result<Runtime, rquickjs::Error> {
    let rt = Runtime::new()?;
    // Fase 1: contexto básico para validar la creación y el registro de console.log.
    let ctx = Context::full(&rt)?;
    ctx.with(|ctx| register_console(ctx))?;
    Ok(rt)
}
