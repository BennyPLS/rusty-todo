use fmt::Debug;
use std::cell::Cell;
use std::fmt;
use std::fmt::{Display, Formatter};

use indoc::indoc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub name: String,
    pub description: String,
    is_completed: Cell<bool>,
}

impl Task {
    pub fn new(name: String, description: String) -> Task {
        Task {
            name,
            description,
            is_completed: Cell::from(false),
        }
    }

    fn completed_symbol(&self) -> String {
        if self.is_completed.get() {
            String::from("✓")
        } else {
            String::from("✗")
        }
    }

    pub fn toggle_completed(&self) {
        self.is_completed.set(!self.is_completed.get());
    }

    pub fn get_is_completed(&self) -> bool {
        self.is_completed.get()
    }

    pub fn to_string_short(&self) -> String {
        format!("{} - {}", self.name, self.completed_symbol())
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            indoc! {"
            Task Name   : {}
            Description : {}
            Completed   : {}
            "},
            self.name,
            self.description,
            self.completed_symbol()
        )
    }
}
