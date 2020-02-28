use orbtk::prelude::*;

use crate::{data::TaskOverview, overview_view::OverviewView, task_view::TaskView};

widget!(MainView {
    task_overview: TaskOverview,
    count: usize,
    overview_view: u32,
    task_view: u32
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let overview_view = OverviewView::create()
            .task_view(id)
            .count(id)
            .task_overview(id)
            .build(ctx);
        let task_view = TaskView::create()
            .back_entity(overview_view.0)
            .visibility("collapsed")
            .build(ctx);

        self.name("MainView")
            .task_view(task_view.0)
            .task_overview(TaskOverview::default())
            .count(0)
            .child(overview_view)
            .child(task_view)
    }
}
