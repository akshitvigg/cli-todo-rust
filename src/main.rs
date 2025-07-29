use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};

use std::{fs, vec};
use std::io::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id : usize,
    title : String,
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

const FILE_PATH: &str = "tasks.json";

fn load_tasks () -> Vec<Task> {

    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    }else {
        vec![]
    }
}

fn save_tasks(tasks: &Vec<Task>)-> Result<()>{
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(FILE_PATH,data)
}


fn main() {
    let cli = Cli::parse();

    match &cli.command{

        Commands::Add { task }=>{
            let mut tasks = load_tasks();

            let new_task = Task{
                id: tasks.len() + 1,
                title : task.clone(),
                done : false
            };

            tasks.push(new_task);
            
            save_tasks(&tasks).expect("failed to add task");

            println!("task added successfully");

        }
        Commands::List => {
            let tasks = load_tasks();

            for task in tasks.iter() {
                let status = if task.done { "✔️" }else {"❌"};
                println!("{} : {} [ {} ]", task.id, task.title,status);
            }
        }
        Commands::Done { id } => {

            let mut tasks = load_tasks();

            if let Some(task) = tasks.iter_mut().find(|t| t.id == *id){
                task.done = true;
                save_tasks(&tasks).expect("Failed to update task");
                println!("🎉 Task {} marked as done!", id);
            }else {
                println!("⚠️ Task with ID {} not found", id);
            }
            
        }
    }
}
