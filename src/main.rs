mod todolist;
mod handler;

use crate::todolist::{ TodoList };
use std::io::{ self, Write };

fn print_stdin(){
    print!("\x1b[0m\x1b[2mType \"help\" to get all commandes\x1b[0m\n    What is your request ?\n\x1b[36m> ");
    io::stdout().flush().unwrap();
}

fn main() -> Result<(), io::Error> {
    // system values
    let mut restart: &mut bool = &mut true;
    let stdin = io::stdin();

    // initialize TodoList
    let mut todo_list = TodoList::new();

    while *restart {
        print_stdin();
        let mut input: String = String::new();
        match stdin.read_line(&mut input) {
            Ok(_) => handler::find_reference(String::from(input.trim()), &mut restart, &mut todo_list),
            _ => println!("\n\x1b[31mOh wait, i can't found any reference for this request...\n    --> Command not found\x1b[0m\n")
        };
    }
    println!("  \x1b[34mSee you soon !!\x1b[0m");
    Ok(())
}
