use fmt::Debug;
use std::cell::Cell;
use std::fmt;
use std::fmt::{Display, Formatter};

use indoc::indoc;
use serde::{Deserialize, Serialize};

/// A module to handle the tasks of the application.

/// `Task` is a struct that contains the information of a task.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub name: String,
    pub description: String,
    is_completed: Cell<bool>,
}

/// `Task` is an implementation of the task.
impl Task {
    /// `new` creates a new instance of `Task`.
    pub fn new(name: String, description: String) -> Task {
        Task {
            name,
            description,
            is_completed: Cell::from(false),
        }
    }

    /// `completed_symbol` returns the symbol of the completed state.
    fn completed_symbol(&self) -> String {
        if self.is_completed.get() {
            String::from("✓")
        } else {
            String::from("✗")
        }
    }

    /// `toggle_completed` toggles the state of completed for a task.
    pub fn toggle_completed(&self) {
        self.is_completed.set(!self.is_completed.get());
    }

    /// `get_is_completed` returns the state of completed for a task.
    pub fn get_is_completed(&self) -> bool {
        self.is_completed.get()
    }

    /// `to_string_short` returns a string with the name and the completed symbol.
    pub fn to_string_short(&self) -> String {
        format!("{} - {}", self.name, self.completed_symbol())
    }
}

/// `Display` is an implementation of the display for `Task`.
impl Display for Task {
    /// `fmt` formats the output of `Task`.
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
