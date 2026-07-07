mod cli;
mod commands;
mod nodes;
mod configs;

use clap::Parser;
use cli::Cli;
use commands::handle_command;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let cli = Cli::parse();
    handle_command(cli);
}
