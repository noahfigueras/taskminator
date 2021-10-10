// Command Functions
use serde::{Deserialize, Serialize};

mod aux;
use aux::{read_json, write_json};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    uid: String,
    task: String,
    due: String,
    project: String,
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
    for(i, task) in tasks.iter().enumerate() {
        if task.status == "pending" {
            println!("{}: {}", i, task.task);
        } else {
            counter = counter + 1;
       }
    }
    println!("Tasks Completed: {}", counter);
}

pub fn remove_task(id: &str) {
    let mut tasks: Vec<Task> = read_json().expect("error while reading");

    //Remove 
    tasks.remove((id.parse::<u8>().unwrap()).into());

    // Write to file
    write_json(&tasks).expect("Couldn't write to file");
    println!("Task Deleted Successfully");
}

pub fn add_task(todo: Vec<String>) {
    let mut tasks: Vec<Task> = read_json().expect("error while reading");

    //Create Task
    let mut task =  Task {
        uid: "123543205".to_string(),
        task: (&todo[0]).to_string(),
        due: "".to_string(),
        project: "".to_string(),
        status: "pending".to_string()
    };

    //Add commands to add info to task.due and task.project
    if todo.len() > 1 {
        for (index,value) in todo.iter().enumerate() {
            if (index+1 < todo.len()) && (index > 0) {
                match value.as_str() {
                    "-d" => {
                        println!("Date: {}", todo[index+1]);
                    }
                    "-p" => {
                        task.project = (&todo[index+1]).to_string();
                    } 
                    _=> {
                        println!("Incorrect commant {} please check --help to see all commands", value);
                    }
                }
            }         
        }
    }

    //Update and Write
    tasks.push(task);
    write_json(&tasks).expect("Unable to write file");

    println!("Task Added: {}", &todo[0]);
}

pub fn completed_task(index: &str) {
    let mut tasks: Vec<Task> = read_json().expect("error while reading");

    for task in &mut tasks {
        if task.uid == index {
            task.status = String::from("completed");
            break;
        }
    }

    // Write to file
    write_json(&tasks).expect("Unable to write file");

    println!("Task {} Added as completed!",&index);
}
