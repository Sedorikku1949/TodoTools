use std::collections::HashMap;
use std::fs::{ OpenOptions };
use std::io::Read;

pub struct RegisteredItem {
  pub(crate) name: String,
  pub(crate) details: String,
  pub(crate) completed: bool
}

/// The TodoList manager
pub struct TodoList {
  pub(crate) list: HashMap<String, RegisteredItem>
}

impl RegisteredItem {
  fn new(raw: &String) -> RegisteredItem {
    let mut registered = RegisteredItem { name: "".to_string(), details: "".to_string(), completed: false };
    let raw_data = raw.splitn(3, "\",\"").collect::<Vec<&str>>();
    let mut actual: i8 = 0_i8;
    for elm in raw_data {
      if actual == 0 { registered.name = elm.replace("\"", ""); actual = actual + 1 }
      else if actual == 1 { registered.details = elm.replace("\"", ""); actual = actual + 1  }
      else { registered.completed = elm.replace("\"", "") == "true"; }
    }
    registered
  }
}

impl TodoList {

  /// Create a new TodoList
  pub(crate) fn new() -> TodoList {
    TodoList::load().unwrap()
  }

  /// Check if the TodoList have a specific item
  pub(crate) fn has(&self, reference: &String) -> bool {
    match self.list.get(&*reference) {
      Some(_) => true,
      _ => false
    }
  }

  /// add a new item to the todo list
  pub(crate) fn add(&mut self, name: String, details: String) -> &mut Self {
    self.list.insert((&*name).to_string(), RegisteredItem { name, details, completed: false });
    self.save();
    self
  }

  /// complete a item
  pub(crate) fn complete(&mut self, key: String) -> &mut Self {
    match self.list.get_mut(key.as_str()) {
      Some(v) => {
        v.completed = true;
        self.save();
        self
      },
      None => self,
    }
  }
  /// undone a item
  pub(crate) fn undone(&mut self, key: String) -> &mut Self {
    match self.list.get_mut(key.as_str()) {
      Some(v) => {
        v.completed = false;
        self.save();
        self
      },
      None => self,
    }
  }

  /// remove an item
  pub(crate) fn remove(&mut self, key: String) -> &mut Self {
    match self.has(&key) {
      true => {
        self.list.remove(&key);
        self.save();
        self
      },
      _ => self
    }
  }
  /// edit the details about an item
  pub(crate) fn details(&mut self, key: String, details: String) -> &mut Self {
    match self.list.get_mut(&key) {
      Some(v) => {
        v.details = details;
        self.save();
        self
      },
      _ => self
    }
  }
  /// get data from an item
  pub(crate) fn get(&mut self, key: &String) -> Result<&RegisteredItem, ()> {
    match self.list.get(key) {
      Some(v) => Ok(v),
      _ => Err(())
    }
  }

  /// load to do list from a file
  fn load() -> Result<TodoList, std::io::Error> {
    let mut f = OpenOptions::new().create(true).write(true).read(true).open("todo.lmay")?;
    let mut content = String::new();
    match f.read_to_string(&mut content) {
      Ok(_) => {
        let map = content.lines()
            .map(|line| line.splitn(2, ",").collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k).replace("\"", ""), RegisteredItem::new(&format!("{},{}", k, v).to_string())))
            .collect();
        Ok(TodoList { list: map })
      },
      _ => Ok(TodoList { list: HashMap::new() })
    }
  }

  /// save to do list in a file
  pub(crate) fn save(&self) -> &Self {
    let mut content = String::new();
    for (_k, v) in &self.list {
      if v.completed {
        let record = format!("\"{name}\",\"{details}\",\"true\"\n", name = v.name, details = v.details);
        content.push_str(&record);
      } else {
        let record = format!("\"{name}\",\"{details}\",\"false\"\n", name = v.name, details = v.details);
        content.push_str(&record);
      }
    }
    match std::fs::write("todo.lmay", content.trim()) {
      Ok(_) => self,
      _ => self
    }
  }
}
