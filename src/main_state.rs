use orbtk::prelude::*;

use crate::TaskList;

pub enum Action {
    AddEntry(),
}

#[derive(Default, AsAny)]
pub struct MainState {}

impl MainState {
    fn adjust_count(&self, ctx: &mut Context) {
        let count = ctx.widget().get::<TaskList>("tasks").len();
        ctx.widget().set("task_count", count);
    }
}

impl State for MainState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Ok(tasks) = registry
            .get::<Settings>("settings")
            .load::<TaskList>("tasks")
        {
            ctx.widget().set("tasks", tasks);
        }

        self.adjust_count(ctx);
    }
}
