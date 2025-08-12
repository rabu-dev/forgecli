use clap::{Parser};
mod funciones;
mod commands;
use commands::command::{Cli, Commands};
use funciones::fun::crear_proyecto;

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::New { name, stack } => {
            let project_name = name.unwrap_or_else(|| "my-project".to_string());
            let stack_type = stack.unwrap_or_else(|| "default".to_string());
                
            crear_proyecto(&project_name, &stack_type);
        }
        Commands::ListStacks => {
            println!("Listing available stacks");
        }
    }
}
