// Fase 3 — Hot-reload mínimo para apps JS/TS (carga inicial + watcher con debounce).
use notify::{EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// Fase 3: debounce simple para evitar recargas múltiples (100–200 ms).
const DEBOUNCE_MS: u64 = 200;

// Fase 3: resuelve el entry a una ruta existente (mismos fallbacks que los smoke tests).
fn resolve_entry(entry_path: &str) -> Option<PathBuf> {
    let candidates = [
        PathBuf::from(entry_path),
        PathBuf::from(format!("../{entry_path}")),
        PathBuf::from(format!("{}/../{entry_path}", env!("CARGO_MANIFEST_DIR"))),
    ];
    candidates.into_iter().find(|p| p.is_file())
}

// Fase 3: resuelve el directorio de la stdlib (app/axo) si existe.
fn resolve_axo_dir() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("app/axo"),
        PathBuf::from("../app/axo"),
        PathBuf::from(format!("{}/../app/axo", env!("CARGO_MANIFEST_DIR"))),
    ];
    candidates.into_iter().find(|p| p.is_dir())
}

/// Carga inicial con `load_app_auto` y observa el directorio de la app y `app/axo/`.
/// En cambios de `.js` / `.ts` → vuelve a cargar y llama `on_reload` con el nuevo `UiNode`.
pub fn watch_and_reload(
    entry_path: &str,
    on_reload: impl Fn(crate::serde::UiNode) + Send + Sync + 'static,
) -> Result<(), Box<dyn std::error::Error>> {
    // Fase 3: carga inicial con load_app_auto.
    let entry = resolve_entry(entry_path).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Fase 3: entry no encontrado: {entry_path}"),
        )
    })?;
    let entry_str = entry.to_string_lossy().to_string();
    on_reload(crate::load_app_auto(&entry_str)?);

    // Fase 3: directorios a observar (padre del entry + stdlib si es distinto).
    let mut dirs: Vec<PathBuf> = Vec::new();
    if let Some(parent) = entry.parent() {
        dirs.push(parent.to_path_buf());
    }
    if let Some(axo) = resolve_axo_dir() {
        if !dirs.contains(&axo) {
            dirs.push(axo);
        }
    }

    let on_reload = Arc::new(on_reload);
    let last = Arc::new(Mutex::new(Instant::now()));
    println!("[HotReload] Fase 3: observando {dirs:?} (entry: {entry_str})...");
    let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
        match res {
            Ok(event) => {
                // Fase 3: solo cambios relevantes en .js / .ts.
                let relevant = matches!(
                    event.kind,
                    EventKind::Modify(_) | EventKind::Create(_)
                ) && event.paths.iter().any(|p| {
                    p.extension()
                        .and_then(|e| e.to_str())
                        .map(|e| e == "js" || e == "ts")
                        .unwrap_or(false)
                });
                if !relevant {
                    return;
                }
                // Fase 3: debounce simple.
                {
                    let mut guard = last.lock().expect("Fase 3: lock de debounce");
                    if guard.elapsed() < Duration::from_millis(DEBOUNCE_MS) {
                        return;
                    }
                    *guard = Instant::now();
                }
                // Fase 4: recrear el runtime retenido (los ids de callbacks anteriores se invalidan).
                crate::app_runtime::reset_js_app_runtime();
                match crate::load_app_auto(&entry_str) {
                    Ok(node) => on_reload(node),
                    Err(e) => eprintln!("[HotReload] Fase 3: error recargando {entry_str}: {e}"),
                }
            }
            Err(e) => eprintln!("[HotReload] Fase 3: error del watcher: {e}"),
        }
    })
    .map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Fase 3: no se pudo crear el watcher: {e}"),
        )
    })?;
    for dir in &dirs {
        watcher
            .watch(dir, RecursiveMode::Recursive)
            .map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Fase 3: no se pudo observar {}: {e}", dir.display()),
                )
            })?;
    }
    // Fase 3: mantener el watcher vivo (mismo patrón que core/src/hot_reload).
    std::mem::forget(watcher);
    Ok(())
}
