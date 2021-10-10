// Command Functions
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

mod aux;
use aux::{read_json, write_json, date_fmt};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    task: String,
    due: String,
    project: String,
    status: String
}

const PATHP: &str = "./db/pending.json";
const PATHC: &str = "./db/completed.json";

pub fn help() {
        println!("usage:
                 match_args <string>
                     Check whether given string is the answer.
                     match_args {{increase|decrease}} <integer>
                         Increase or decrease given integer by one.");
}

pub fn list_tasks() {
    let tasks: Vec<Task> = read_json(PATHP).expect("error while reading");
    
    let mut completed_c: u8 = 0;
    let mut pending_c: u8 = 0;

    //Print tasks
    for(i, task) in tasks.iter().enumerate() {
        if task.status == "pending" {
            println!("{}: {}", i, task.task);
            pending_c = pending_c + 1;
        } else if task.status == "completed" {
            //Show only completed today
            let dt: DateTime<Local> = Local::now();
            println!("{}", dt.format("%Y-%m-%d").to_string());
            completed_c = completed_c + 1;
       }
    }
    println!("Completed today: {} | Total pending: {}", completed_c, pending_c);
}

pub fn remove_task(id: &str) {
    let mut tasks: Vec<Task> = read_json(PATHP).expect("error while reading");

    //Remove 
    tasks.remove((id.parse::<u8>().unwrap()).into());

    // Write to file
    write_json(&tasks, PATHP).expect("Couldn't write to file");
    println!("Task Deleted Successfully");
}

pub fn add_task(todo: Vec<String>) {
    let mut tasks: Vec<Task> = read_json(PATHP).expect("error while reading");
    let mut incorrect_cmd: bool = false;

    //Create Task
    let mut task =  Task {
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
                        //Insert Date
                        date_fmt(&todo[index+1], &mut task);
                    }
                    "-p" => {
                        //Insert Project
                        task.project = (&todo[index+1]).to_string();
                    } 
                    _=> {
                        incorrect_cmd = true;
                        println!("Incorrect commant {} please check --help to see all commands", value);
                    }
                }
            }         
        }
    }
    
    if !incorrect_cmd {
        //Update and Write
        tasks.push(task);
        write_json(&tasks, PATHP).expect("Unable to write file");

        println!("Task Added: {}", &todo[0]);
    } 
}

pub fn completed_task(index: &str) {
    let mut tasks: Vec<Task> = read_json(PATHP).expect("error while reading");
    let task_i: u8 = index.parse::<u8>().unwrap();

    let mut i: u8 = 0;
    for task in &mut tasks {
        if i == task_i {
            task.status = String::from("completed");
            break;
        }
        i = i+1;
    }

    // Write to file
    write_json(&tasks, PATHP).expect("Unable to write file");

    println!("Task {} Added as completed!",&index);
}
