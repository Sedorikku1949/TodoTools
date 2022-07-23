mod save;

use std::collections::HashMap;
use std::io;
#[allow(unused_imports)]
use std::io::{ Read, Write };
use std::process::exit;
use signal_hook::{ consts::SIGINT, iterator::Signals };
use std::{ error::Error, thread };
use std::ops::Deref;
use crate::save::TodoItem;

pub struct TodoList {
    map: HashMap<String, TodoItem>
}

impl TodoList {
    fn insert(&mut self, key: String) -> Option<TodoItem> {
        self.map.insert(key, TodoItem { details: String::new(), date: String::from("0"), done: false })
    }
    fn save(&self) -> Result<(), io::Error> {
        let mut content = String::new();
        for (k, _) in &self.map {
            let v = self.map.get(k).expect("TODO: cannot find key to save");
            let record = format!("{name},{details},{date},{done}\n", name = k, details = v.details, date = v.date, done = v.done);
            content.push_str(&record);
        }
        std::fs::write("todo.lmay", content).expect("TODO: Cannot write save file");
        Ok(())
    }
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(v.done = true),
            None => None,
        }
    }
    fn set_detail(&mut self, key: &String, detail: String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(v.details = detail),
            None => None,
        }
    }
    fn undone(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key){
            Some(v) => Some(v.done = false),
            None => None,
        }
    }
    fn remove(&mut self, key: &String) -> Option<TodoItem> {
        self.map.remove(key)
    }
}

fn main_loop_call() {
    let mut action: String = String::new();
    let mut item = String::new();
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    if args.len() < 1 { println!("You need to provide an action !") }
    else {
        for arg in args {
            if (arg == "add" || arg == "done" || arg == "complete" || arg == "remove" || arg == "list") && action.len() < 1 {
                action = arg
            } else { item.push_str(&*format!("{} ", &arg)) }
        }
        item = item.trim().parse().unwrap();
        //println!("action: {action}");
        //println!("item: {item}");
        //let action = std::env::args().nth(0);
        let todo = save::load().expect("TODO: cannot load save file"); //TodoList::new().expect("Initialisation of db failed");
        if action == "add" {
            add(todo, item)
        } else if action == "complete" {
            complete(todo, item);
        } else if action == "list" {
            list(todo)
        }  else {
            println!("i don't understand")
        }
    }
}

fn add(mut todo: TodoList, item: String){
    todo.insert(item);
    match todo.save() {
        Ok(_) => println!("todo list saved"),
        Err(why) => println!("An error occurred: {}", why)
    }
}
fn complete(mut todo: TodoList, item: String){
    match todo.complete(&item) {
        None => println!("\"{}\" is not present in your todo list !", item),
        Some(_) => match todo.save() {
            Ok(_) => println!("todo list saved"),
            Err(why) => println!("An error occurred: {}", why)
        }
    }
}
fn list(todo: TodoList){
    let mut size = 0;
    for (k, v) in todo.map {
        if v.done { println!("\x1b[32m✅\x1b[0m -- \x1b[32m{}\x1b[0m", k) } else { println!("\x1b[31m❌\x1b[0m -- \x1b[31m{}\x1b[0m", k) };
        size += 1;
    }
    println!("\n     {} elements in the Todo List", size)
}
fn print_help(){
    print!("\n        <-- Help -->\n\n");
    print!("\x1b[34madd\x1b[0m       \x1b[33m<item>\x1b[0m            -- \x1b[36mAdd an item to the todo list\x1b[0m\n");
    print!("\x1b[34mremove\x1b[0m    \x1b[33m<item>\x1b[0m            -- \x1b[36mRemove an item to the todo list\x1b[0m\n");
    print!("\x1b[34mcomplete\x1b[0m  \x1b[33m<item>\x1b[0m            -- \x1b[36mSpecify that this specific item is done\x1b[0m\n");
    print!("\x1b[34mundone\x1b[0m    \x1b[33m<item>\x1b[0m            -- \x1b[36mSpecify that this specific item is to do\x1b[0m\n");
    print!("\x1b[34mlist\x1b[0m                        -- \x1b[36mList all items and if they are done\x1b[0m\n");
    print!("\x1b[34mdetail\x1b[0m    \x1b[33m<item>\x1b[0m  \x1b[33m<detail>\x1b[0m  -- \x1b[36mDefine a detail for a specifid item\x1b[0m\n");
    print!("\x1b[33mclear\x1b[0m                       -- \x1b[33mClear the todo list\x1b[0m\n");
    print!("\x1b[33mreload\x1b[0m                      -- \x1b[33mReload the todo list from the save file\x1b[0m\n");
    print!("\x1b[31mstop\x1b[0m                        -- \x1b[31mStop the CLI\x1b[0m\n");
    io::stdout().flush().unwrap();
}

fn format_args(data: String) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    let mut is_string: bool = false;
    let mut actual: String = String::new();
    for chr in data.chars() {
        if (chr == '"' || chr == '\'') && !is_string {
            // string start
            actual = String::new();
            is_string = true;
        } else if (chr == '"' || chr == '\'') && is_string {
            // string end
            is_string = false;
            let _ = &content.push((*actual).parse().unwrap());
            actual = String::new();
        } else if (chr == ' ' || chr == '\n' || chr == '\t') && !is_string && actual.len() > 0 {
            // args end
            let _ = &content.push((*actual).parse().unwrap());
            actual = String::new();
        } else {
            actual.push_str(&(String::from(chr)));
        }
    };

    if actual.len() > 0 && !is_string { let _ = &content.push((*actual).parse().unwrap()); }

    content
}

fn main() -> Result<(), Box<dyn Error>> {
    // signal handler
    let mut signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for _sig in signals.forever() {
            println!("\n      \x1b[34mBye!\x1b[0m");
            exit(0);
        }
    });

    // CLI
    if std::env::args().len() < 2 {
        let mut restart: bool = true;

        //let mut todo = TodoList::new().expect("Initialisation of db failed");
        let mut todo = save::load().expect("TODO: cannot load save file");




        while restart {
            println!("\n\x1b[2mType help to get the command list\x1b[0m");
            println!("      What do you want to do?");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut content = String::new();
            io::stdin().read_line(&mut content).expect("TODO: cannot read line");
            let mut args = content.split_whitespace();
            let args_size = vec!(&args).len();
            if args_size < 1 {
                println!("I don't understand !")
            } else {
                let action = args.nth(0).expect("Action is required");
                if action == "stop" {
                    restart = false;
                } else if action == "help" {
                    print_help()
                }
                else if action == "add" {
                    if args_size < 1 {
                        println!("\nCannot add item to the todo list if you don't provide it!")
                    } else {
                        let mut item_content = String::new();
                        for s in args { item_content.push_str(&*format!("{} ", String::from(s))) };
                        item_content = item_content.trim().parse().unwrap();
                        let item_args = format_args(item_content);
                        let raw_item = item_args.get(0);
                        let item = raw_item.as_deref().unwrap();

                        let _ = &todo.insert(String::from(item));
                        match &todo.save() {
                            Ok(_) => println!("\nThe item \x1b[34m{}\x1b[0m has been added to the todo list", String::from(item)),
                            Err(why) => println!("An error occurred: {}", why)
                        }
                    }
                } else if action == "list" {
                    println!("Sending the list...\n");
                    let mut size = 0;
                    for (k, v) in &todo.map {
                        if v.done {
                            if v.details.len() > 0 { println!("\x1b[32m✅\x1b[0m -- \x1b[32m{}\x1b[0m -> \x1b[2m{}\x1b[0m", k, v.details) }
                            else { println!("\x1b[32m✅\x1b[0m -- \x1b[32m{}\x1b[0m", k) }
                        } else {
                            if v.details.len() > 0 { println!("\x1b[31m❌\x1b[0m -- \x1b[31m{}\x1b[0m -> \x1b[2m{}\x1b[0m", k, v.details) }
                            else { println!("\x1b[31m❌\x1b[0m -- \x1b[31m{}\x1b[0m", k) }
                        };
                        size += 1;
                    }
                    println!("\n     {} elements in the Todo List", size)
                } else if action == "complete" || action == "done" {
                    let mut item_content = String::new();
                    for s in args { item_content.push_str(&*format!("{} ", String::from(s))) };
                    item_content = item_content.trim().parse().unwrap();
                    let item_args = format_args(item_content);
                    let raw_item = item_args.get(0);
                    let item = raw_item.as_deref().unwrap();


                    match &todo.complete(&item) {
                        None => println!("\"{}\" is not present in your todo list !", &item),
                        Some(_) => match todo.save() {
                            Ok(_) => println!("\nThe task \x1b[34m{}\x1b[0m was marked has done", &item),
                            Err(why) => println!("An error occurred: {}", why)
                        }
                    }
                } else if action == "undone" {
                    let mut item_content = String::new();
                    for s in args { item_content.push_str(&*format!("{} ", String::from(s))) };
                    item_content = item_content.trim().parse().unwrap();
                    let item_args = format_args(item_content);
                    let raw_item = item_args.get(0);
                    let item = raw_item.as_deref().unwrap();


                    match &todo.undone(&item) {
                        Some(_) => match todo.save(){
                            Ok(_) => println!("\nThe item \x1b[34m{currentItem}\x1b[0m is now marked has to do", currentItem = item),
                            Err(why) => println!("An error occurred: {}", why)
                        }
                        None => println!("\nThe item \x1b[34m{currentItem}\x1b[0m doesn't exist", currentItem = item)
                    }
                } else if action == "remove" {
                    let mut item_content = String::new();
                    for s in args { item_content.push_str(&*format!("{} ", String::from(s))) };
                    item_content = item_content.trim().parse().unwrap();
                    let item_args = format_args(item_content);
                    let raw_item = item_args.get(0);
                    let item = raw_item.as_deref().unwrap();


                    match todo.remove(&item) {
                        Some(_) => match todo.save(){
                            Ok(_) => println!("\nThe item \x1b[34m{currentItem}\x1b[0m has been removed from to todo list", currentItem = item),
                            Err(why) => println!("An error occurred: {}", why)
                        }
                        None => println!("\nThe item \x1b[34m{currentItem}\x1b[0m doesn't exist", currentItem = item)
                    }
                } else if action == "clear" {
                    if todo.map.len() > 0 {
                        let mut i: i32 = 0_i32;
                        for (k, _v) in &todo.map {
                            println!("The item \x1b[34m{currentItem}\x1b[0m has been removed from to todo list", currentItem = k);
                            i += 1;
                        }
                        todo.map = HashMap::new();
                        match todo.save() {
                            Ok(_) => println!("\n      {i} items has been removed from the todo list !"),
                            Err(why) => println!("An error occurred: {}", why)
                        }
                    } else { println!("\n     \x1b[31mThe todo list is already empy.\x1b[0m") }
                } else if action == "reload" {
                    todo = save::load().expect("TODO: cannot load save file"); // TodoList::new().expect("Fail to load file");
                    println!("\n     \x1b[32mThe todo list has been reloaded !\x1b[0m")
                } else if action == "detail" {
                    let mut item_content = String::new();
                    for s in args { item_content.push_str(&*format!("{} ", String::from(s))) };
                    item_content = item_content.trim().parse().unwrap();
                    let item_args = format_args(item_content);

                    let raw_item = item_args.get(0);
                    let item = raw_item.as_deref().unwrap();

                    let raw_detail = item_args.get(1).expect("TODO: cannot find raw_detail");
                    let detail: String = String::from(raw_detail.deref());

                    match todo.set_detail(item, detail) {
                        Some(_) => match todo.save(){
                            Ok(_) => println!("The detail of the item \x1b[34m{}\x1b[0m has been updated !", item),
                            Err(why) => println!("An error occurred: {}", why)
                        }
                        None => println!("\nThe item \x1b[34m{currentItem}\x1b[0m doesn't exist", currentItem = item)
                    }
                }
            }

        }
        println!("\n      \x1b[34mBye!\x1b[0m");
        Ok(())
    } else {
        main_loop_call();
        Ok(())
    }
}