use orbtk::prelude::*;

use crate::{data::TaskOverview, overview_view::OverviewView, task_view::TaskView};

widget!(MainView {
    task_overview: TaskOverview,
    count: usize,
    overview_view: u32,
    task_view: u32,
    master_detail: u32,
    back_visibility: Visibility
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let overview_view = OverviewView::new()
            .task_view(id)
            .master_detail(id)
            .count(id)
            .task_overview(id)
            .build(ctx);

        let task_view = TaskView::new()
            .overview(overview_view.0)
            .master_detail(id)
            .task_overview(id)
            .back_visibility(id)
            .build(ctx);

        let master_detail = MasterDetail::new()
            .responsive(true)
            .break_point(800)
            .master_detail(overview_view, task_view)
            .navigation_visibility(("back_visibility", id))
            .build(ctx);

        self.name("MainView")
            .task_view(task_view.0)
            .master_detail(master_detail.0)
            .task_overview(TaskOverview::default())
            .count(0)
            .child(master_detail)
    }
}
