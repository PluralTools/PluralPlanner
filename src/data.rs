use orbtk::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TaskOverview {
    pub task_lists: Vec<TaskList>,
}

impl TaskOverview {
    pub fn push(&mut self, task_list: TaskList) {
        self.task_lists.push(task_list);
    }

    pub fn insert_front(&mut self, task_list: TaskList) {
        self.task_lists.insert(0, task_list);
    }

    pub fn remove(&mut self, index: usize) -> TaskList {
        self.task_lists.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&TaskList> {
        self.task_lists.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut TaskList> {
        self.task_lists.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.task_lists.len()
    }

    pub fn is_empty(&self) -> bool {
        self.task_lists.is_empty()
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub text: String,

    pub selected: bool,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TaskList {
    pub title: String,

    pub list: Vec<Task>,
}

impl TaskList {
    pub fn new(title: impl Into<String>) -> Self {
        TaskList {
            title: title.into(),
            ..Default::default()
        }
    }

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

into_property_source!(TaskOverview);
into_property_source!(TaskList);
