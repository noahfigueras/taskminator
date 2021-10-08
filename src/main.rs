use std::env;
use std::fs::{File, write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    uid: String,
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

fn remove_task(id: &str) {
    let file = File::open(PATH).expect("File not found");
    let mut tasks: Vec<Task> = serde_json::from_reader(file).expect("error while reading");
    //Find taskbyId
    let mut found: bool = false;
    for task in &tasks {
        if task.uid == id {
            tasks.remove((id.parse::<u8>().unwrap()).into());
            found = true;
            break;
        }
    }
    if found {
        // Write to file
        let _json: String = serde_json::to_string(&tasks).expect("Error parsing to json");
        write(PATH, &_json).expect("Unable to write file");
        println!("Task Deleted Successfully");
    } else {
        println!("Task Not found")
    }
}

fn add_task(todo: &str) -> std::io::Result<()>{
    let file = File::open(PATH).expect("File not found");
    let mut tasks: Vec<Task> = serde_json::from_reader(file).expect("error while reading");
    //Create Task
    let task =  Task {
        uid: "0".to_string(),
        task: (&todo).to_string(),
        due: "110921".to_string(),
        tag: "computers".to_string(),
        status: "pending".to_string()
    };
    //Update Json
    tasks.push(task);
    let _json: String = serde_json::to_string(&tasks)?;
    // Write to file
    write(PATH, &_json).expect("Unable to write file");

    println!("{:?}", tasks);
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();    

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
                    remove_task(&args[2]);
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
        }
    }
}

