mod cli;
mod commands;
mod nodes;
mod args;

use clap::Parser;
use cli::Cli;
use commands::handle_command;
use nodes::{RootNode, ChildNode};
use arrival_core::Runtime;

fn main() {
    let cli = Cli::parse();

    let mut runtime = Runtime::new();
    runtime.add_node(Box::new(RootNode));
    runtime.add_node(Box::new(ChildNode));

    handle_command(cli, runtime);
}
