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
    CreateEntry(Entity),
    RemoveEntry(usize),
    TextChanged(Entity, usize),
    EditEntry(Entity),
    RemoveFocus(Entity),
    OpenTaskList(Entity),
}

/// Handles the requests of the `OverviewView`.
#[derive(Default, AsAny)]
pub struct OverviewState {
    action: Option<Action>,
    add_button: Entity,
    task_view: Entity,
}

impl BaseState for OverviewState {}

impl OverviewState {
    /// Sets a new action.
    pub fn action(&mut self, action: Action) {
        self.action = action.into();
    }

    // Creates a new task list.
    fn create_entry(&self, text: String, ctx: &mut Context) {
        ctx.widget()
            .get_mut::<TaskOverview>(PROP_TASK_OVERVIEW)
            .push(TaskList::new(text));
        self.adjust_count(ctx);
    }

    // removes a task list.
    fn remove_entry(&self, index: usize, ctx: &mut Context) {
        ctx.widget()
            .get_mut::<TaskOverview>(PROP_TASK_OVERVIEW)
            .remove(index);
        self.adjust_count(ctx);
    }

    // If input text is empty the add button is disabled, otherwise enabled.
    fn adjust_add_button_enabled(&self, text_box: Entity, ctx: &mut Context) {
        if ctx.get_widget(text_box).get::<String16>("text").is_empty() {
            ctx.get_widget(self.add_button).set("enabled", false);
        } else {
            ctx.get_widget(self.add_button).set("enabled", true);
        }

        ctx.get_widget(self.add_button).update_theme_by_state(true);
    }

    // Adjusts the task list count.
    fn adjust_count(&self, ctx: &mut Context) {
        let count = ctx.widget().get::<TaskOverview>(PROP_TASK_OVERVIEW).len();
        ctx.widget().set(PROP_COUNT, count);
    }

    // Save the data.
    fn save(&self, registry: &mut Registry, ctx: &mut Context) {
        registry
            .get::<Settings>("settings")
            .save(
                PROP_TASK_OVERVIEW,
                ctx.widget().get::<TaskOverview>(PROP_TASK_OVERVIEW),
            )
            .unwrap();
    }

    // opens a task list.
    fn open_task_list(&self, entity: Entity, ctx: &mut Context) {
        self.navigate(self.task_view, ctx);
    }
}

impl State for OverviewState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.add_button = ctx
            .entity_of_child(ID_OVERVIEW_ADD_BUTTON)
            .expect("OverviewState.init: Add button child could not be found.");
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
                Action::CreateEntry(entity) => {
                    if let Some(text) = self.fetch_text(ctx, entity) {
                        self.create_entry(text, ctx);
                        self.save(registry, ctx);
                    }
                }
                Action::RemoveEntry(index) => {
                    self.remove_entry(index, ctx);
                    self.save(registry, ctx);
                }
                Action::TextChanged(entity, index) => {
                    let text: String16 = ctx.get_widget(entity).clone("text");

                    if let Some(overview) = ctx
                        .widget()
                        .get_mut::<TaskOverview>("task_overview")
                        .get_mut(index)
                    {
                        overview.title = text.to_string();
                    }

                    self.save(registry, ctx);
                }
                Action::EditEntry(text_box) => {
                    self.edit_entry(text_box, ctx);
                }
                Action::RemoveFocus(text_box) => {
                    self.remove_focus(text_box, ctx);
                }
                Action::OpenTaskList(list_box) => {
                    self.open_task_list(list_box, ctx);
                }
            }
        }

        self.action = None;
    }
}
