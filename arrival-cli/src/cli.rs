use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "arrival")]
#[command(about = "Abstract layer communication framework")]
#[command(version = "1.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Ask {
        raw: String,
        #[arg(short, long)]
        verbose: bool,
    },
    List,
}
