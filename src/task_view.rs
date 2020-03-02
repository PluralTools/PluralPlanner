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

use crate::{
    data::TaskOverview,
    keys::*,
    overview_state::*,
    task_state::{Action, TaskState},
};

type ListIndex = Option<usize>;

widget!(TaskView<TaskState> {
    back_entity: u32,

    list_index: ListIndex,

    task_overview: TaskOverview,

    count: usize,

    title: String16
});

impl Template for TaskView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        // list of task lists
        let items_widget = ItemsWidget::create()
            .id(ID_TASK_ITEMS_WIDGET)
            .vertical_alignment("start")
            .items_builder(move |ctx, index| {
                let mut text = "".to_string();
                let mut selected = false;

                if let Some(list_index) = ctx.get_widget(id).clone::<ListIndex>("list_index") {
                    if let Some(task_overview) = ctx
                        .get_widget(id)
                        .get::<TaskOverview>(PROP_TASK_OVERVIEW)
                        .get(list_index)
                    {
                        if let Some(task) = task_overview.get(index) {
                            text = task.text.clone();
                            selected = task.selected;
                        }
                    }
                }

                let text_block = TextBlock::create()
                    .margin((8.0, 0.0, 0.0, 0.0))
                    .vertical_alignment("center")
                    .attach(Grid::column(3))
                    .text(text)
                    .element("text-box")
                    .build(ctx);

                let text_box = TextBox::create()
                    .visibility("collapsed")
                    .vertical_alignment("center")
                    .water_mark("Insert text...")
                    .class("inplace")
                    .attach(Grid::column(3))
                    .text(text_block)
                    .on_changed(move |ctx, entity| {
                        ctx.get_mut::<TaskState>(id)
                            .action(Action::TextChanged(entity, index));
                    })
                    .on_activate(move |ctx, entity| {
                        ctx.get_mut::<TaskState>(id)
                            .action(Action::RemoveFocus(entity));
                    })
                    .build(ctx);

                Grid::create()
                    .columns(
                        Columns::create()
                            .column(10.0)
                            .column(24.0)
                            .column(4.0)
                            .column("*")
                            .column(4.0)
                            .column(32.0)
                            .column(4.0)
                            .column(32.0)
                            .column(4.0)
                            .build(),
                    )
                    .child(
                        CheckBox::create()
                            .attach(Grid::column(1))
                            .vertical_alignment("center")
                            .selected(selected)
                            .on_changed(move |ctx, entity| {
                                // ctx.get_mut::<TaskState>(id)
                                //     .action(Action::SelectionChanged(entity, index));
                            })
                            .build(ctx),
                    )
                    .child(text_box)
                    .child(text_block)
                    .child(
                        ToggleButton::create()
                            .selected(("focused", text_box))
                            .class(CLASS_ICON_ONLY)
                            .attach(Grid::column(5))
                            .min_size(32.0, 32.0)
                            .vertical_alignment("center")
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            // .selected(("focused", text_box))
                            .class(CLASS_ICON_ONLY)
                            .attach(Grid::column(5))
                            .min_size(32.0, 32.0)
                            .vertical_alignment("center")
                            // todo use remove from icons
                            // .icon(material_font_icons::DELETE_FONT_ICON)
                            .icon("")
                            .on_mouse_down(|_, _| true)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<TaskState>(id)
                                    .action(Action::EditEntry(text_box));
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .class("icon_only")
                            .attach(Grid::column(7))
                            .min_size(32.0, 32.0)
                            .vertical_alignment("center")
                            // todo use remove from icons
                            // .icon(material_font_icons::DELETE_FONT_ICON)
                            .icon("")
                            .on_mouse_down(|_, _| true)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<TaskState>(id)
                                    .action(Action::RemoveEntry(index));
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx)
            })
            .count((PROP_COUNT, id))
            .build(ctx);

        let scroll_viewer = ScrollViewer::create()
            .scroll_viewer_mode(("disabled", "auto"))
            .child(items_widget)
            .build(ctx);

        let task_text_box = TextBox::create()
            .id(ID_TASK_TEXT_BOX)
            .attach(Grid::row(2))
            .vertical_alignment("center")
            .margin((4.0, 0.0, 0.0, 0.0))
            .lost_focus_on_activation(false)
            .on_activate(move |ctx, entity| {
                ctx.get_mut::<TaskState>(id)
                    .action(Action::CreateEntry(entity));
            })
            .on_changed(move |ctx, entity| {
                ctx.get_mut::<TaskState>(id)
                    .action(Action::InputTextChanged(entity));
            })
            .build(ctx);

        self.name("TaskView").child(
            Grid::create()
                .rows(Rows::create().row(52.0).row("*").row("auto").build())
                .columns(
                    Columns::create()
                        .column("*")
                        .column(4.0)
                        .column(36.0)
                        .build(),
                )
                // Content
                .child(
                    Container::create()
                        .attach(Grid::row(1))
                        .attach(Grid::column(0))
                        .attach(Grid::column_span(3))
                        .child(scroll_viewer)
                        .child(
                            ScrollIndicator::create()
                                .padding((0.0, 4.0, 0.0, 0.0))
                                .content_id(items_widget.0)
                                .scroll_offset(scroll_viewer)
                                .build(ctx),
                        )
                        .build(ctx),
                )
                // Top Bar
                .child(
                    Container::create()
                        .class(CLASS_TOP_BAR)
                        .attach(Grid::row(0))
                        .attach(Grid::column(0))
                        .attach(Grid::column_span(3))
                        .child(
                            Grid::create()
                                .columns(
                                    Columns::create()
                                        .column(32.0)
                                        .column(4.0)
                                        .column("*")
                                        .column(4.0)
                                        .column(32.0)
                                        .build(),
                                )
                                .child(
                                    Button::create()
                                        .height(32.0)
                                        .icon("")
                                        .class(CLASS_ICON_ONLY)
                                        .vertical_alignment("center")
                                        .on_click(move |ctx, _| {
                                            ctx.get_mut::<TaskState>(id)
                                                .action(Action::NavigateBack());
                                            true
                                        })
                                        .build(ctx),
                                )
                                .child(
                                    TextBlock::create()
                                        .class(CLASS_HEADER)
                                        .attach(Grid::column(2))
                                        .vertical_alignment("center")
                                        .horizontal_alignment("center")
                                        .text(("title", id))
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .build(ctx),
                )
                // Bottom bar
                .child(
                    Container::create()
                        .class(CLASS_BOTTOM_BAR)
                        .attach(Grid::row(2))
                        .attach(Grid::column(0))
                        .attach(Grid::column_span(3))
                        .build(ctx),
                )
                .child(task_text_box)
                .child(
                    Button::create()
                        .id(ID_TASK_ADD_BUTTON)
                        .class(CLASS_ICON_ONLY)
                        .attach(Grid::row(2))
                        .attach(Grid::column(2))
                        .margin((0.0, 0.0, 4.0, 0.0))
                        .enabled(false)
                        .min_size(32.0, 32.0)
                        .vertical_alignment("center")
                        .icon(material_font_icons::ADD_FONT_ICON)
                        .on_click(move |ctx, _| {
                            ctx.get_mut::<TaskState>(id)
                                .action(Action::CreateEntry(task_text_box));
                            true
                        })
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
