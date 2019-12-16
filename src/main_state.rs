use std::collections::VecDeque;

use orbtk::prelude::*;

use crate::{Task, TaskList};

#[derive(Debug)]
pub enum Action {
    CreateEntry(Entity),
    RemoveEntry(usize),
    SelectionChanged(Entity, usize),
    TextChanged(Entity, usize),
}

#[derive(Default, AsAny)]
pub struct MainState {
    actions: VecDeque<Action>,
}

impl MainState {
    pub fn action(&mut self, action: Action) {
        self.actions.push_front(action);
    }

    pub fn create_entry(&self, text: String, ctx: &mut Context) {
        ctx.widget().get_mut::<TaskList>("tasks").push(Task {
            text,
            selected: false,
        });
        self.adjust_count(ctx);
    }

    pub fn remove_entry(&self, index: usize, ctx: &mut Context) {
        ctx.widget().get_mut::<TaskList>("tasks").remove(index);
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

    fn adjust_count(&self, ctx: &mut Context) {
        let count = ctx.widget().get::<TaskList>("tasks").len();
        ctx.widget().set("task_count", count);
    }

    fn save(&self, registry: &mut Registry, ctx: &mut Context) {
        registry
            .get::<Settings>("settings")
            .save("tasks", ctx.widget().get::<TaskList>("tasks"))
            .unwrap();
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

    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.actions.pop_front() {
            match action {
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
                Action::SelectionChanged(entity, index) => {
                    let selected: bool = *ctx.get_widget(entity).get("selected");

                    if let Some(task) = ctx.widget().get_mut::<TaskList>("tasks").get_mut(index) {
                        task.selected = selected;
                    }

                    self.save(registry, ctx);
                }
                Action::TextChanged(entity, index) => {
                    let text: String16 = ctx.get_widget(entity).clone("text");

                    if let Some(task) = ctx.widget().get_mut::<TaskList>("tasks").get_mut(index) {
                        task.text = text.to_string();
                    }

                    self.save(registry, ctx);
                }
            }
        }
    }
}
