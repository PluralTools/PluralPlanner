use orbtk::prelude::*;

use crate::{data::TaskOverview, keys::*, overview_state::*};

widget!(OverviewView<OverviewState> {
    task_overview: TaskOverview,
    count: usize,
    task_view: u32
});

impl Template for OverviewView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let list_view = ListView::create()
            .items_builder(move |ctx, index| {
                let mut text = "".to_string();

                if let Some(task_overview) = ctx
                    .get_widget(id)
                    .get::<TaskOverview>(PROP_TASK_OVERVIEW)
                    .get(index)
                {
                    text = task_overview.title.clone();
                }
                let text_box = TextBox::create()
                    .vertical_alignment("center")
                    .water_mark("Insert text...")
                    .class("inplace")
                    .enabled(false)
                    .attach(Grid::column(0))
                    .text(text)
                    .on_changed(move |ctx, entity| {
                        ctx.get_mut::<OverviewState>(id)
                            .action(Action::TextChanged(entity, index));
                    })
                    .on_activate(move |ctx, entity| {
                        ctx.get_mut::<OverviewState>(id)
                            .action(Action::RemoveFocus(entity));
                    })
                    .build(ctx);

                Grid::create()
                    .margin((0.0, 0.0, 0.0, 0.4))
                    .columns(
                        Columns::create()
                            .column("*")
                            .column(4.0)
                            .column(32.0)
                            .column(4.0)
                            .column(32.0)
                            .build(),
                    )
                    .child(text_box)
                    .child(
                        Button::create()
                            .class("icon_only")
                            .attach(Grid::column(2))
                            .min_size(32.0, 32.0)
                            .vertical_alignment("center")
                            // todo use remove from icons
                            // .icon(material_font_icons::DELETE_FONT_ICON)
                            .icon("")
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<OverviewState>(id)
                                    .action(Action::EditEntry(text_box));
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .class("icon_only")
                            .attach(Grid::column(4))
                            .min_size(32.0, 32.0)
                            .vertical_alignment("center")
                            // todo use remove from icons
                            // .icon(material_font_icons::DELETE_FONT_ICON)
                            .icon("")
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
            .on_changed(move |ctx, entity| {
                ctx.get_mut::<OverviewState>(id)
                    .action(Action::OpenTaskList(entity));
            })
            .build(ctx);

        let text_box = TextBox::create()
            .attach(Grid::row(2))
            .vertical_alignment("center")
            .margin((4.0, 0.0, 0.0, 0.0))
            .lost_focus_on_activation(false)
            .on_activate(move |ctx, entity| {
                ctx.get_mut::<OverviewState>(id)
                    .action(Action::CreateEntry(entity));
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
                Grid::create()
                    .rows(Rows::create().row("auto").row("*").row("auto").build())
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
                            .child(list_view)
                            .build(ctx),
                    )
                    // Top Bar
                    .child(
                        Grid::create()
                            .class(CLASS_TOP_BAR)
                            .attach(Grid::row(0))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(3))
                            .child(
                                TextBlock::create()
                                    .class(CLASS_HEADER)
                                    .vertical_alignment("center")
                                    .horizontal_alignment("center")
                                    .text("Overview")
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
                    .child(text_box)
                    .child(
                        Button::create()
                            .id(ID_OVERVIEW_ADD_BUTTON)
                            .class("icon_only")
                            .attach(Grid::row(2))
                            .attach(Grid::column(2))
                            .margin((0.0, 0.0, 4.0, 0.0))
                            .enabled(false)
                            .min_size(32.0, 32.0)
                            .vertical_alignment("center")
                            .icon(material_font_icons::ADD_FONT_ICON)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<OverviewState>(id)
                                    .action(Action::CreateEntry(text_box));
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
