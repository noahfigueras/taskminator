use std::env;
use std::fs::{File, write};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    uid: String,
    task: String,
    due: String,
    tag: String,
    status: String
}

const PATH: &str = "./db/pending.json";

fn read_json() -> Result<Vec<Task>> {
    let file = File::open(PATH).expect("File not found");
    let j: Vec<Task> = serde_json::from_reader(file)?;
    Ok(j)
}

fn write_json(tasks: &Vec<Task>) -> Result<()> {
    let _json: String = serde_json::to_string(&tasks).expect("Error parsing to json");
    write(PATH, &_json).expect("Unable to write file");
    println!("Task Deleted Successfully");
    Ok(())
}

fn help() {
        println!("usage:
                 match_args <string>
                     Check whether given string is the answer.
                     match_args {{increase|decrease}} <integer>
                         Increase or decrease given integer by one.");
}

fn list_tasks() {
    let tasks: Vec<Task> = read_json().expect("error while reading");
    
    let mut counter: u8 = 0;
    //Print tasks
    for task in &tasks {
        if task.status == "pending" {
            println!("Task: {}", task.task);
        } else {
            counter = counter + 1;
        }
    }
    println!("Tasks Completed: {}", counter);
}

fn remove_task(id: &str) {
    let mut tasks: Vec<Task> = read_json().expect("error while reading");

    //Find taskbyId
    let mut found: bool = false;
    for task in &tasks {
        if task.uid == id {
            tasks.remove((id.parse::<u8>().unwrap()).into());
            found = true;
            break;
        }
    }

    // Write to file
    if found {
        write_json(&tasks).expect("Couldn't write to file");
    } else {
        println!("Task Not found")
    }
}

fn add_task(todo: &str) {
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
    let _json: String = serde_json::to_string(&tasks).expect("Json not parsed correctly");
    // Write to file
    write(PATH, &_json).expect("Unable to write file");

    println!("Task Added: {}", &todo);
}

fn completed_task(index: &str) {
    let file = File::open(PATH).expect("File not found");
    let mut tasks: Vec<Task> = serde_json::from_reader(file).expect("error while reading");

    for task in &mut tasks {
        if task.uid == index {
            task.status = String::from("completed");
            break;
        }
    }
    println!("{:?}", tasks);
    let _json: String = serde_json::to_string(&tasks).expect("Json not parsed correctly");
    // Write to file
    write(PATH, &_json).expect("Unable to write file");

    println!("Task {} Added as completed!",&index);
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
                    completed_task(&args[2]);
               }
               "--help" => {
                    help();
               }
               _ => {
                    println!("Incorrect Arguments please use the --help command
                             to check all of the possible options.");
               }
            }
        }
    }
}

