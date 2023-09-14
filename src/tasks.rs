use std::cell::RefCell;
use std::collections::HashMap;
use std::process;

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::tasks::task::Task;

pub mod io;
pub mod task;

macro_rules! feedback {
    ($hash: expr, $index: ident) => {
        let entry = $hash.get_key_value(&$index).unwrap();

        println!("{} - {}", entry.0, entry.1)
    };
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Tasks {
    tasks: RefCell<HashMap<usize, Task>>,
}

trait FirstAvailableKey {
    fn available_index(&self) -> usize;
}

impl FirstAvailableKey for HashMap<usize, Task> {
    fn available_index(&self) -> usize {
        let mut index = 0;

        while self.contains_key(&index) {
            index += 1;
        }

        index
    }
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks {
            tasks: RefCell::new(HashMap::new()),
        }
    }

    pub fn add(&self, name: &str, description: &Option<String>) -> () {
        let description = match description {
            Some(desc) => desc,
            None => "",
        };

        self.tasks.borrow_mut().insert(
            self.tasks.borrow().available_index(),
            Task::new(name.to_string(), description.to_string()),
        );

        let index = self.tasks.borrow().len() - 1;

        let borrow = self.tasks.borrow();
        feedback!(borrow, index);
    }

    // pub fn modify(&self, index: usize, name: Option<&str>, description: Option<&str>) -> () {
    //     self.valid_index(index);
    //
    //     let old_task = self.tasks.borrow().get(&index).unwrap();
    //
    //     let task = Task::new(
    //         match name {
    //             Some(name) => name.to_string(),
    //             None => old_task.name.clone(),
    //         },
    //         match description {
    //             Some(description) => description.to_string(),
    //             None => old_task.description.clone(),
    //         },
    //     );
    //
    //     self.tasks.borrow_mut().insert(index, task);
    //
    //     let borrow = self.tasks.borrow();
    //     feedback!(borrow, index);
    // }

    pub fn remove(&self, index: usize) -> () {
        self.valid_index(index);

        println!(
            "{index} - {}",
            self.tasks.borrow().get(&index).unwrap().to_string_short()
        );
        self.tasks.borrow_mut().remove_entry(&index);
    }

    pub fn toggle_completed(&self, index: usize) -> () {
        self.valid_index(index);

        self.tasks.borrow().get(&index).unwrap().toggle_completed();
        println!(
            "{index} - {}",
            self.tasks.borrow().get(&index).unwrap().to_string_short()
        );
    }

    pub fn list_long(&self) -> () {
        for (number, task) in self.tasks.borrow().iter().enumerate() {
            println!("TASK NUMBER : {number}");
            println!("{}", task.1.to_string())
        }
    }

    pub fn list_short(&self) -> () {
        for (number, task) in self.tasks.borrow().iter().enumerate() {
            println!("Task : {number} - {}", task.1.to_string_short());
        }
    }

    pub fn len(&self) -> usize {
        self.tasks.borrow().len()
    }

    fn valid_index(&self, index: usize) -> () {
        if !self.tasks.borrow().contains_key(&index) {
            println!("{} : Not found.", "ERROR".red());
            process::exit(1);
        }
    }
}
