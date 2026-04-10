use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "xtask", about = "Project maintenance commands")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ci,
    LinuxCheck,
    DevcontainerCheck,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ci => println!("xtask ci: placeholder for local CI routine"),
        Commands::LinuxCheck => println!("xtask linux-check: placeholder for Linux-only checks"),
        Commands::DevcontainerCheck => {
            println!("xtask devcontainer-check: placeholder for devcontainer validation")
        }
    }
}
