use orbtk::prelude::*;

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
        let items_widget = ItemsWidget::new()
            .id(ID_OVERVIEW_ITEMS_WIDGET)
            .v_align("start")
            .items_builder(move |ctx, index| {
                let mut text = "".to_string();

                if let Some(task_overview) = ctx
                    .get_widget(id)
                    .get::<TaskOverview>(PROP_TASK_OVERVIEW)
                    .get(index)
                {
                    text = task_overview.title.clone();
                }

                let helper_button = Button::new()
                    .min_height(48)
                    .style(STYLE_ITEM_BUTTON)
                    .attach(Grid::column(0))
                    .attach(Grid::column_span(6))
                    .on_click(move |ctx, _| {
                        ctx.get_mut::<OverviewState>(id)
                            .action(Action::OpenTaskList(index));
                        true
                    })
                    .build(ctx);

                let text_block = TextBlock::new()
                    .foreground(helper_button)
                    .margin((14, 0, 0, 0))
                    .v_align("center")
                    .attach(Grid::column(0))
                    .text(text)
                    .style("text_box")
                    .build(ctx);

                let text_box = TextBox::new()
                    .margin((8, 0, 0, 0))
                    .visibility("collapsed")
                    .v_align("center")
                    .water_mark("Insert text...")
                    .attach(Grid::column(0))
                    .text(text_block)
                    .on_changed(move |ctx, entity| {
                        ctx.get_mut::<OverviewState>(id)
                            .action(Action::TextChanged(entity, index));
                    })
                    .on_activate(move |ctx, entity| {
                        ctx.get_mut::<OverviewState>(id)
                            .action(Action::RemoveFocus(entity));
                    })
                    .build(ctx);

                Grid::new()
                    .height(48)
                    .columns(Columns::new().add("*").add(8).add(32).add(4).add(32).add(8))
                    .child(helper_button)
                    .child(text_box)
                    .child(text_block)
                    .child(
                        ToggleButton::new()
                            .style(STYLE_ICON_ONLY)
                            .selected(("focused", text_box))
                            .attach(Grid::column(2))
                            .min_size(32, 32)
                            .v_align("center")
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            // .selected(("focused", text_box))
                            .style(STYLE_ICON_ONLY)
                            .attach(Grid::column(2))
                            .min_size(32, 32)
                            .v_align("center")
                            // todo use remove from icons
                            // .icon(material_font_icons::DELETE_FONT_ICON)
                            .icon("")
                            .on_mouse_down(|_, _| true)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<OverviewState>(id)
                                    .action(Action::EditEntry(text_box));
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .style("icon_only")
                            .attach(Grid::column(4))
                            .min_size(32, 32)
                            .v_align("center")
                            // todo use remove from icons
                            // .icon(material_font_icons::DELETE_FONT_ICON)
                            .icon("")
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
            .build(ctx);

        let scroll_viewer = ScrollViewer::new()
            .scroll_viewer_mode(("disabled", "auto"))
            .child(items_widget)
            .build(ctx);

        let over_view_text_box = TextBox::new()
            .id(ID_OVERVIEW_TEXT_BOX)
            .attach(Grid::row(4))
            .v_align("center")
            .margin((4, 0, 0, 0))
            .lost_focus_on_activation(false)
            .on_activate(move |ctx, entity| {
                ctx.get_mut::<OverviewState>(id)
                    .action(Action::newEntry(entity));
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
                    // Content
                    .child(
                        Container::new()
                            .attach(Grid::row(2))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(3))
                            .child(scroll_viewer)
                            .child(
                                ScrollIndicator::new()
                                    .padding((0, 4, 0, 0))
                                    .content_id(items_widget.0)
                                    .scroll_offset(scroll_viewer)
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
                                    .child(
                                        TextBlock::new()
                                            .style(STYLE_HEADER)
                                            .v_align("center")
                                            .horizontal_alignment("center")
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
                        // workaround, todo fix scroll viewer mouse behavior in OrbTk
                        Button::new()
                            .attach(Grid::row(4))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(3))
                            .on_mouse_down(|_, _| true)
                            .on_mouse_up(|_, _| true)
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
                                    .action(Action::newEntry(over_view_text_box));
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
