// use std::collections::VecDeque;

// use orbtk::prelude::*;

// use crate::{Task, TaskList};

// #[derive(Debug)]
// pub enum Action {
//     CreateEntry(Entity),
//     RemoveEntry(usize),
//     SelectionChanged(Entity, usize),
//     TextChanged(Entity, usize),
// }

// #[derive(Default, AsAny)]
// pub struct MainState {
//     actions: VecDeque<Action>,
// }

// impl MainState {
//     pub fn action(&mut self, action: Action) {
//         self.actions.push_front(action);
//     }

//     pub fn create_entry(&self, text: String, ctx: &mut Context) {
//         ctx.widget().get_mut::<TaskList>("tasks").push(Task {
//             text,
//             selected: false,
//         });
//         self.adjust_count(ctx);
//     }

//     pub fn remove_entry(&self, index: usize, ctx: &mut Context) {
//         ctx.widget().get_mut::<TaskList>("tasks").remove(index);
//         self.adjust_count(ctx);
//     }

//     fn fetch_text(&self, ctx: &mut Context, entity: Entity) -> Option<String> {
//         let mut widget = ctx.get_widget(entity);

//         let entry = widget.get_mut::<String16>("text");
//         if entry.is_empty() {
//             return None;
//         }

//         let copy = entry.to_string();
//         entry.clear();
//         Some(copy)
//     }

//     fn adjust_count(&self, ctx: &mut Context) {
//         let count = ctx.widget().get::<TaskList>("tasks").len();
//         ctx.widget().set("task_count", count);
//     }

//     fn save(&self, registry: &mut Registry, ctx: &mut Context) {
//         registry
//             .get::<Settings>("settings")
//             .save("tasks", ctx.widget().get::<TaskList>("tasks"))
//             .unwrap();
//     }
// }

// impl State for MainState {
//     fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
//         if let Ok(tasks) = registry
//             .get::<Settings>("settings")
//             .load::<TaskList>("tasks")
//         {
//             ctx.widget().set("tasks", tasks);
//         }

//         self.adjust_count(ctx);
//     }

//     fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
//         if let Some(action) = self.actions.pop_front() {
//             match action {
//                 Action::CreateEntry(entity) => {
//                     if let Some(text) = self.fetch_text(ctx, entity) {
//                         self.create_entry(text, ctx);
//                         self.save(registry, ctx);
//                     }
//                 }
//                 Action::RemoveEntry(index) => {
//                     self.remove_entry(index, ctx);
//                     self.save(registry, ctx);
//                 }
//                 Action::SelectionChanged(entity, index) => {
//                     let selected: bool = *ctx.get_widget(entity).get("selected");

//                     if let Some(task) = ctx.widget().get_mut::<TaskList>("tasks").get_mut(index) {
//                         task.selected = selected;
//                     }

//                     self.save(registry, ctx);
//                 }
//                 Action::TextChanged(entity, index) => {
//                     let text: String16 = ctx.get_widget(entity).clone("text");

//                     if let Some(task) = ctx.widget().get_mut::<TaskList>("tasks").get_mut(index) {
//                         task.text = text.to_string();
//                     }

//                     self.save(registry, ctx);
//                 }
//             }
//         }
//     }
// }

use orbtk::prelude::*;

use crate::{base_state::BaseState, data::TaskOverview, keys::*};

/// Actions that can execute on the task view.
#[derive(Debug, Copy, Clone)]
pub enum Action {
    InputTextChanged(Entity),
    CreateEntry(Entity),
    RemoveEntry(usize),
    TextChanged(Entity, usize),
    EditEntry(Entity),
    RemoveFocus(Entity),
    NavigateBack(),
}

/// Handles the requests of the `OverviewView`.
#[derive(Default, AsAny)]
pub struct TaskState {
    action: Option<Action>,
    add_button: Entity,
    back_entity: Entity,
    pub text_box: Entity,
    open: bool,
}

impl BaseState for TaskState {}

impl TaskState {
    /// Sets a new action.
    pub fn action(&mut self, action: Action) {
        self.action = action.into();
    }

    fn navigate_back(&mut self, ctx: &mut Context) {
        ctx.widget().set("enabled", false);
        self.open = false;
        ctx.widget().set::<Option<usize>>("list_index", None);
        self.navigate(self.back_entity, ctx);
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

    pub fn open(&mut self, ctx: &mut Context) {
        ctx.get_widget(self.text_box).set("enabled", true);
        if let Some(index) = ctx.widget().clone::<Option<usize>>("list_index") {
            let mut title: String16 = "".into();
            if let Some(task_list) = ctx.widget().get::<TaskOverview>("task_overview").get(index) {
                title = String16::from(task_list.title.as_str());
            }
            ctx.widget().set("title", title);
            self.open = true;
        }
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
                Action::CreateEntry(entity) => {
                    // if let Some(text) = self.fetch_text(ctx, entity) {
                    //     self.create_entry(text, ctx);
                    //     self.save(registry, ctx);
                    // }
                }
                Action::RemoveEntry(index) => {
                    // self.remove_entry(index, ctx);
                    // self.save(registry, ctx);
                }
                Action::TextChanged(entity, index) => {
                    // let text: String16 = ctx.get_widget(entity).clone("text");

                    // if let Some(overview) = ctx
                    //     .widget()
                    //     .get_mut::<TaskOverview>("task_overview")
                    //     .get_mut(index)
                    // {
                    //     overview.title = text.to_string();
                    // }

                    // self.save(registry, ctx);
                }
                Action::EditEntry(text_box) => {
                    // self.edit_entry(text_box, ctx);
                }
                Action::RemoveFocus(text_box) => {
                    // self.remove_focus(text_box, ctx);
                }
                Action::NavigateBack() => {
                    self.navigate_back(ctx);
                }
            }
        }

        self.action = None;
    }
}
