use crate::lib::{Task, PATHC};
use std::fs::{File, write};
use serde_json::Result;
use regex::Regex;

pub fn read_json(path: &str) -> Result<Vec<Task>> {
    let file = File::open(path).expect("File not found");
    let j: Vec<Task> = serde_json::from_reader(file)?;
    Ok(j)
}

pub fn write_json(tasks: &Vec<Task>, path: &str) -> Result<()> {
    let _json: String = serde_json::to_string(&tasks).expect("Error parsing to json");
    write(path, &_json).expect("Unable to write file");
    Ok(())
}

pub fn date_fmt(date: &String, task: &mut Task) {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if re.is_match(&date) {
        task.due = (&date).to_string();
    } else {
        println!("Wrong Format for date please enter: YYYY-MM-DD")
    }
} 

pub fn append_json(task: Task, path: &str) {
   let mut tasks: Vec<Task> = read_json(path).expect("error while reading file");
   tasks.push(task);
   write_json(&tasks, path).expect("Unable to write file");
}

pub fn completed_count(today: String) -> String {
    let tasks: Vec<Task> = read_json(PATHC).expect("error while reading");
    let mut index: usize = tasks.len() - 1;
    let mut count: u8 = 0;

    let result = loop {
        if tasks[index].due == today {
            count += 1;
        } else {
            break count;
        }
        index -= 1;
    };
    result.to_string()
}
