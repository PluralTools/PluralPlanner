use orbtk::prelude::*;

use crate::{
    base_state::BaseState,
    data::{TaskList, TaskOverview},
    keys::*,
    overview_view::OverviewView,
    task_view::TaskView,
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
        OverviewView::task_overview_mut(&mut ctx.widget()).push(TaskList::new("New entry"));

        self.adjust_count(ctx);
        self.save(registry, ctx);

        let index = OverviewView::task_overview_ref(&ctx.widget()).len() - 1;
        // todo select new entry

        self.open_task_list(ctx, index);
    }

    // Adjusts the task list count.
    fn adjust_count(&self, ctx: &mut Context) {
        let count = OverviewView::task_overview_ref(&ctx.widget()).len();
        OverviewView::count_set(&mut ctx.widget(), count);
        ListView::request_update_set(&mut ctx.get_widget(self.list_view), true);
    }

    // opens a task list.
    fn open_task_list(&self, ctx: &mut Context, index: usize) {
        TaskView::list_index_set(&mut ctx.get_widget(self.task_view), Some(index));
        let master_detail = *OverviewView::master_detail_ref(&ctx.widget());
        MasterDetail::show_detail(ctx, master_detail.into());
    }
}

impl State for OverviewState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.list_view = ctx.child(ID_OVERVIEW_LIST_VIEW).entity();
        self.task_view = (*ctx.widget().get::<u32>("task_view")).into();

        if let Ok(tasks) = registry
            .get::<Settings>("settings")
            .load::<TaskOverview>(PROP_TASK_OVERVIEW)
        {
            OverviewView::task_overview_set(&mut ctx.widget(), tasks);
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
