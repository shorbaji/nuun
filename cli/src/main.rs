//! The nuun CLI

mod run;
mod dust;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nuun", version = "0.1.0", about = "The nuun cli")]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[clap(short='H', long, default_value = "localhost")]
    host: Option<String>,

    #[clap(short, long, default_value = "8000")]
    port: Option<u16>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Dust {
        #[command(subcommand)]
        cmd: dust::DustCommand,
    },
    Run {
        #[clap(short, long)]
        debug: bool,
        #[clap(short, long, default_value = "python")]
        language: run::Language,
        program: String,
        expr: Option<String>,
    },
    Start,
    Stop,
    Restart,
    Status,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let host = cli.host.unwrap();
    let port = cli.port.unwrap();

    match cli.command {
        Command::Dust { cmd } => dust::run(cmd),
        Command::Run { debug, language, program, expr } => run::run(debug, language, program, expr, host, port).await,
        _ => unimplemented!("Command not implemented"),
    }

}
