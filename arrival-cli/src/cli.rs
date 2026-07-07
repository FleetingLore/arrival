use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "arrival")]
#[command(about = "Abstract layer communication framework")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 使用硬编码的节点（三种实现）
    Builtin {
        raw: String,
        #[arg(short, long)]
        verbose: bool,
        #[arg(long)]
        node_type: Option<NodeType>,
    },
    /// 从 TOML 配置文件加载节点
    Toml {
        raw: String,
        #[arg(short, long)]
        verbose: bool,
        #[arg(long)]
        config: String,
    },
    /// 列出所有节点
    List,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum NodeType {
    Custom,
    String,
    CliReturn,
}
