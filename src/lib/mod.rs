// Command Functions
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

mod aux;
use aux::{
    read_json,
    write_json,
    date_fmt,
    append_json,
    completed_count
};

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
    
    let mut pending_c: u8 = 0;

    //Print tasks
    for(i, task) in tasks.iter().enumerate() {
        println!("{}: {}", i, task.task);
        pending_c = pending_c + 1;
    }

    //Get Today's date
    let dt: DateTime<Local> = Local::now();
    let today: String = dt.format("%Y-%m-%d").to_string();
    println!("Completed today: {} | Total pending: {}", completed_count(today), pending_c);
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

    //Check for incorrect paramaters after task: String added
    if todo.len() > 1 {
        for (index,value) in todo.iter().enumerate() {
            //Check for unrecoverable error
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
    let tasks: Vec<Task> = read_json(PATHP).expect("error while reading");
    let task_i: u8 = index.parse::<u8>().unwrap();


    let mut i: u8 = 0;
    //Move to completed.json
    for mut task in tasks {
        if i == task_i {
            task.due = Local::now().format("%Y-%m-%d").to_string();
            append_json(task,PATHC);
            remove_task(index);
            println!("Task {} Added as completed!",&index);
            break;
        }
        i = i+1;
    }
}
