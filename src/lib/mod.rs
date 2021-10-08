// Helper functions to manipulate files
use std::fs::{File, write};
use serde::{Deserialize, Serialize};

mod aux;
use aux::{read_json, write_json};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    uid: String,
    task: String,
    due: String,
    tag: String,
    status: String
}

const PATH: &str = "./db/pending.json";

pub fn help() {
        println!("usage:
                 match_args <string>
                     Check whether given string is the answer.
                     match_args {{increase|decrease}} <integer>
                         Increase or decrease given integer by one.");
}

pub fn list_tasks() {
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

pub fn remove_task(id: &str) {
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

pub fn add_task(todo: &str) {
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

pub fn completed_task(index: &str) {
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
