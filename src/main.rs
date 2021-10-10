use std::env;
mod lib;
use lib::{
    list_tasks, 
    add_task,
    remove_task,
    completed_task,
    help
};

fn main() {
    let args: Vec<String> = env::args().collect();    

    match args.len() {
        1 => {
            list_tasks();
        } 
        _ => {
            match args[1].as_str() {
               "-a" => {
                    add_task(args[2..].to_vec());
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

