use orbtk::prelude::*;

use crate::{TaskState};
// use crate::{Action, MainState, TaskList};


widget!(TaskView<TaskState> {

});

impl Template for TaskView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Task")
    }
}