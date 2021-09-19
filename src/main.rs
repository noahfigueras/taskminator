use std::io::prelude::*;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();    

    if args[1] == "add" {
        add_todo(&args[2]);
    }
    println!("{:?}", args);
}

fn add_todo(task: &str) -> std::io::Result<()> {
    let mut file = File::open("./db/pending.json")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    
    //Push new task
    let fmt_task = format!("['task': '{}'],\n}}", task);
    content = content.replace("}", &fmt_task);
    println!("{}", content);
    Ok(())
}
