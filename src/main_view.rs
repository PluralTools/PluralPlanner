use orbtk::prelude::*;

use crate::{MainState, TaskList};

widget!(MainView<MainState> {
    tasks: TaskList,

    task_count: usize
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let items_widget = ItemsWidget::create()
            .items_builder(move |ctx, index| {
                let mut text = "".to_string();
                let mut checked = false;
                
                if let Some(task) = ctx.get_widget(id).get::<TaskList>("tasks").get(index).clone() {
                    text = task.text.clone();
                    checked = task.checked;
                }

                TextBlock::create().text(text).build(ctx)
              
              

                // Button::create()
                //     .margin((0.0, 0.0, 0.0, 2.0))
                //     .text(text)
                //     .build(bc)
            })
            .count(("task_count", id))
            .build(ctx);
        let scroll_viewer = ScrollViewer::create().build(ctx);

        self.name("MainView")
            .tasks(TaskList::default())
            .task_count(0)
            .child(
                Grid::create()
                    .rows(Rows::create().row("*").row(32.0).build())
                    .child(
                        Container::create()
                            .attach(Grid::row(0))
                            .child(items_widget)
                            .child(scroll_viewer)
                            .child(
                                ScrollIndicator::create()
                                    .content_id(items_widget.0)
                                    .scroll_offset(scroll_viewer)
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(TextBox::create().attach(Grid::row(1)).build(ctx))
                    .build(ctx),
            )
    }
}
