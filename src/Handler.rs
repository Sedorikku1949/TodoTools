use std::io::{self, Write };
use std::str::SplitWhitespace;
use crate::TodoList;

pub fn find_reference(content: String, restart: &mut bool, todo_list: &mut TodoList){
    if content.len() < 1 {
        println!("\n\x1b[31mOh wait, i didn't find your request...\n    --> Command not found\x1b[0m\n")
    } else {
        let mut args: SplitWhitespace = content.trim().split_whitespace();
        let command = args.nth(0).unwrap();
        match command {
            "sclear" => clear_shell(),                                               // clear shell
            "list" | "ls" | "l" => list_todolist(&todo_list),                   // list all items in the list
            "add" | "new" => add_item(todo_list),                               // add a new item to the list
            "done" | "complete" => complete_item(todo_list),                    // mark a task as done
            "undone" | "uncomplete" | "todo" => uncomplete_item(todo_list),     // mark a task as to do
            "remove" | "delete" => remove_item(todo_list),                      // delete an item
            "get" | "read" => get_item(todo_list),                              // watch data about an item
            "detail" | "details" | "editd" => edit_detail(todo_list),           // edit details from an item
            "stop" | "exit" => { *restart = false; },                                // exit the CLI
            _ => println!("\n\x1b[31mOh wait, I could not find a reference for this request....\n    --> Command \"{}\" not found\x1b[0m\n", &content)
        }
    }
}

fn clear_shell(){
    print!("\x1B[2J\x1B[1;1H\x1b[0m");
    io::stdout().flush().unwrap();
}

fn list_todolist(todo: &TodoList){
    io::stdout().flush().unwrap();
    print!("\n");
    for (_k, v) in &todo.list {
        if v.completed { println!("\x1b[32m✅   {name}\x1b[0m", name = v.name) }
        else { println!("\x1b[31m❌   {name}\x1b[0m", name = v.name) }
    }

    println!("   {size} items in the todo list.\n", size = todo.list.len());
}

fn add_item(todo: &mut TodoList){
    let mut name: String = String::new();
    print!("\x1b[0m\n\x1b[2mWhat is the name of the new item ?\n\x1b[2m> ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            if name.len() < 1 { println!("\x1b[0m\n\x1b[31mInvalid response provided\n    --> Cannot create an item with a empty name\x1b[0m\n") }
            else if todo.has(&(&*name.trim()).to_string()) { println!("\x1b[0m\n\x1b[31mCannot create new item\n    --> The item already exist!\x1b[0m\n") }
            else {
                let mut detail: String = String::new();
                print!("\x1b[0m\n\x1b[2mWhat is the details of the new item ?\n    \x1b[0m\x1b[1;34m[i]️ Type enter to ignore this field.\x1b[0m\n\x1b[2m> ");
                io::stdout().flush().unwrap();
                match io::stdin().read_line(&mut detail) {
                    Ok(_) => {
                        todo.add((&*name.trim()).to_string(), detail.trim().to_string());
                        println!("\x1b[0m\n  \n\x1b[32m✅ The item \"{name}\" was successfully saved !\x1b[0m\n\n", name = &name.trim())
                    },
                    _ => println!("\n\x1b[31mAn error occured while waiting for the detail\n    --> Cannot read stdin channel\x1b[0m\n")
                }
            }
        }
        _ => println!("\n\x1b[31mAn error occured while waiting for the name\n    --> Cannot read stdin channel\x1b[0m\n")
    }
}

fn complete_item(todo: &mut TodoList){
    let mut name: String = String::new();
    print!("\x1b[0m\n\x1b[2mWhat is the name of the new item ?\n\x1b[2m> ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            if name.trim().len() < 1 { println!("\x1b[0m\n\x1b[31mInvalid response provided\n    --> Cannot mark a task as complete without a name\x1b[0m\n") }
            else if !todo.has(&(name.trim()).to_string()) { println!("\x1b[0m\n\x1b[31mInvalid name provided\n    --> Cannot mark a task as complete if she doesn't exist\x1b[0m\n") }
            else {
                todo.complete((&*name.trim()).to_string());
                println!("\x1b[0m\n  \n\x1b[32m✅ The item \"{name}\" was successfully marked has completed !\x1b[0m\n\n", name = &name.trim())
            }
        }
        _ => println!("\n\x1b[31mAn error occured while waiting for the name\n    --> Cannot read stdin channel\x1b[0m\n")
    }
}
fn uncomplete_item(todo: &mut TodoList){
    let mut name: String = String::new();
    print!("\x1b[0m\n\x1b[2mWhat is the name of the item ?\n\x1b[2m> ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            if name.trim().len() < 1 { println!("\x1b[0m\n\x1b[31mInvalid response provided\n    --> Cannot mark a task as todo without a name\x1b[0m\n") }
            else if !todo.has(&(name.trim()).to_string()) { println!("\x1b[0m\n\x1b[31mInvalid name provided\n    --> Cannot mark a task as todo if she doesn't exist\x1b[0m\n") }
            else {
                todo.undone((&*name.trim()).to_string());
                println!("\x1b[0m\n  \n\x1b[32m✅ The item \"{name}\" was successfully marked has to do !\x1b[0m\n\n", name = &name.trim())
            }
        }
        _ => println!("\n\x1b[31mAn error occured while waiting for the name\n    --> Cannot read stdin channel\x1b[0m\n")
    }
}

fn remove_item(todo: &mut TodoList){
    let mut name: String = String::new();
    print!("\x1b[0m\n\x1b[2mWhat is the name of the item ?\n\x1b[2m> ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            if name.trim().len() < 1 { println!("\x1b[0m\n\x1b[31mInvalid response provided\n    --> Cannot remove a item without a name\x1b[0m\n") }
            else if !todo.has(&(name.trim()).to_string()) { println!("\x1b[0m\n\x1b[31mInvalid name provided\n    --> Cannot delete an item if he doesn't exist\x1b[0m\n") }
            else {
                todo.remove((&*name.trim()).to_string());
                println!("\x1b[0m\n  \n\x1b[32m🗑✅ The item \"{name}\" was successfully deleted !\x1b[0m\n\n", name = &name.trim())
            }
        }
        _ => println!("\n\x1b[31mAn error occured while waiting for the name\n    --> Cannot read stdin channel\x1b[0m\n")
    }
}

fn edit_detail(todo: &mut TodoList){
    let mut name: String = String::new();
    print!("\x1b[0m\n\x1b[2mWhat is the name of the new item ?\n\x1b[2m> ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            if name.len() < 1 { println!("\x1b[0m\n\x1b[31mInvalid response provided\n    --> Cannot edit an item detail with a empty name\x1b[0m\n") }
            else if !todo.has(&(&*name.trim()).to_string()) { println!("\x1b[0m\n\x1b[31mCannot edit item\n    --> The item doesn't exist!\x1b[0m\n") }
            else {
                let mut detail: String = String::new();
                print!("\x1b[0m\n\x1b[2mWhat is the details of the item ?\n> ");
                io::stdout().flush().unwrap();
                match io::stdin().read_line(&mut detail) {
                    Ok(_) => {
                        todo.details((&*name.trim()).to_string(), detail);
                        println!("\x1b[0m\n  \n\x1b[32m✅ The details of the item \"{name}\" was successfully saved !\x1b[0m\n\n", name = &name.trim())
                    },
                    _ => println!("\n\x1b[31mAn error occured while waiting for the detail\n    --> Cannot read stdin channel\x1b[0m\n")
                }
            }
        }
        _ => println!("\n\x1b[31mAn error occured while waiting for the name\n    --> Cannot read stdin channel\x1b[0m\n")
    }
}

fn get_item(todo: &mut TodoList){
    let mut name: String = String::new();
    print!("\x1b[0m\n\x1b[2mWhat is the name of the new item ?\n\x1b[2m> ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            if name.trim().len() < 1 { println!("\x1b[0m\n\x1b[31mInvalid response provided\n    --> Cannot mark a task as complete without a name\x1b[0m\n") }
            else {
                match todo.get(&(name.trim()).to_string()) {
                    Ok(v) => {
                        let detail: String;
                        if v.details.trim().len() > 0 { detail = ("\x1b[32m".to_owned() + v.details.trim() + "\x1b[0m").to_string() }
                        else { detail = "\x1b[2mNo details\x1b[0m".to_string() }
                        if v.completed { print!("\x1b[0m\nName: \x1b[32m{name}\x1b[0m\nDetails: {details} \n\nCompleted: \x1b[32m✅  ​\n\n\x1b[0m", name = v.name, details = detail) }
                        else { print!("\x1b[0m\nName: \x1b[32m{name}\x1b[0m\nDetails: {details} \n\nCompleted: \x1b[31m❌  ​\n\n\x1b[0m", name = v.name, details = detail) }
                    },
                    _ => println!("\n\x1b[31mUnknown item\n    --> The item was not found\x1b[0m\n")
                }
            }
        }
        _ => println!("\n\x1b[31mAn error occured while waiting for the name\n    --> Cannot read stdin channel\x1b[0m\n")
    }
}
