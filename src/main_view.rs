use orbtk::prelude::*;

use crate::{Action, MainState, TaskList};

widget!(MainView<MainState> {
    tasks: TaskList,

    task_count: usize
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let items_widget = ItemsWidget::create()
            .vertical_alignment("start")
            .items_builder(move |ctx, index| {
                let mut text = "".to_string();
                let mut selected = false;

                if let Some(task) = ctx
                    .get_widget(id)
                    .get::<TaskList>("tasks")
                    .get(index)
                    .clone()
                {
                    text = task.text.clone();
                    selected = task.selected;
                }

                Grid::create()
                    .margin((0.0, 0.0, 0.0, 0.4))
                    .columns(
                        Columns::create()
                            .column("Auto")
                            .column("*")
                            .column(4.0)
                            .column(32.0)
                            .build(),
                    )
                    .child(
                        CheckBox::create()
                            .attach(Grid::column(0))
                            .vertical_alignment("center")
                            .selected(selected)
                            .build(ctx),
                    )
                    .child(
                        TextBox::create()
                            .water_mark("Insert text...")
                            .selector(Selector::from("text-box").class("inplace"))
                            .attach(Grid::column(1))
                            .text(text)
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .selector(Selector::from("button").class("icon_only"))
                            .attach(Grid::column(3))
                            .min_size(32.0, 32.0)
                            .vertical_alignment("center")
                            .icon(material_font_icons::MINUS_FONT_ICON)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<MainState>(id).action(Action::RemoveEntry(index));
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx)
            })
            .count(("task_count", id))
            .build(ctx);

        let scroll_viewer = ScrollViewer::create()
            .scroll_viewer_mode(("disabled", "auto"))
            .child(items_widget)
            .build(ctx);

        self.name("MainView")
            .tasks(TaskList::default())
            .task_count(0)
            .child(
                Grid::create()
                    .rows(Rows::create().row("*").row(32.0).build())
                    .child(
                        Container::create()
                            .attach(Grid::row(0))
                            .child(scroll_viewer)
                            .child(
                                ScrollIndicator::create()
                                    .content_id(items_widget.0)
                                    .scroll_offset(scroll_viewer)
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        TextBox::create()
                            .attach(Grid::row(1))
                            .on_activate(move |ctx, entity| {
                                ctx.get_mut::<MainState>(id)
                                    .action(Action::CreateEntry(entity));
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
