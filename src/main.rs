use std::env;
use std::fs::{File, write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    uid: u8,
    task: String,
    due: String,
    tag: String,
    status: String
}

const PATH: &str = "./db/pending.json";

fn help() {
        println!("usage:
                 match_args <string>
                     Check whether given string is the answer.
                     match_args {{increase|decrease}} <integer>
                         Increase or decrease given integer by one.");
}

fn list_tasks() {
    let file = File::open(PATH).expect("File not found");
    let tasks: Vec<Task> = serde_json::from_reader(file).expect("error while reading");

    //Print tasks
    for task in tasks {
        println!("Task: {}", task.task);
    }
}

fn add_todo(todo: &str) -> std::io::Result<()>{
    let mut file = File::open(PATH).expect("File not found");
    let mut tasks: Vec<Task> = serde_json::from_reader(file).expect("error while reading");

    let task =  Task {
        uid: 0,
        task: (&todo).to_string(),
        due: "110921".to_string(),
        tag: "computers".to_string(),
        status: "pending".to_string()
    };

    tasks.push(task);
    let _json: String = serde_json::to_string(&tasks)?;

    write(PATH, &_json).expect("Unable to write file");

    println!("{:?}", tasks);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();    
    let cmd: &str = &args[1]; 

    match cmd {
        "list" => {
            list_tasks();
        }
        "add" => {
            add_todo(&args[2]);
        }
        _ => {
              help();
            }
        }
}

