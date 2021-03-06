use orbtk::prelude::*;

use crate::{
    base_state::BaseState,
    data::{TaskList, TaskOverview},
    keys::*,
};

/// Actions that can execute on the overview.
#[derive(Debug, Copy, Clone)]
pub enum Action {
    NewEntry,
    OpenTaskList(usize),
}

/// Handles the requests of the `OverviewView`.
#[derive(Default, AsAny)]
pub struct OverviewState {
    action: Option<Action>,
    task_view: Entity,
    list_view: Entity,
}

impl BaseState for OverviewState {}

impl OverviewState {
    /// Sets a new action.
    pub fn action(&mut self, action: Action) {
        self.action = action.into();
    }

    // news a new task list.
    fn new_entry(&self, registry: &mut Registry, ctx: &mut Context) {
        ctx.widget()
            .get_mut::<TaskOverview>(PROP_TASK_OVERVIEW)
            .push(TaskList::new("New entry"));
        self.adjust_count(ctx);
        self.save(registry, ctx);

        let index = ctx.widget().get::<TaskOverview>(PROP_TASK_OVERVIEW).len() - 1;
        ctx.get_widget(self.task_view).set("create", true);
        self.open_task_list(ctx, index);
    }

    // Adjusts the task list count.
    fn adjust_count(&self, ctx: &mut Context) {
        let count = ctx.widget().get::<TaskOverview>(PROP_TASK_OVERVIEW).len();
        ctx.widget().set("count", count);
    }

    // opens a task list.
    fn open_task_list(&self, ctx: &mut Context, index: usize) {
        ctx.get_widget(self.task_view)
            .set("list_index", Some(index));
        self.navigate(self.task_view, ctx);
    }
}

impl State for OverviewState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.list_view = ctx
            .entity_of_child(ID_OVERVIEW_ITEMS_WIDGET)
            .expect("OverviewState.init: Items widget child could not be found.");
        self.task_view = (*ctx.widget().get::<u32>("task_view")).into();

        if let Ok(tasks) = registry
            .get::<Settings>("settings")
            .load::<TaskOverview>(PROP_TASK_OVERVIEW)
        {
            ctx.widget().set(PROP_TASK_OVERVIEW, tasks);
        }

        self.adjust_count(ctx);
    }

    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::NewEntry => {
                    self.new_entry(registry, ctx);
                }

                Action::OpenTaskList(index) => {
                    self.open_task_list(ctx, index);
                }
            }
        }

        self.action = None;
    }
}
