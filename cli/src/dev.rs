use std::sync::{Arc, Mutex};

use clap::Args;

use axo_core::window::{ClickHandler, RebuildFn};

#[derive(Args)]
pub struct DevArgs {
    #[arg(long, default_value = "app/app.ts")]
    pub entry: String,

    #[arg(long, default_value_t = 9876)]
    pub port: u16,

    #[arg(long, default_value_t = false)]
    pub no_watch: bool,

    #[arg(long, default_value_t = false)]
    pub no_qr: bool,
}

fn platform_indicator() -> &'static str {
    if cfg!(target_os = "windows") {
        "w"
    } else if cfg!(target_os = "macos") {
        "m"
    } else if cfg!(target_os = "linux") {
        if std::env::var("ANDROID_ROOT").is_ok() || std::env::var("TERMINUX_VERSION").is_ok() {
            "a"
        } else {
            "l"
        }
    } else if cfg!(target_os = "ios") {
        "i"
    } else if cfg!(target_os = "android") {
        "a"
    } else {
        "?"
    }
}

fn platform_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "linux") {
        if std::env::var("ANDROID_ROOT").is_ok() || std::env::var("TERMINUX_VERSION").is_ok() {
            "Android (Termux)"
        } else {
            "Linux"
        }
    } else if cfg!(target_os = "ios") {
        "iOS"
    } else if cfg!(target_os = "android") {
        "Android"
    } else {
        "Unknown"
    }
}

fn print_banner(port: u16) {
    let indicator = platform_indicator();
    let os_name = platform_name();

    // QR code if available
    #[cfg(feature = "qr")]
    {
        let local_ip = local_ip_address();
        let url = format!("http://{}:{}", local_ip, port);
        if let Ok(qr) = qrcode::QrCode::new(&url) {
            let qr_str = render_qr(&qr);
            println!();
            println!("  ╔═══════════════════════════════════════╗");
            println!("  ║        Axo Framework Dev              ║");
            println!("  ╠═══════════════════════════════════════╣");
            println!("  ║  Platform : {}  {}                     ", indicator, os_name);
            println!("  ║  URL      : {:<31} ║", url);
            println!("  ║  Hot reload: ON                        ║");
            println!("  ╠═══════════════════════════════════════╣");
            println!("  ║  Scan QR on your phone:               ║");
            for line in qr_str.lines() {
                println!("  ║  {}  ║", line);
            }
            println!("  ╠═══════════════════════════════════════╣");
            println!("  ║  [r] Restart    [q] Quit              ║");
            println!("  ╚═══════════════════════════════════════╝");
            println!();
            return;
        }
    }

    // Fallback without QR
    println!();
    println!("  ╔═══════════════════════════════════════╗");
    println!("  ║        Axo Framework Dev              ║");
    println!("  ╠═══════════════════════════════════════╣");
    println!("  ║  Platform : {}  {}                     ", indicator, os_name);
    println!("  ║  Hot reload: ON                        ║");
    println!("  ╠═══════════════════════════════════════╣");
    println!("  ║  [r] Restart    [q] Quit              ║");
    println!("  ╚═══════════════════════════════════════╝");
    println!();
}

#[cfg(feature = "qr")]
fn local_ip_address() -> String {
    // Try to get the local IP address of the first non-loopback interface
    if let Ok(addrs) = local_ip_address::local_ip() {
        return addrs.to_string();
    }
    "127.0.0.1".to_string()
}

#[cfg(feature = "qr")]
fn render_qr(qr: &qrcode::QrCode) -> String {
    let mut output = String::new();
    let size = qr.width();
    for y in 0..size {
        output.push_str("       ");
        for x in 0..size {
            let dark = qr[(x, y)] == qrcode::types::Color::Dark;
            output.push(if dark { '█' } else { ' ' });
        }
        output.push('\n');
    }
    output
}

fn resolve_entry(requested: &str) -> Result<String, String> {
    if std::path::Path::new(requested).exists() {
        return Ok(requested.to_string());
    }
    if requested.ends_with(".lua") {
        return Err(format!(
            "Axo ya no usa Lua. Entrypoint no encontrado: {requested} (usa app/app.ts o app/app.js)"
        ));
    }
    for cand in ["app/app.ts", "app/app.js"] {
        if std::path::Path::new(cand).exists() {
            return Ok(cand.to_string());
        }
    }
    Err(format!(
        "Entrypoint no encontrado: {requested} (buscado app/app.ts, app/app.js)"
    ))
}

fn load_and_build(entry: &str, viewport_w: f32, viewport_h: f32) -> Option<Vec<axo_core::Rect>> {
    match axo_bridge::load_app_auto(entry) {
        Ok(root) => Some(build_rects(&root, viewport_w, viewport_h)),
        Err(e) => {
            eprintln!("  [JS] Failed to load app: {}", e);
            None
        }
    }
}

fn build_rects(root: &axo_bridge::serde::UiNode, viewport_w: f32, viewport_h: f32) -> Vec<axo_core::Rect> {
    let mut engine = axo_core::layout::Engine::new();
    let rects = axo_bridge::taffy_conv::build_rects_simple(
        &mut engine, root, viewport_w, viewport_h,
    );
    println!("  [CLI] Built {} rectangles", rects.len());
    rects
}

fn make_click_handler(
    shared_rects: Arc<Mutex<Vec<axo_core::Rect>>>,
    entry: String,
) -> ClickHandler {
    let js_click = axo_bridge::callbacks::click_handler_from_js();
    Arc::new(move |_, cb_id, _, _| {
        if cb_id.is_empty() {
            return;
        }
        js_click(cb_id.to_string());

        let viewport_w = 1024.0;
        let viewport_h = 768.0;
        if let Some(rects) = load_and_build(&entry, viewport_w, viewport_h) {
            *shared_rects.lock().unwrap() = rects;
        }
    })
}

fn make_rebuild_fn(entry: String) -> RebuildFn {
    Arc::new(move |viewport_w: f32, viewport_h: f32| -> Vec<axo_core::Rect> {
        load_and_build(&entry, viewport_w, viewport_h).unwrap_or_default()
    })
}

fn start_http_server(port: u16, _shared_rects: Arc<Mutex<Vec<axo_core::Rect>>>) {
    std::thread::spawn(move || {
        let addr = format!("0.0.0.0:{}", port);
        let listener = match std::net::TcpListener::bind(&addr) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("  [Server] Failed to bind {}: {}", addr, e);
                return;
            }
        };
        listener.set_nonblocking(true).ok();
        println!("  [Server] Listening on http://0.0.0.0:{}", port);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    use std::io::{Read, Write};
                    let mut buf = [0u8; 4096];
                    let _ = stream.read(&mut buf);

                    let body = r#"<!DOCTYPE html>
<html><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Axo Preview</title>
<style>
body { font-family: system-ui, sans-serif; background: #1a1a2e; color: #fff; display: flex; align-items: center; justify-content: center; min-height: 100vh; margin: 0; }
.container { text-align: center; padding: 2rem; }
h1 { color: #e94560; }
p { opacity: 0.7; }
</style></head><body>
<div class="container">
<h1>Axo Framework</h1>
<p>Crea m&aacute;s r&aacute;pido. Hazlo completo. Exti&eacute;ndelo todo.</p>
<p>Esta es una vista previa remota. La aplicaci&oacute;n se renderiza en el dispositivo de escritorio.</p>
</div></body></html>"#;
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(), body
                    );
                    let _ = stream.write_all(response.as_bytes());
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                Err(_) => break,
            }
        }
    });
}

pub fn run(args: DevArgs) {
    print_banner(args.port);

    // Start HTTP server for phone preview
    if !args.no_qr {
        let shared_rects = Arc::new(Mutex::new(Vec::new()));
        start_http_server(args.port, shared_rects);
    }

    let shared_rects = Arc::new(Mutex::new(Vec::new()));
    let entry = match resolve_entry(&args.entry) {
        Ok(e) => e,
        Err(msg) => {
            eprintln!("  [Dev] {msg}");
            return;
        }
    };

    // Initial load
    if let Some(rects) = load_and_build(&entry, 1024.0, 768.0) {
        *shared_rects.lock().unwrap() = rects;
    }

    let click_handler = make_click_handler(shared_rects.clone(), entry.clone());
    let rebuild_fn = make_rebuild_fn(entry.clone());

    // Start file watcher for hot reload (JS/TS via bridge)
    if !args.no_watch {
        let entry_watch = entry.clone();
        let shared = shared_rects.clone();
        if let Err(e) = axo_bridge::watch::watch_and_reload(&entry_watch, move |root| {
            let rects = build_rects(&root, 1024.0, 768.0);
            *shared.lock().unwrap() = rects;
            println!("  [HotReload] UI updated!");
        }) {
            eprintln!("  [HotReload] No se pudo iniciar el watcher: {e}");
        }
    }

    // Start key listener thread for [r] restart and [q] quit
    let shared = shared_rects.clone();
    let entry_r = entry.clone();
    std::thread::spawn(move || {
        use std::io::Read;
        let mut stdin_buf = [0u8; 1];
        loop {
            match std::io::stdin().read(&mut stdin_buf) {
                Ok(0) | Err(_) => break,
                Ok(_) => {
                    match stdin_buf[0] as char {
                        'r' | 'R' => {
                            println!("  [Dev] Restarting...");
                            if let Some(rects) = load_and_build(&entry_r, 1024.0, 768.0) {
                                *shared.lock().unwrap() = rects;
                                println!("  [Dev] Restarted!");
                            }
                        }
                        'q' | 'Q' => {
                            println!("  [Dev] Quitting...");
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
            }
        }
    });

    axo_core::window::run_with_shared_rects_and_handler(shared_rects, Some(click_handler), Some(rebuild_fn));
}
