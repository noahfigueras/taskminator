use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use ansi_term::{Style, Colour};
use crate::lib::{PATHP,Task};
use crate::lib::aux::{
    read_json,
    write_json,
    date_fmt,
    append_json,
    completed_count,
};


pub fn start(_index: &str) {
    let mut tasks: Vec<Task> = read_json(PATHP).expect("error while reading");
    let index: u8 = _index.parse::<u8>().unwrap();
    
    let mut i: u8 = 0;
    for mut task in &mut tasks {
        if i == index {
           task.status.active = true;   
           task.status.start = Local::now().format("%H:%M").to_string();
        }
        i=i+1;
    }
    write_json(&tasks, PATHP).expect("Unable to write file");
}

pub fn stop(_index: &str) {
    println!("Stop timer for this task!");
}
