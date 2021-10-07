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

fn add_task(todo: &str) -> std::io::Result<()>{
    let file = File::open(PATH).expect("File not found");
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

//Helper fn

fn main() {
    let args: Vec<String> = env::args().collect();    

    println!("Arguments: {}", args.len());
    match args.len() {
        1 => {
            list_tasks();
        } 
        _ => {
            match args[1].as_str() {
               "-a" => {
                    add_task(&args[2]);
               } 
               "-r" => {
                    println!("Remove Todo");
               }
               "-c" => {
                    println!("Completed Task");
               }
               "--help" => {
                    println!("Show Help")
               }
               _ => {
                    println!("Incorrect Arguments please use the --help command
                             to check all of the possible options.");
               }
            }
            println!("Everything else");
        }
    }
}

