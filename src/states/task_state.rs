use orbtk::prelude::*;

use crate::{
    data::{Task, TaskOverview},
    keys::*,
    states::BaseState,
    views::TaskView,
};

/// TaskActions that can execute on the task view.
#[derive(Debug, Copy, Clone)]
pub enum TaskAction {
    InputTextChanged(Entity),
    NewEntry(Entity),
    RemoveEntry(usize),
    RemoteList,
    UpdateEntry(Entity, usize),
    RemoveFocus(Entity),
    SelectionChanged(Entity, usize),
    NavigateBack,
    Rename,
    Open,
}

/// Handles the requests of the `OverviewView`.
#[derive(Default, AsAny)]
pub struct TaskState {
    action: Option<TaskAction>,
    add_button: Entity,
    overview: Entity,
    header_text_box: Entity,
    items_widget: Entity,
    pub text_box: Entity,
    window: Entity,
}

impl BaseState for TaskState {}

impl TaskState {
    /// Sets a new action.
    pub fn action(&mut self, action: TaskAction) {
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
        ctx.event_adapter()
            .push_event_direct(self.window, FocusEvent::RemoveFocus(self.header_text_box));
        let master_detail = *TaskView::master_detail_ref(&ctx.widget());
        MasterDetail::show_master(ctx, master_detail.into());
    }

    // If input text is empty the add button is disabled, otherwise enabled.
    fn adjust_add_button_enabled(&self, text_box: Entity, ctx: &mut Context) {
        if ctx.get_widget(text_box).get::<String>("text").is_empty() {
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

                task_list.list.sort_by(|t1, t2| {
                    if t1.selected == t2.selected {
                        std::cmp::Ordering::Equal
                    } else if t1.selected && !t2.selected {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                });
            }
        }

        ItemsWidget::request_update_set(&mut ctx.get_widget(self.items_widget), true);
        self.save(registry, ctx);
    }

    pub fn open(&mut self, ctx: &mut Context) {
        if let Some(index) = ctx.widget().clone::<Option<usize>>("list_index") {
            let mut title: String = "".into();
            let mut count = 0;
            if let Some(task_list) = ctx.widget().get::<TaskOverview>("task_overview").get(index) {
                title = String::from(task_list.title.as_str());
                count = task_list.len();
            }
            ctx.widget().set("title", title);
            ctx.widget().set("count", count);
        }
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

            self.adjust_count(ctx);
            self.save(registry, ctx);
        }
    }

    // removes a task list.
    fn remove_list(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(index) = ctx.widget().clone::<Option<usize>>("list_index") {
            ctx.get_widget(self.overview)
                .get_mut::<TaskOverview>(PROP_TASK_OVERVIEW)
                .remove(index);

            let count = ctx
                .get_widget(self.overview)
                .get::<TaskOverview>(PROP_TASK_OVERVIEW)
                .len();

            ctx.get_widget(self.overview).set("count", count);
            ctx.get_widget(self.overview).set("list_dirty", true);

            self.save(registry, ctx);
        }

        self.navigate_back(ctx);
    }

    fn update_entry(
        &self,
        text_box: Entity,
        index: usize,
        registry: &mut Registry,
        ctx: &mut Context,
    ) {
        let text: String = ctx.get_widget(text_box).clone("text");
        //TextBox::selection_set(&mut ctx.get_widget(text_box), TextSelection::default());

        if let Some(idx) = ctx.widget().clone::<Option<usize>>("list_index") {
            if let Some(task_list) = ctx
                .widget()
                .get_mut::<TaskOverview>("task_overview")
                .get_mut(idx)
            {
                if let Some(task) = task_list.get_mut(index) {
                    task.text = text;
                }
            }
        }

        self.save(registry, ctx);
    }

    fn rename(&self, registry: &mut Registry, ctx: &mut Context) {
        let title = ctx.get_widget(self.header_text_box).clone::<String>("text");

        if let Some(idx) = ctx.widget().clone::<Option<usize>>("list_index") {
            if let Some(task_list) = ctx
                .widget()
                .get_mut::<TaskOverview>("task_overview")
                .get_mut(idx)
            {
                task_list.title = title;
            }
        }

        ctx.get_widget(self.overview).set("list_dirty", true);

        self.save(registry, ctx);
    }
}

impl State for TaskState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.overview = (*ctx.widget().get::<u32>("overview")).into();
        self.add_button = ctx
            .entity_of_child(ID_TASK_ADD_BUTTON)
            .expect("TaskState.init: Add button child could not be found.");
        self.text_box = ctx
            .entity_of_child(ID_TASK_TEXT_BOX)
            .expect("TaskState.init: Add text box could not be found.");
        self.header_text_box = ctx
            .entity_of_child(ID_TASK_HEADER_TEXT_BOX)
            .expect("TaskState.init: Header text box could not be found.");
        self.items_widget = ctx
            .entity_of_child(ID_TASK_ITEMS_WIDGET)
            .expect("TaskState.init: Items widget could not be found.");
        self.window = ctx.entity_of_window();
    }

    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                TaskAction::InputTextChanged(text_box) => {
                    self.adjust_add_button_enabled(text_box, ctx);
                }
                TaskAction::NewEntry(entity) => {
                    if let Some(text) = self.fetch_text(ctx, entity) {
                        self.new_entry(text, registry, ctx);
                    }
                }
                TaskAction::RemoveEntry(index) => {
                    self.remove_entry(index, registry, ctx);
                }
                TaskAction::SelectionChanged(entity, index) => {
                    self.toggle_selection(entity, index, registry, ctx);
                }
                TaskAction::UpdateEntry(entity, index) => {
                    self.update_entry(entity, index, registry, ctx);
                }
                TaskAction::RemoveFocus(text_box) => {
                    ctx.get_widget(text_box).set("enabled", false);
                    ctx.event_adapter()
                        .push_event_direct(self.window, FocusEvent::RemoveFocus(text_box));
                }
                TaskAction::NavigateBack => {
                    self.navigate_back(ctx);
                }
                TaskAction::RemoteList => {
                    self.remove_list(registry, ctx);
                }
                TaskAction::Rename => {
                    self.rename(registry, ctx);
                }
                TaskAction::Open => self.open(ctx),
            }
        }

        self.action = None;
    }
}
