use std::process;

use crate::libs::tasks::task::Task;
use colored::Colorize;
use serde::{Deserialize, Serialize};

pub mod io;
pub mod task;

/// A module to handle the tasks of the application.
/// It contains functions to add, modify, remove, toggle and list tasks.
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Tasks {
    tasks: Vec<Task>,
}

/// `Tasks` is an implementation of the tasks.
impl Tasks {
    /// `new` creates a new instance of `Tasks`.
    pub fn new() -> Tasks {
        Tasks { tasks: vec![] }
    }

    /// `add` adds a new task to the tasks.
    pub fn add(&mut self, name: &str, description: &Option<String>) {
        let description = match description {
            Some(desc) => desc,
            None => "",
        };

        let task = Task::new(name.to_string(), description.to_string());

        println!("{}", &task);

        self.tasks.push(task);
    }

    /// `remove` removes a task from the tasks.
    pub fn remove(&mut self, index: usize) {
        let tasks = self.get(index);

        println!("{index} - {}", tasks.to_string_short());
        self.tasks.remove(index);
    }

    /// `toggle_completed` toggles the completed status of a task.
    pub fn toggle_completed(&self, index: usize) {
        let tasks = self.get(index);

        tasks.toggle_completed();

        println!("{index} - {}", tasks.to_string_short());
    }

    /// `list_long` lists the tasks in a long format.
    pub fn list_long(&self) {
        for (number, task) in self.tasks.iter().enumerate() {
            println!("TASK NUMBER : {number}");
            println!("{}", task.to_string())
        }
    }

    /// `list_short` lists the tasks in a short format.
    pub fn list_short(&self) {
        for (number, task) in self.tasks.iter().enumerate() {
            println!("Task : {number} - {}", task.to_string_short());
        }
    }

    /// `len` returns the length of the tasks.
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Checks if the tasks list is empty.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Get a Task by Index, exit process if not found.
    pub fn get(&self, index: usize) -> &Task {
        match self.tasks.get(index) {
            Some(s) => s,
            None => {
                eprintln!("{} : Not found.", "ERROR".red());
                process::exit(1);
            }
        }
    }
}
