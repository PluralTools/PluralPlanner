use std::collections::VecDeque;

use orbtk::prelude::*;

use crate::{
    data::{TaskList, TaskOverview},
    keys::*,
};

#[derive(Debug)]
pub enum Action {
    InputTextChanged(Entity),
    CreateEntry(Entity),
    RemoveEntry(usize),
    TextChanged(Entity, usize),
    EditEntry(Entity),
    RemoveFocus(Entity),
    OpenTaskList(Entity)
}

#[derive(Default, AsAny)]
pub struct OverviewState {
    actions: VecDeque<Action>,
    add_button: Entity,
}

impl OverviewState {
    pub fn action(&mut self, action: Action) {
        self.actions.push_front(action);
    }

    pub fn create_entry(&self, text: String, ctx: &mut Context) {
        ctx.widget()
            .get_mut::<TaskOverview>(PROP_TASK_OVERVIEW)
            .push(TaskList::new(text));
        self.adjust_count(ctx);
    }

    pub fn remove_focus(&self, text_box: Entity, ctx: &mut Context) {
        ctx.get_widget(text_box).set("enabled", false);
        ctx.window().get_mut::<Global>("global").focused_widget = None;
        ctx.get_widget(text_box).set("focused", false);
        ctx.get_widget(text_box).update_theme_by_state(false);
    }

    pub fn edit_entry(&self, text_box: Entity, ctx: &mut Context) {
        if let Some(old_focused_element) = ctx.window().get::<Global>("global").focused_widget {
            if old_focused_element == text_box {
                self.remove_focus(text_box, ctx);
                return;
            }

            let mut old_focused_element = ctx.get_widget(old_focused_element);
            old_focused_element.set("focused", false);
            old_focused_element.update_theme_by_state(false);
        }

        ctx.window().get_mut::<Global>("global").focused_widget = Some(text_box);

        ctx.get_widget(text_box).set("focused", true);
        ctx.get_widget(text_box).update_theme_by_state(false);
        ctx.get_widget(text_box).set("enabled", true);

    }

    pub fn remove_entry(&self, index: usize, ctx: &mut Context) {
        ctx.widget()
            .get_mut::<TaskOverview>(PROP_TASK_OVERVIEW)
            .remove(index);
        self.adjust_count(ctx);
    }

    fn fetch_text(&self, ctx: &mut Context, entity: Entity) -> Option<String> {
        let mut widget = ctx.get_widget(entity);

        let entry = widget.get_mut::<String16>("text");
        if entry.is_empty() {
            return None;
        }

        let copy = entry.to_string();
        entry.clear();
        Some(copy)
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

    fn adjust_count(&self, ctx: &mut Context) {
        let count = ctx.widget().get::<TaskOverview>(PROP_TASK_OVERVIEW).len();
        ctx.widget().set(PROP_COUNT, count);
    }

    fn save(&self, registry: &mut Registry, ctx: &mut Context) {
        registry
            .get::<Settings>("settings")
            .save(
                PROP_TASK_OVERVIEW,
                ctx.widget().get::<TaskOverview>(PROP_TASK_OVERVIEW),
            )
            .unwrap();
    }

    fn open_task_list(&self, entity: Entity, ctx: &mut Context) {

    }
}

impl State for OverviewState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.add_button = ctx
            .entity_of_child(ID_OVERVIEW_ADD_BUTTON)
            .expect("OverviewState.init: Add button child could not be found.");
        if let Ok(tasks) = registry
            .get::<Settings>("settings")
            .load::<TaskOverview>(PROP_TASK_OVERVIEW)
        {
            ctx.widget().set(PROP_TASK_OVERVIEW, tasks);
        }

        self.adjust_count(ctx);
    }

    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.actions.pop_front() {
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
    }
}
