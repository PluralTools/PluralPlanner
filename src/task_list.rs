use orbtk::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,

    pub selected: bool,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub list: Vec<Task>,
}

impl TaskList {
    pub fn push(&mut self, task: Task) {
        self.list.push(task);
    }

    pub fn insert_front(&mut self, task: Task) {
        self.list.insert(0, task);
    }

    pub fn remove(&mut self, index: usize) -> Task {
        self.list.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&Task> {
        self.list.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Task> {
        self.list.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

into_property_source!(TaskList);
