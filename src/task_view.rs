use orbtk::prelude::*;

use crate::{
    data::TaskOverview,
    keys::*,
    task_state::{Action, TaskState},
};

type ListIndex = Option<usize>;

widget!(TaskView<TaskState> {
    overview: u32,
    list_index: ListIndex,
    task_overview: TaskOverview,
    count: usize,
    title: String16,
    create: bool
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
                    .columns(Columns::new().add(24).add(4).add("*").add(4).add(32))
                    .child(
                        CheckBox::new()
                            .v_align("center")
                            .selected(selected)
                            .on_changed(move |ctx, entity, key| {
                                if key == "selected" {
                                    ctx.get_mut::<TaskState>(id)
                                        .action(Action::SelectionChanged(entity, index));
                                }
                            })
                            .build(ctx),
                    )
                    .child(
                        TextBox::new()
                            .style(STYLE_TEXT_BOX_INLINE)
                            .text(text)
                            .v_align("center")
                            .water_mark("Insert text...")
                            .attach(Grid::column(2))
                            .on_activate(move |ctx, entity| {
                                ctx.get_mut::<TaskState>(id)
                                    .action(Action::UpdateEntry(entity, index));
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
                                    .action(Action::RemoveEntry(index));
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx)
            })
            .count((PROP_COUNT, id))
            .build(ctx);

        let scroll_viewer = ScrollViewer::new()
            .mode(("disabled", "auto"))
            .child(items_widget)
            .build(ctx);

        let task_text_box = TextBox::new()
            .id(ID_TASK_TEXT_BOX)
            .attach(Grid::row(4))
            .v_align("center")
            .margin((8, 0, 0, 0))
            .lost_focus_on_activation(false)
            .on_activate(move |ctx, entity| {
                ctx.get_mut::<TaskState>(id)
                    .action(Action::NewEntry(entity));
            })
            .on_changed(move |ctx, entity, key| {
                if key == "text" {
                    ctx.get_mut::<TaskState>(id)
                        .action(Action::InputTextChanged(entity));
                }
            })
            .build(ctx);

        self.name("TaskView").child(
            Grid::new()
                .rows(
                    Rows::new()
                        .add(52)
                        .add(1)
                        .add("*")
                        .add(1)
                        .add("auto")
                        .build(),
                )
                .columns(Columns::new().add("*").add(4).add(36))
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
                                .columns(Columns::new().add(32).add(4).add("*").add(4).add(32))
                                .child(
                                    Button::new()
                                        .style(STYLE_BUTTON_ICON_ONLY)
                                        .icon(material_icons_font::MD_ARROW_BACK)
                                        .v_align("center")
                                        .on_click(move |ctx, _| {
                                            ctx.get_mut::<TaskState>(id)
                                                .action(Action::NavigateBack);
                                            true
                                        })
                                        .build(ctx),
                                )
                                .child(
                                    TextBox::new()
                                        .id(ID_TASK_HEADER_TEXT_BOX)
                                        .style(STYLE_TEXT_BOX_HEADER)
                                        .attach(Grid::column(2))
                                        .v_align("center")
                                        .text(("title", id))
                                        .on_activate(move |ctx, _| {
                                            ctx.get_mut::<TaskState>(id).action(Action::Rename);
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
                                            ctx.get_mut::<TaskState>(id).action(Action::RemoteList);
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
                        .style(STYLE_BUTTON_ICON_ONLY)
                        .attach(Grid::row(4))
                        .attach(Grid::column(2))
                        .margin((0, 0, 4, 0))
                        .enabled(false)
                        .min_size(32, 32)
                        .v_align("center")
                        .icon(material_icons_font::MD_ADD)
                        .on_click(move |ctx, _| {
                            ctx.get_mut::<TaskState>(id)
                                .action(Action::NewEntry(task_text_box));
                            true
                        })
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
