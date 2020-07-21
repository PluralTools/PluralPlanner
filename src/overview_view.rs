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
            .style(STYLE_LIST_VIEW_BORDER_LESS)
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
            .on_selection_changed(move |ctx, _, indices| {
                if indices.is_empty() {
                    return;
                }

                ctx.get_mut::<OverviewState>(id)
                    .action(Action::OpenTaskList(indices[0]))
            })
            .build(ctx);

        self.name("Overview")
            .task_overview(TaskOverview::default())
            .count(0)
            .child(
                Grid::new()
                    .rows(Rows::new().add(52).add(1).add("*").add(25))
                    // Top Bar
                    .child(
                        Container::new()
                            .style(STYLE_TOP_BAR)
                            .attach(Grid::row(0))
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
                            .style(STYLE_SEPARATOR)
                            .attach(Grid::row(1))
                            .build(ctx),
                    )
                    // Content
                    .child(list_view)
                    .child(
                        Button::new()
                            .style(STYLE_BUTTON_FLOAT)
                            .attach(Grid::row(2))
                            .attach(Grid::row_span(2))
                            .margin(8)
                            .min_size(32, 32)
                            .v_align("end")
                            .h_align("end")
                            .icon(material_icons_font::MD_ADD)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<OverviewState>(id).action(Action::NewEntry);
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
