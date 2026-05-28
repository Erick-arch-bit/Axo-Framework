use clap::Args;
use std::process::Command;

#[derive(Args)]
pub struct BuildArgs {
    #[arg(long, default_value = "debug")]
    pub mode: String,

    #[arg(long)]
    pub target: Option<String>,

    #[arg(long, default_value = "false")]
    pub run: bool,
}

pub fn run(args: BuildArgs) {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    if args.mode.as_str() == "release" {
        cmd.arg("--release");
    }

    if let Some(target) = &args.target {
        cmd.arg("--target");
        cmd.arg(target);
    }

    println!("[CLI] Building Axo ({})...", args.mode);
    println!("[CLI] Running: {:?}", cmd);

    let status = cmd.status().expect("Failed to run cargo build");

    if status.success() {
        println!("[CLI] Build complete!");

        if args.run {
            println!("[CLI] Running...");
            let mut run_cmd = Command::new("cargo");
            run_cmd.arg("run");
            if args.mode == "release" {
                run_cmd.arg("--release");
            }
            run_cmd.status().expect("Failed to run");
        }
    } else {
        eprintln!("[CLI] Build failed");
        std::process::exit(1);
    }
}
