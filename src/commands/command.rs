use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "forgecli")]
#[command(about = "A CLI tool for project scaffolding")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New {
        name: Option<String>,
        #[arg(long)]
        stack: Option<String>,
    },
    ListStacks,
}
