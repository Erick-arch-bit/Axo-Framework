// Fase 3 — Transpile TypeScript → JavaScript (Opción B: esbuild vía npx).
use std::io::Write;
use std::process::{Command, Stdio};

/// Transpila TypeScript a JavaScript.
/// Entrada: código fuente TS como string.
/// Salida: código JS como string.
pub fn transpile_ts_to_js(
    source: &str,
    filename_hint: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Fase 3 (Opción B): esbuild vía npx, fuente por stdin, JS por stdout.
    let mut child = Command::new("npx")
        .args([
            "--yes",
            "esbuild",
            "--loader=ts",
            "--format=esm",
            "--target=es2020",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Fase 3: no se pudo ejecutar `npx esbuild` (instala Node.js 18+ con `fnm use stable` o desde https://nodejs.org) [archivo: {filename_hint}]: {e}"
                ),
            )
        })?;
    child
        .stdin
        .take()
        .expect("Fase 3: stdin con pipe")
        .write_all(source.as_bytes())
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                format!("Fase 3: no se pudo enviar el fuente a esbuild [archivo: {filename_hint}]: {e}"),
            )
        })?;
    let output = child.wait_with_output().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Fase 3: error esperando a esbuild [archivo: {filename_hint}]: {e}"),
        )
    })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Fase 3: esbuild falló [archivo: {filename_hint}]: {stderr}"),
        )
        .into());
    }
    String::from_utf8(output.stdout)
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Fase 3: salida no-UTF8 de esbuild [archivo: {filename_hint}]: {e}"),
            )
        })
        .map_err(|e| e.into())
}
