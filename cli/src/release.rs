use clap::Args;
use std::process::Command;

#[derive(Args)]
pub struct ReleaseArgs {
    #[arg(long)]
    pub target: Option<String>,

    #[arg(long, default_value = "false")]
    pub bundle: bool,

    #[arg(long, default_value = "axo-app")]
    pub name: String,
}

pub fn run(args: ReleaseArgs) {
    println!("[CLI] Building release...");

    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--release"]);

    if let Some(target) = &args.target {
        cmd.arg("--target");
        cmd.arg(target);
    }

    let status = cmd.status().expect("Failed to run cargo build --release");

    if !status.success() {
        eprintln!("[CLI] Release build failed");
        std::process::exit(1);
    }

    println!("[CLI] Release build complete!");

    if args.bundle {
        let target_dir = format!("target/release/{}", if cfg!(windows) { format!("{}.exe", &args.name) } else { args.name.clone() });
        let bundle_dir = format!("dist/{}", &args.name);

        println!("[CLI] Bundling into {}", bundle_dir);

        let _ = std::fs::remove_dir_all(&bundle_dir);
        std::fs::create_dir_all(&bundle_dir).unwrap();

        // Copy binary
        if std::path::Path::new(&target_dir).exists() {
            let dest = std::path::Path::new(&bundle_dir).join(std::path::Path::new(&target_dir).file_name().unwrap());
            std::fs::copy(&target_dir, &dest).unwrap();
            println!("  Copied binary");
        }

        // Copy app directory
        let app_dir = std::path::Path::new("app");
        if app_dir.exists() {
            let dest = std::path::Path::new(&bundle_dir).join("app");
            cp_r(app_dir, &dest);
            println!("  Copied app/");
        }

        println!("[CLI] Bundle created at {}", bundle_dir);
    }
}

fn cp_r(src: &std::path::Path, dst: &std::path::Path) {
    if src.is_dir() {
        std::fs::create_dir_all(dst).unwrap();
        for entry in std::fs::read_dir(src).unwrap() {
            let entry = entry.unwrap();
            cp_r(&entry.path(), &dst.join(entry.file_name()));
        }
    } else {
        std::fs::copy(src, dst).unwrap();
    }
}
