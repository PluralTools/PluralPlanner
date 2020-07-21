use orbtk::{prelude::*, widgets::behaviors::MouseBehavior};

use crate::{data::TaskOverview, keys::*, overview_state::*};

widget!(
    /// Represents the start page with the overview of task lists.
    OverviewView<OverviewState> {
        task_overview: TaskOverview,
        count: usize,
        task_view: u32
    }
);

impl Template for OverviewView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        // list of task lists
        let list_view = ListView::new()
            .id(ID_OVERVIEW_ITEMS_WIDGET)
            .attach(Grid::column(0))
            .attach(Grid::column_span(3))
            .attach(Grid::row(2))
            .items_builder(move |ctx, index| {
                let mut text = "".to_string();

                if let Some(task_overview) = ctx
                    .get_widget(id)
                    .get::<TaskOverview>(PROP_TASK_OVERVIEW)
                    .get(index)
                {
                    text = task_overview.title.clone();
                }
                Grid::new()
                    .columns(Columns::new().add("*").add(4).add(32))
                    .child(
                        TextBlock::new()
                            .style(STYLE_TITLE)
                            .text(text)
                            .v_align("center")
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .style("icon_only")
                            .attach(Grid::column(2))
                            .v_align("center")
                            .icon(material_icons_font::MD_DELETE)
                            .on_mouse_down(|_, _| true)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<OverviewState>(id)
                                    .action(Action::RemoveEntry(index));
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx)
            })
            .count((PROP_COUNT, id))
            // selection changed
            .on_changed(move |ctx, _| {
                ctx.get_mut::<OverviewState>(id)
                    .action(Action::OpenTaskList)
            })
            .build(ctx);

        let over_view_text_box = TextBox::new()
            .id(ID_OVERVIEW_TEXT_BOX)
            .attach(Grid::row(4))
            .v_align("center")
            .margin((4, 0, 0, 0))
            .lost_focus_on_activation(false)
            .on_activate(move |ctx, entity| {
                ctx.get_mut::<OverviewState>(id)
                    .action(Action::NewEntry(entity));
            })
            .on_changed(move |ctx, entity| {
                ctx.get_mut::<OverviewState>(id)
                    .action(Action::InputTextChanged(entity));
            })
            .build(ctx);

        self.name("Overview")
            .task_overview(TaskOverview::default())
            .count(0)
            .child(
                Grid::new()
                    .rows(Rows::new().add(52).add(1).add("*").add(1).add(40).build())
                    .columns(Columns::new().add("*").add(4).add(36).build())
                    // Top Bar
                    .child(
                        Container::new()
                            .style(STYLE_TOP_BAR)
                            .attach(Grid::row(0))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(3))
                            .child(
                                Grid::new()
                                    .child(
                                        TextBlock::new()
                                            .style(STYLE_HEADER)
                                            .v_align("center")
                                            .h_align("center")
                                            .text("Overview")
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
                    // Content
                    .child(list_view)
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
                    .child(
                        Button::new()
                            .attach(Grid::row(4))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(3))
                            .on_mouse_down(|_, _| true)
                            .on_click(|_, _| true)
                            .style(STYLE_TRANSPARENT)
                            .build(ctx),
                    )
                    .child(over_view_text_box)
                    .child(
                        Button::new()
                            .id(ID_OVERVIEW_ADD_BUTTON)
                            .style(STYLE_ICON_ONLY)
                            .attach(Grid::row(4))
                            .attach(Grid::column(2))
                            .margin((0, 0, 4, 0))
                            .enabled(false)
                            .min_size(32, 32)
                            .v_align("center")
                            .icon(material_icons_font::MD_ADD)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<OverviewState>(id)
                                    .action(Action::NewEntry(over_view_text_box));
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
