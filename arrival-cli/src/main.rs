use clap::{Parser, Subcommand};
use arrival_core::{Arg, Target, Node, NodeResult, Registry, Runtime};

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
    fn to_string(&self) -> String {
        self.raw.clone()
    }
}

struct StringTarget {
    value: String,
}

impl Target for StringTarget {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

struct RootNode;

impl Node for RootNode {
    fn name(&self) -> &str {
        "root"
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        let s = arg.to_string();
        if s.contains("hello") {
            NodeResult::Done(Box::new(StringTarget {
                value: format!("root done: {}", s),
            }))
        } else {
            NodeResult::Next(Box::new(StringArg {
                raw: format!("root next: {}", s),
            }))
        }
    }
}

struct ChildNode;

impl Node for ChildNode {
    fn name(&self) -> &str {
        "child"
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        let s = arg.to_string();
        if s.contains("world") {
            NodeResult::Done(Box::new(StringTarget {
                value: format!("child done: {}", s),
            }))
        } else {
            NodeResult::Next(Box::new(StringArg {
                raw: format!("child next: {}", s),
            }))
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ask { raw, verbose } => {
            let mut registry = Registry::new();
            registry.register(Box::new(RootNode));
            registry.register(Box::new(ChildNode));

            let mut runtime = Runtime::new(&registry);
            let initial_arg = Box::new(StringArg { raw });
            let result = runtime.run(initial_arg);

            if verbose {
                println!("path: {:?}", runtime.path().nodes);
            }

            match result {
                Some(target) => println!("{}", target.to_string()),
                None => println!("no answer"),
            }
        }
        Commands::List => {
            println!("registered nodes:");
            println!("  root");
            println!("  child");
        }
    }
}
