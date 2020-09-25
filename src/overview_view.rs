use orbtk::prelude::*;

use crate::{data::TaskOverview, keys::*, overview_state::*};

widget!(
    /// Represents the start page with the overview of task lists.
    OverviewView<OverviewState> {
        task_overview: TaskOverview,
        count: usize,
        task_view: u32,
        master_detail: u32,
        list_dirty: bool
    }
);

impl Template for OverviewView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        // list of task lists
        let list_view = ListView::new()
            .id(ID_OVERVIEW_LIST_VIEW)
            .count(id)
            .attach(Grid::row(2))
            .request_update(("list_dirty", id))
            .items_builder(move |ctx, index| {
                let mut text = "".to_string();

                if let Some(task_overview) = ctx
                    .get_widget(id)
                    .get::<TaskOverview>(PROP_TASK_OVERVIEW)
                    .get(index)
                {
                    text = task_overview.title.clone();
                }

                TextBlock::new().text(text).v_align("center").build(ctx)
            })
            .on_selection_changed(move |ctx, _, selected| {
                ctx.get_mut::<OverviewState>(id)
                    .action(Action::OpenTaskList(selected[0]));
            })
            .build(ctx);

        self.name("Overview")
            .task_overview(TaskOverview::default())
            .count(0)
            .child(
                Grid::new()
                    .rows(Rows::create().push(52).push(1).push("*").push(25))
                    // Top Bar
                    .child(
                        Container::new()
                            .style(STYLE_TOP_BAR)
                            .attach(Grid::row(0))
                            .child(
                                Grid::new()
                                    .child(
                                        TextBlock::new()
                                            .margin((36, 0, 0, 0))
                                            .style(STYLE_HEADER)
                                            .v_align("center")
                                            .h_align("start")
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
                    .child(
                        Container::new()
                            .width(1)
                            .h_align("end")
                            .style(STYLE_SEPARATOR)
                            .attach(Grid::row(0))
                            .attach(Grid::row_span(4))
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
