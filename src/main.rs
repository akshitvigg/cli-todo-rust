use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id : usize,
    task : String,
    done : bool
}

#[derive(Parser)]
#[command(name= "todo cli")]
#[command(about = " a simple todo cli written in rust", long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: Commands
}


#[derive(Subcommand)]
enum Commands {
    Add {task : String},
    List,
    Done {id:usize}
}
fn main() {
    let cli = Cli::parse();

    match &cli.command{

        Commands::Add { task } => {

        }
        Commands::List => {

        }
        Commands::Done { id } => {
            
        }
    }
}
