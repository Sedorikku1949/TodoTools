#[allow(unused_imports)]
use std::io::{ Error, Read };
use crate::TodoList;

pub struct TodoItem {
    pub(crate) details: String,
    pub(crate) date: String,
    pub(crate) done: bool
}

impl TodoItem {

    fn new(content: &str) -> TodoItem {
        let all_variables = content.splitn(3, ",").collect::<Vec<&str>>();
        let mut actual_col = 0;
        let mut details = String::new();
        let mut date = String::new();
        let mut done = false;
        for c in all_variables {
            let line = &*c.replace(|s| s == '[' || s == ']', "");
            if actual_col == 0 {
                details = line.parse().unwrap();
            } else if actual_col == 1 {
                date = line.parse().unwrap();
            } else if actual_col == 2 {
                if line == "false" { done = false } else { done = true }
            }
            actual_col += 1;
        }

        TodoItem { details, date, done }
    }

}

pub fn load() -> Result<TodoList, Error> {
    let mut f = std::fs::OpenOptions::new().write(true).create(true).read(true).open("todo.lmay")?;
    let mut content = String::new();
    f.read_to_string(&mut content).expect("TODO: cannot read save");
    let map: TodoList = TodoList {
        map: content.lines()
            .map(|line| line.splitn(2, ",").collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), TodoItem::new(v)))
            .collect()
        };

    Ok(map)
}
