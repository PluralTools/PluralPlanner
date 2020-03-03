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
        let items_widget = ItemsWidget::create()
            .id(ID_OVERVIEW_ITEMS_WIDGET)
            .vertical_alignment("start")
            .items_builder(move |ctx, index| {
                let mut text = "".to_string();

                if let Some(task_overview) = ctx
                    .get_widget(id)
                    .get::<TaskOverview>(PROP_TASK_OVERVIEW)
                    .get(index)
                {
                    text = task_overview.title.clone();
                }

                let helper_button = Button::create()
                    .min_height(48.0)
                    .class(CLASS_ITEM_BUTTON)
                    .attach(Grid::column(0))
                    .attach(Grid::column_span(6))
                    .on_click(move |ctx, _| {
                        ctx.get_mut::<OverviewState>(id)
                            .action(Action::OpenTaskList(index));
                        true
                    })
                    .build(ctx);

                let text_block = TextBlock::create()
                    .foreground(helper_button)
                    .margin((14.0, 0.0, 0.0, 0.0))
                    .vertical_alignment("center")
                    .attach(Grid::column(0))
                    .text(text)
                    .element("text-box")
                    .build(ctx);

                let text_box = TextBox::create()
                    .margin((8.0, 0.0, 0.0, 0.0))
                    .visibility("collapsed")
                    .vertical_alignment("center")
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

                Grid::create()
                    .height(48.0)
                    .columns(
                        Columns::create()
                            .column("*")
                            .column(8.0)
                            .column(32.0)
                            .column(4.0)
                            .column(32.0)
                            .column(8.0)
                            .build(),
                    )
                    .child(helper_button)
                    .child(text_box)
                    .child(text_block)
                    .child(
                        ToggleButton::create()
                            .selected(("focused", text_box))
                            .class(CLASS_ICON_ONLY)
                            .attach(Grid::column(2))
                            .min_size(32.0, 32.0)
                            .vertical_alignment("center")
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            // .selected(("focused", text_box))
                            .class(CLASS_ICON_ONLY)
                            .attach(Grid::column(2))
                            .min_size(32.0, 32.0)
                            .vertical_alignment("center")
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
                        Button::create()
                            .class("icon_only")
                            .attach(Grid::column(4))
                            .min_size(32.0, 32.0)
                            .vertical_alignment("center")
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

        let scroll_viewer = ScrollViewer::create()
            .scroll_viewer_mode(("disabled", "auto"))
            .child(items_widget)
            .build(ctx);

        let over_view_text_box = TextBox::create()
            .id(ID_OVERVIEW_TEXT_BOX)
            .attach(Grid::row(4))
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
                    .rows(Rows::create().row(52.0).row(1.0).row("*").row(1.0).row(40.0).build())
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
                            .attach(Grid::row(2))
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
                            .build(ctx),
                    )
                    .child(
                        Container::create()
                            .class("separator")
                            .attach(Grid::row(1))
                            .attach(Grid::column_span(3))
                            .build(ctx),
                    )
                    .child(
                        Container::create()
                            .class("separator")
                            .attach(Grid::row(3))
                            .attach(Grid::column_span(3))
                            .build(ctx),
                    )
                    // Bottom bar
                    .child(
                        Container::create()
                            .class(CLASS_BOTTOM_BAR)
                            .attach(Grid::row(4))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(3))
                            .build(ctx),
                    )
                    .child(
                        // workaround, todo fix scroll viewer mouse behavior in OrbTk
                        Button::create()
                            .attach(Grid::row(4))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(3))
                            .on_mouse_down(|_, _| true)
                            .on_mouse_up(|_, _| true)
                            .on_click(|_, _| true)
                            .class(CLASS_TRANSPARENT)
                            .build(ctx),
                    )
                    .child(over_view_text_box)
                    .child(
                        Button::create()
                            .id(ID_OVERVIEW_ADD_BUTTON)
                            .class(CLASS_ICON_ONLY)
                            .attach(Grid::row(4))
                            .attach(Grid::column(2))
                            .margin((0.0, 0.0, 4.0, 0.0))
                            .enabled(false)
                            .min_size(32.0, 32.0)
                            .vertical_alignment("center")
                            .icon(material_font_icons::ADD_FONT_ICON)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<OverviewState>(id)
                                    .action(Action::CreateEntry(over_view_text_box));
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
