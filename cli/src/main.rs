use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lumina", about = "Crea más rápido. Hazlo completo. Extiéndelo todo.")]
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

mod init;
mod dev;
mod build;
mod release;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => init::run(args),
        Commands::Dev(args) => dev::run(args),
        Commands::Build(args) => build::run(args),
        Commands::Release(args) => release::run(args),
    }
}
