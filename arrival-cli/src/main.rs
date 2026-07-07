use clap::{Parser, Subcommand};
use arrival_core::{Arg, Target, Node, NodeResult, Runtime, Path};

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
    fn path(&self) -> Path {
        Path::from_str("root")
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        let s = arg.to_string();
        if s.contains("hello") {
            NodeResult::Done(Box::new(StringTarget {
                value: format!("root done: {}", s),
            }))
        } else if s.contains("child") {
            NodeResult::Next(
                Box::new(StringArg {
                    raw: format!("root forwarded: {}", s),
                }),
                Path::from_str("root/child"),
            )
        } else {
            NodeResult::Next(
                Box::new(StringArg {
                    raw: format!("root next: {}", s),
                }),
                Path::from_str("root"),
            )
        }
    }
}

struct ChildNode;

impl Node for ChildNode {
    fn path(&self) -> Path {
        Path::from_str("root/child")
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        let s = arg.to_string();
        if s.contains("world") {
            NodeResult::Done(Box::new(StringTarget {
                value: format!("child done: {}", s),
            }))
        } else {
            NodeResult::Next(
                Box::new(StringArg {
                    raw: format!("child next: {}", s),
                }),
                Path::from_str("root/child"),
            )
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let mut runtime = Runtime::new();
    runtime.add_node(Box::new(RootNode));
    runtime.add_node(Box::new(ChildNode));

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
