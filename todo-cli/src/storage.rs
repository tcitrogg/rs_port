use std::{error::Error, fs};

use crate::todo::Todo;

static DATABASE_PATH: &str = "TODOS.json";

pub fn load() -> Vec<Todo> {
    let json = match fs::read_to_string(DATABASE_PATH) {
        Ok(contents) => contents,
        Err(_) => return Vec::new(),
    };

    serde_json::from_str(&json).unwrap_or_default()
}

pub fn save(todos: &Vec<Todo>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(todos)?;
    fs::write(DATABASE_PATH, json)?;
    Ok(())
}
