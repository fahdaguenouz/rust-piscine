mod err;
use err::{ParseErr, ReadErr};
use std::error::Error;
use std::fs;
use std::result::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(file_path)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;
        
        if contents.trim().is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let todo_list: TodoList = serde_json::from_str(&contents)
            .map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>)?;
        
        Ok(todo_list)
    }
}
