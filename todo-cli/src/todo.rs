use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub status: bool,
}

static RESET: &str = "\x1b[0m";

pub fn list(todos: &Vec<Todo>) {
    println!("\x1b[35m(TODO List){RESET}");
    if todos.len() > 0 {
        for each_todo in todos {
            if each_todo.status {
                println!("\x1b[32m[x] {}: {}{RESET}", each_todo.id, each_todo.title);
                continue;
            }
            println!("[ ] {}: {}", each_todo.id, each_todo.title);
        }
        return;
    }
    println!("\x1b[33m(Empty) Add a task & it'll show up here.{RESET}");
}

//impl Todo {
pub fn add_todo(todos: &mut Vec<Todo>, title: String) {
    todos.push(Todo {
        id: todos.len() + 1,
        title,
        status: false,
    });
}

pub fn delete_todo(todos: &mut Vec<Todo>, id: usize) -> Result<(), String> {
    if id <= todos.len() && id > 0 {
        todos.remove(id - 1);
        return Ok(());
    }

    Err("Invalid ID".to_string())
}

pub fn mark_todo(todos: &mut Vec<Todo>, id: usize) -> Result<bool, String> {
    if id <= todos.len() && id > 0 {
        let new_id = id - 1;
        todos[new_id].status = !todos[new_id].status;
        return Ok(todos[new_id].status);
    }
    Err("Invalid ID".to_string())
}

pub fn clear(todos: &mut Vec<Todo>) {
    if todos.len() == 0 {
        return;
    }
    *todos = Vec::new();
}
//}
