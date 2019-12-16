use orbtk::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,

    pub checked: bool,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub list: Vec<Task>
}

impl TaskList {
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
