use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub status: bool,
}

//impl Todo {
pub fn add_todo(todos: &mut Vec<Todo>, title: String) {
    todos.push(Todo {
        id: todos.len() + 1,
        title,
        status: false,
    });
}

pub fn delete_todo(todos: &mut Vec<Todo>, id: usize) {
    if id <= todos.len() - 1 {
        todos.remove(id);
    }
}

pub fn mark_todo(todos: &mut Vec<Todo>, id: usize) {
    if id <= todos.len() - 1 {
        todos[id].status = !todos[id].status
    }
}

pub fn clear(todos: &mut Vec<Todo>) {
    *todos = Vec::new();
}
//}
