pub mod err;

pub use err::{ParseErr, ReadErr};
use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(Box::new(ReadErr { child_err: Box::new(e) })),
        };

        
        let parsed = match json::parse(&content) {
            Ok(p) => p,
            Err(e) => return Err(Box::new(ParseErr::Malformed(Box::new(e)))),
        };

        
        let tasks_val = &parsed["tasks"];
        if tasks_val.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        
        let mut tasks = Vec::new();
        for t in tasks_val.members() {
            let id = t["id"].as_u32().unwrap_or(0);
            let description = t["description"].as_str().unwrap_or("").to_string();
            let level = t["level"].as_u32().unwrap_or(0);
            tasks.push(Task { id, description, level });
        }

        let title = parsed["title"].as_str().unwrap_or("").to_string();

        Ok(TodoList { title, tasks })
    }
}