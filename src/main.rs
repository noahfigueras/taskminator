use std::io::prelude::*;
use std::env;
use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    uid: u8,
    task: String,
    due: String,
    tag: String,
    status: String
}

const PATH: &str = "./db/pending.json";

fn main() {
    let args: Vec<String> = env::args().collect();    

    if args[1] == "add" {
        add_todo(&args[2]);
    } else if args[1] == "list" {
        list_tasks();
    }
    println!("{:?}", args);
}

fn list_tasks() {
    let file = File::open(PATH).expect("File not found");
    let tasks: Vec<Task> = serde_json::from_reader(file).expect("error while reading");

    //Print tasks
    for task in tasks {
        println!("Task: {}", task.task);
    }
}

fn add_todo(task: &str) -> std::io::Result<()> {
    let mut file = File::open("./db/pending.json")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    
    //Push new task
    let fmt_task = format!("['task': '{}'],\n}}", task);
    content = content.replace("}", &fmt_task);
    println!("{}", content);
    Ok(())
}
