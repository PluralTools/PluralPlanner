use orbtk::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,

    pub selected: bool,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub list: Vec<Task>
}

impl TaskList {
    pub fn push(&mut self, task: Task) {
        self.list.push(task);
    }

    pub fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }

    pub fn get(&self, index: usize) -> Option<&Task> {
        self.list.get(index)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

into_property_source!(TaskList);
