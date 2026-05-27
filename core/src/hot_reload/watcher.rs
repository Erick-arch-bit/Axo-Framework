use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;

pub fn watch<F>(path: &str, mut on_change: F)
where
    F: FnMut() + Send + 'static,
{
    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>();

    let mut watcher = match notify::recommended_watcher(tx) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("[HotReload] Failed to create watcher: {}", e);
            return;
        }
    };

    if let Err(e) = watcher.watch(Path::new(path), RecursiveMode::Recursive) {
        eprintln!("[HotReload] Failed to watch {}: {}", path, e);
        return;
    }

    println!("[HotReload] Watching {} for changes...", path);

    // Keep watcher alive
    std::thread::spawn(move || {
        for event in rx {
            match event {
                Ok(event) => {
                    let is_lua_change = matches!(
                        event.kind,
                        EventKind::Modify(_) | EventKind::Create(_)
                    ) && event
                        .paths
                        .iter()
                        .any(|p| p.extension().map(|e| e == "lua").unwrap_or(false));

                    if is_lua_change {
                        println!("[HotReload] Lua file changed, reloading...");
                        on_change();
                    }
                }
                Err(e) => eprintln!("[HotReload] Watch error: {}", e),
            }
        }

        // Drop watcher when thread ends
        drop(watcher);
    });
}
