use clap::{Parser, Subcommand};
use arrival_core::{Arg, Target, Node, Runtime};

#[derive(Parser)]
#[command(name = "arrival")]
#[command(about = "Abstract layer communication framework")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ask {
        raw: String,
        #[arg(short, long)]
        verbose: bool,
    },
    List,
}

struct StringArg {
    raw: String,
}

impl Arg for StringArg {
    fn toString(&self) -> String {
        self.raw.clone()
    }
}

struct StringTarget {
    value: String,
}

impl Target for StringTarget {
    fn toString(&self) -> String {
        self.value.clone()
    }
}

struct ExampleNode;

impl Node for ExampleNode {
    fn name(&self) -> &str {
        "example"
    }

    fn provide_arg(&self) -> Box<dyn Arg> {
        Box::new(StringArg {
            raw: "provided arg".to_string(),
        })
    }

    fn arrive(&self, arg: &dyn Arg) -> Option<Box<dyn Target>> {
        if arg.toString().contains("hello") {
            Some(Box::new(StringTarget {
                value: format!("ok: {}", arg.toString()),
            }))
        } else {
            None
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ask { raw, verbose } => {
            let node = ExampleNode;
            let mut runtime = Runtime::new(&node, 10);

            let initial_arg = StringArg { raw };
            let result = runtime.run(&initial_arg);

            if verbose {
                println!("path: {:?}", runtime.path().nodes);
            }

            match result {
                Some(target) => println!("{}", target.toString()),
                None => println!("no answer"),
            }
        }
        Commands::List => {
            println!("no nodes implemented yet");
        }
    }
}
