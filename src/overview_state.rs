use orbtk::prelude::*;

use crate::{
    base_state::BaseState,
    data::{TaskList, TaskOverview},
    keys::*,
};

/// Actions that can execute on the overview.
#[derive(Debug, Copy, Clone)]
pub enum Action {
    InputTextChanged(Entity),
    NewEntry(Entity),
    RemoveEntry(usize),
    OpenTaskList,
}

/// Handles the requests of the `OverviewView`.
#[derive(Default, AsAny)]
pub struct OverviewState {
    action: Option<Action>,
    add_button: Entity,
    task_view: Entity,
    list_view: Entity,
    text_box: Entity,
}

impl BaseState for OverviewState {}

impl OverviewState {
    /// Sets a new action.
    pub fn action(&mut self, action: Action) {
        self.action = action.into();
    }

    // news a new task list.
    fn new_entry(&self, text: String, registry: &mut Registry, ctx: &mut Context) {
        ctx.widget()
            .get_mut::<TaskOverview>(PROP_TASK_OVERVIEW)
            .push(TaskList::new(text));
        self.adjust_count(ctx);
        self.save(registry, ctx);
    }

    // removes a task list.
    fn remove_entry(&self, index: usize, registry: &mut Registry, ctx: &mut Context) {
        ctx.widget()
            .get_mut::<TaskOverview>(PROP_TASK_OVERVIEW)
            .remove(index);
        self.adjust_count(ctx);
        self.save(registry, ctx);
    }

    // If input text is empty the add button is disabled, otherwise enabled.
    fn adjust_add_button_enabled(&self, text_box: Entity, ctx: &mut Context) {
        if ctx.get_widget(text_box).get::<String16>("text").is_empty() {
            ctx.get_widget(self.add_button).set("enabled", false);
        } else {
            ctx.get_widget(self.add_button).set("enabled", true);
        }

        ctx.get_widget(self.add_button).update(true);
    }

    // Adjusts the task list count.
    fn adjust_count(&self, ctx: &mut Context) {
        let count = ctx.widget().get::<TaskOverview>(PROP_TASK_OVERVIEW).len();
        ctx.widget().set(PROP_COUNT, count);
    }

    // opens a task list.
    fn open_task_list(&self, ctx: &mut Context) {
        let mut index = None;
        for i in &ctx
            .get_widget(self.list_view)
            .get::<SelectedIndices>("selected_indices")
            .0
        {
            index = Some(*i);

            // single selection
            break;
        }

        ctx.get_widget(self.text_box)
            .set("text", String16::from(""));
        ctx.get_widget(self.task_view).set("list_index", index);
        self.navigate(self.task_view, ctx);
    }
}

impl State for OverviewState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.text_box = ctx
            .entity_of_child(ID_OVERVIEW_TEXT_BOX)
            .expect("OverviewState.init: Text box child could not be found.");
        self.add_button = ctx
            .entity_of_child(ID_OVERVIEW_ADD_BUTTON)
            .expect("OverviewState.init: Add button child could not be found.");
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
                Action::InputTextChanged(text_box) => {
                    self.adjust_add_button_enabled(text_box, ctx);
                }
                Action::NewEntry(entity) => {
                    if let Some(text) = self.fetch_text(ctx, entity) {
                        self.new_entry(text, registry, ctx);
                    }
                }
                Action::RemoveEntry(index) => {
                    self.remove_entry(index, registry, ctx);
                }

                Action::OpenTaskList => {
                    self.open_task_list(ctx);
                }
            }
        }

        self.action = None;
    }
}
