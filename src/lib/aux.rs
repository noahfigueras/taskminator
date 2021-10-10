use crate::lib::{PATH, Task};
use std::fs::{File, write};
use serde_json::Result;

pub fn read_json() -> Result<Vec<Task>> {
    let file = File::open(PATH).expect("File not found");
    let j: Vec<Task> = serde_json::from_reader(file)?;
    Ok(j)
}

pub fn write_json(tasks: &Vec<Task>) -> Result<()> {
    let _json: String = serde_json::to_string(&tasks).expect("Error parsing to json");
    write(PATH, &_json).expect("Unable to write file");
    Ok(())
}
