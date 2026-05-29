use clap::{Parser, Subcommand};
use axo_cli::{init, dev, build, release};

#[derive(Parser)]
#[command(name = "axo", about = "Crea más rápido. Hazlo completo. Extiéndelo todo.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(init::InitArgs),
    Dev(dev::DevArgs),
    Build(build::BuildArgs),
    Release(release::ReleaseArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => init::run(args),
        Commands::Dev(args) => dev::run(args),
        Commands::Build(args) => build::run(args),
        Commands::Release(args) => release::run(args),
    }
}
