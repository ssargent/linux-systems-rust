mod commands;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "syslab", about = "CLI for systems programming exercises")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Cat,
    Ls,
    Exec,
    Pipeline,
    Signald,
    TcpEcho,
    Inspect,
}

fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Cat => commands::cat::run(),
        Commands::Ls => commands::ls::run(),
        Commands::Exec => commands::exec::run(),
        Commands::Pipeline => commands::pipeline::run(),
        Commands::Signald => commands::signald::run(),
        Commands::TcpEcho => commands::tcp_echo::run(),
        Commands::Inspect => commands::inspect::run(),
    }
}
