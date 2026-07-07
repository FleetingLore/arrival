use crate::cli::{Cli, Commands};
use crate::args::StringArg;
use arrival_core::{Runtime, Path};

pub fn handle_command(cli: Cli, mut runtime: Runtime) {
    match cli.command {
        Commands::Ask { raw, verbose } => {
            let initial_arg = Box::new(StringArg { raw });
            let result = runtime.run(initial_arg, Path::from_str("root"));

            if verbose {
                println!("path: {:?}", runtime.path().nodes);
            }

            match result {
                Some(target) => println!("{}", target.to_string()),
                None => println!("no answer"),
            }
        }
        Commands::List => {
            println!("nodes:");
            for node in runtime.iter_nodes() {
                println!("  {}", node.path().to_string());
            }
        }
    }
}
