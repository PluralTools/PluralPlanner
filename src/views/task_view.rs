use orbtk::prelude::*;

use crate::{
    data::TaskOverview,
    keys::*,
    states::{TaskAction, TaskState},
};

type ListIndex = Option<usize>;

widget!(TaskView<TaskState> {
    overview: u32,
    list_index: ListIndex,
    task_overview: TaskOverview,
    count: usize,
    title: String,
    create: bool,
    master_detail: u32,
    back_visibility: Visibility
});

impl Template for TaskView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        // list of task lists
        let items_widget = ItemsWidget::new()
            .id(ID_TASK_ITEMS_WIDGET)
            .v_align("start")
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

                Grid::new()
                    .height(48)
                    .margin((8, 0))
                    .columns(
                        Columns::create()
                            .push(24)
                            .push(4)
                            .push("*")
                            .push(4)
                            .push(32),
                    )
                    .child(
                        CheckBox::new()
                            .v_align("center")
                            .selected(selected)
                            .on_changed("selected", move |ctx, entity| {
                                ctx.get_mut::<TaskState>(id)
                                    .action(TaskAction::SelectionChanged(entity, index));
                            })
                            .build(ctx),
                    )
                    .child(
                        TextBox::new()
                            .style(STYLE_TEXT_BOX_INLINE)
                            .text(text)
                            .v_align("center")
                            .water_mark("Insert text...")
                            .lose_focus_on_activation(true)
                            .attach(Grid::column(2))
                            .on_activate(move |ctx, entity| {
                                ctx.get_mut::<TaskState>(id)
                                    .action(TaskAction::UpdateEntry(entity, index));
                            })
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .style(STYLE_BUTTON_ICON_ONLY)
                            .attach(Grid::column(3))
                            .v_align("center")
                            .icon(material_icons_font::MD_DELETE)
                            .on_mouse_down(|_, _| true)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<TaskState>(id)
                                    .action(TaskAction::RemoveEntry(index));
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx)
            })
            .count(id)
            .build(ctx);

        let scroll_viewer = ScrollViewer::new()
            .mode(("disabled", "auto"))
            .child(items_widget)
            .build(ctx);

        let task_text_box = TextBox::new()
            .id(ID_TASK_TEXT_BOX)
            .water_mark("Add new task...")
            .attach(Grid::row(4))
            .v_align("center")
            .margin((8, 0, 0, 0))
            .lose_focus_on_activation(false)
            .on_activate(move |ctx, entity| {
                ctx.get_mut::<TaskState>(id)
                    .action(TaskAction::NewEntry(entity));
            })
            .on_changed("text", move |ctx, entity| {
                ctx.get_mut::<TaskState>(id)
                    .action(TaskAction::InputTextChanged(entity));
            })
            .build(ctx);

        self.name("TaskView")
            .child(
                Grid::new()
                    .style(STYLE_TASK_VIEW_GRID)
                    .rows(
                        Rows::create()
                            .push(52)
                            .push(1)
                            .push("*")
                            .push(1)
                            .push("auto"),
                    )
                    .columns(Columns::create().push("*").push(4).push(36))
                    // Content
                    .child(
                        Container::new()
                            .attach(Grid::row(2))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(3))
                            .child(scroll_viewer)
                            .child(
                                ScrollIndicator::new()
                                    .padding((0, 4, 4, 0))
                                    .content_bounds(("bounds", items_widget))
                                    .view_port_bounds(("bounds", scroll_viewer))
                                    .scroll_padding(("padding", scroll_viewer))
                                    .mode(scroll_viewer)
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    // Top Bar
                    .child(
                        Container::new()
                            .style(STYLE_TOP_BAR)
                            .attach(Grid::row(0))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(3))
                            .child(
                                Grid::new()
                                    .margin((4, 0))
                                    .columns(
                                        Columns::create()
                                            .push(32)
                                            .push(4)
                                            .push("*")
                                            .push(4)
                                            .push(32),
                                    )
                                    .child(
                                        Button::new()
                                            .style(STYLE_BUTTON_ICON_ONLY)
                                            .icon(material_icons_font::MD_ARROW_BACK)
                                            .visibility(("back_visibility", id))
                                            .v_align("center")
                                            .on_click(move |ctx, _| {
                                                ctx.get_mut::<TaskState>(id)
                                                    .action(TaskAction::NavigateBack);
                                                true
                                            })
                                            .build(ctx),
                                    )
                                    .child(
                                        TextBox::new()
                                            .id(ID_TASK_HEADER_TEXT_BOX)
                                            .style(STYLE_TEXT_BOX_HEADER)
                                            .lose_focus_on_activation(true)
                                            .attach(Grid::column(2))
                                            .v_align("center")
                                            .text(("title", id))
                                            .on_activate(move |ctx, _| {
                                                ctx.get_mut::<TaskState>(id)
                                                    .action(TaskAction::Rename);
                                            })
                                            .build(ctx),
                                    )
                                    .child(
                                        Button::new()
                                            .attach(Grid::column(4))
                                            .style(STYLE_BUTTON_ICON_ONLY)
                                            .icon(material_icons_font::MD_DELETE)
                                            .v_align("center")
                                            .on_click(move |ctx, _| {
                                                ctx.get_mut::<TaskState>(id)
                                                    .action(TaskAction::RemoteList);
                                                true
                                            })
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Container::new()
                            .style("separator")
                            .attach(Grid::row(1))
                            .attach(Grid::column_span(3))
                            .build(ctx),
                    )
                    .child(
                        Container::new()
                            .style("separator")
                            .attach(Grid::row(3))
                            .attach(Grid::column_span(3))
                            .build(ctx),
                    )
                    // Bottom bar
                    .child(
                        Container::new()
                            .style(STYLE_BOTTOM_BAR)
                            .attach(Grid::row(4))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(3))
                            .build(ctx),
                    )
                    .child(task_text_box)
                    .child(
                        Button::new()
                            .id(ID_TASK_ADD_BUTTON)
                            .style("button_single_content")
                            .attach(Grid::row(4))
                            .attach(Grid::column(2))
                            .margin((0, 0, 4, 0))
                            .enabled(false)
                            .min_size(32, 32)
                            .v_align("center")
                            .icon(material_icons_font::MD_SEND)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<TaskState>(id)
                                    .action(TaskAction::NewEntry(task_text_box));
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_changed("list_index", move |ctx, _| {
                ctx.get_mut::<TaskState>(id).action(TaskAction::Open)
            })
    }
}
