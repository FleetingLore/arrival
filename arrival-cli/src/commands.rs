use crate::cli::{Cli, Commands, NodeType};
use crate::configs::SAMPLE_TOML;
use crate::nodes::{CustomNode, create_cli_return_nodes, create_string_nodes};
use arrival_core::{Arg, Runtime, Trace};
use arrival_toml::from_str;
use log::{debug, error, info, warn};

pub fn handle_command(cli: Cli) {
    match cli.command {
        Commands::Builtin {
            raw,
            verbose,
            node_type,
        } => {
            let mut runtime = Runtime::new();
            let node_type = node_type.unwrap_or(NodeType::Custom);

            info!("using builtin nodes, type: {:?}", node_type);
            debug!("raw input: {}", raw);

            match node_type {
                NodeType::Custom => {
                    debug!("adding custom nodes");
                    runtime.add_node(Box::new(CustomNode::new("root")));
                    runtime.add_node(Box::new(CustomNode::new("root/child")));
                }
                NodeType::String => {
                    debug!("adding string nodes");
                    for node in create_string_nodes() {
                        runtime.add_node(node);
                    }
                }
                NodeType::CliReturn => {
                    debug!("adding cli-return nodes");
                    for node in create_cli_return_nodes() {
                        runtime.add_node(node);
                    }
                }
            }

            let initial_arg = Box::new(StringArg { raw });
            info!("starting runtime from root");
            let result = runtime.run(initial_arg, Trace::from_str("root"));

            if verbose {
                println!("path: {:?}", runtime.path().segments_str());
            }

            match result {
                Some(target) => {
                    info!("got result: {}", target.to_string());
                    println!("{}", target.to_string());
                }
                None => {
                    warn!("no answer");
                    println!("no answer");
                }
            }
        }
        Commands::Toml {
            raw,
            verbose,
            config,
        } => {
            info!("loading TOML config from: {}", config);
            debug!("raw input: {}", raw);

            let content = std::fs::read_to_string(&config).unwrap_or_else(|_| {
                warn!("reading {} failed, using sample config", config);
                SAMPLE_TOML.to_string()
            });

            let mut runtime = match from_str(&content) {
                Ok(r) => {
                    info!("TOML parsed successfully");
                    r
                }
                Err(e) => {
                    error!("parse error: {}", e);
                    std::process::exit(1);
                }
            };

            let initial_arg = Box::new(StringArg { raw });
            info!("starting runtime from root");
            let result = runtime.run(initial_arg, Trace::from_str("root"));

            if verbose {
                println!("path: {:?}", runtime.path().segments_str());
            }

            match result {
                Some(target) => {
                    info!("got result: {}", target.to_string());
                    println!("{}", target.to_string());
                }
                None => {
                    warn!("no answer");
                    println!("no answer");
                }
            }
        }
        Commands::List => {
            info!("listing all nodes");

            let mut runtime = Runtime::new();
            runtime.add_node(Box::new(CustomNode::new("root")));
            runtime.add_node(Box::new(CustomNode::new("root/child")));

            println!("custom nodes:");
            for node in runtime.iter_nodes() {
                println!("  {}", node.path().to_string());
            }

            println!("\nstring nodes:");
            for node in create_string_nodes() {
                println!("  {}", node.path().to_string());
            }

            println!("\ncli-return nodes:");
            for node in create_cli_return_nodes() {
                println!("  {}", node.path().to_string());
            }

            println!("\ntoml sample:");
            println!("{}", SAMPLE_TOML);
        }
    }
}

pub struct StringArg {
    pub raw: String,
}

impl Arg for StringArg {
    fn to_string(&self) -> String {
        self.raw.clone()
    }
}
