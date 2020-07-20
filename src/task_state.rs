use orbtk::prelude::*;

use crate::{
    base_state::BaseState,
    data::{Task, TaskOverview},
    keys::*,
};

/// Actions that can execute on the task view.
#[derive(Debug, Copy, Clone)]
pub enum Action {
    InputTextChanged(Entity),
    newEntry(Entity),
    RemoveEntry(usize),
    TextChanged(Entity, usize),
    EditEntry(Entity),
    RemoveFocus(Entity),
    SelectionChanged(Entity, usize),
    NavigateBack(),
}

/// Handles the requests of the `OverviewView`.
#[derive(Default, AsAny)]
pub struct TaskState {
    action: Option<Action>,
    add_button: Entity,
    back_entity: Entity,
    last_focused: Option<Entity>,
    pub text_box: Entity,
    open: bool,
}

impl BaseState for TaskState {}

impl TaskState {
    /// Sets a new action.
    pub fn action(&mut self, action: Action) {
        self.action = action.into();
    }

    fn new_entry(&self, text: String, registry: &mut Registry, ctx: &mut Context) {
        let index = ctx.widget().clone::<Option<usize>>("list_index");

        if let Some(index) = index {
            if let Some(task_list) = ctx
                .widget()
                .get_mut::<TaskOverview>("task_overview")
                .get_mut(index)
            {
                task_list.push(Task {
                    text,
                    selected: false,
                });
            }

            self.adjust_count(ctx);
        }

        self.save(registry, ctx);
    }

    fn adjust_count(&self, ctx: &mut Context) {
        if let Some(index) = ctx.widget().clone::<Option<usize>>("list_index") {
            if let Some(task_list) = ctx
                .widget()
                .clone::<TaskOverview>("task_overview")
                .get(index)
            {
                ctx.widget().set("count", task_list.len());
            }
        }
    }

    fn navigate_back(&mut self, ctx: &mut Context) {
        ctx.get_widget(self.text_box)
            .set("text", String16::from(""));
        self.open = false;
        ctx.widget().set::<Option<usize>>("list_index", None);
        ctx.widget().set("count", 0 as usize);
        self.navigate(self.back_entity, ctx);
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

    fn toggle_selection(
        &self,
        entry: Entity,
        index: usize,
        registry: &mut Registry,
        ctx: &mut Context,
    ) {
        let selected: bool = *ctx.get_widget(entry).get("selected");

        if let Some(idx) = ctx.widget().clone::<Option<usize>>("list_index") {
            if let Some(task_list) = ctx
                .widget()
                .get_mut::<TaskOverview>("task_overview")
                .get_mut(idx)
            {
                if let Some(task) = task_list.get_mut(index) {
                    task.selected = selected;
                }
            }
        }

        self.save(registry, ctx);
    }

    pub fn open(&mut self, ctx: &mut Context) {
        if let Some(index) = ctx.widget().clone::<Option<usize>>("list_index") {
            let mut title: String16 = "".into();
            let mut count = 0;
            if let Some(task_list) = ctx.widget().get::<TaskOverview>("task_overview").get(index) {
                title = String16::from(task_list.title.as_str());
                count = task_list.len();
            }
            ctx.widget().set("title", title);
            ctx.widget().set("count", count);
            self.open = true;
        }
    }

    // Set the given text box to edit mode.
    fn edit_entry(&self, text_box: Entity, ctx: &mut Context) {
        if *ctx.get_widget(text_box).get::<bool>("focused") {
            ctx.get_widget(text_box).set("enabled", false);
            ctx.push_event_by_window(FocusEvent::RemoveFocus(text_box));
            return;
        }

        if let Some(old_focused_element) = ctx.window().get::<Global>("global").focused_widget {
            ctx.push_event_by_window(FocusEvent::RemoveFocus(old_focused_element));
        }

        ctx.get_widget(text_box).set("enabled", true);

        // select all
        ctx.get_widget(text_box)
            .get_mut::<TextSelection>("text_selection")
            .start_index = 0;
        ctx.get_widget(text_box)
            .get_mut::<TextSelection>("text_selection")
            .length = ctx.get_widget(text_box).get::<String16>("text").len();
        ctx.push_event_by_window(FocusEvent::RequestFocus(text_box));
    }

    fn remove_entry(&self, index: usize, registry: &mut Registry, ctx: &mut Context) {
        if let Some(idx) = ctx.widget().clone::<Option<usize>>("list_index") {
            if let Some(task_list) = ctx
                .widget()
                .get_mut::<TaskOverview>("task_overview")
                .get_mut(idx)
            {
                task_list.remove(index);
            }
        }

        self.adjust_count(ctx);

        self.save(registry, ctx);
    }

    fn update_entry(
        &self,
        text_box: Entity,
        index: usize,
        registry: &mut Registry,
        ctx: &mut Context,
    ) {
        let text: String16 = ctx.get_widget(text_box).clone("text");

        if let Some(idx) = ctx.widget().clone::<Option<usize>>("list_index") {
            if let Some(task_list) = ctx
                .widget()
                .get_mut::<TaskOverview>("task_overview")
                .get_mut(idx)
            {
                if let Some(task) = task_list.get_mut(index) {
                    task.text = text.to_string();
                }
            }
        }

        self.save(registry, ctx);
    }
}

impl State for TaskState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.back_entity = (*ctx.widget().get::<u32>("back_entity")).into();
        self.add_button = ctx
            .entity_of_child(ID_TASK_ADD_BUTTON)
            .expect("TaskState.init: Add button child could not be found.");
        self.text_box = ctx
            .entity_of_child(ID_TASK_TEXT_BOX)
            .expect("TaskState.init: Add text box could not be found.");
    }

    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if !self.open {
            self.open(ctx);
        }
        if let Some(action) = self.action {
            match action {
                Action::InputTextChanged(text_box) => {
                    self.adjust_add_button_enabled(text_box, ctx);
                }
                Action::newEntry(entity) => {
                    if let Some(text) = self.fetch_text(ctx, entity) {
                        self.new_entry(text, registry, ctx);
                    }
                }
                Action::RemoveEntry(index) => {
                    self.remove_entry(index, registry, ctx);
                }
                Action::SelectionChanged(entity, index) => {
                    self.toggle_selection(entity, index, registry, ctx);
                }
                Action::TextChanged(entity, index) => {
                    self.update_entry(entity, index, registry, ctx);
                }
                Action::EditEntry(text_box) => {
                    self.last_focused = Some(text_box);
                    self.edit_entry(text_box, ctx);
                }
                Action::RemoveFocus(text_box) => {
                    ctx.get_widget(text_box).set("enabled", false);
                    ctx.push_event_by_window(FocusEvent::RemoveFocus(text_box));
                }
                Action::NavigateBack() => {
                    self.navigate_back(ctx);
                }
            }
        }

        self.action = None;
    }
}
