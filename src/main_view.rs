use orbtk::prelude::*;

use crate::{
    data::TaskOverview,
    overview_view::OverviewView,
};

widget!(MainView {
    task_overview: TaskOverview,

    count: usize
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .task_overview(TaskOverview::default())
            .count(0)
            .child(
                OverviewView::create()
                    .count(id)
                    .task_overview(id)
                    .build(ctx),
            )
    }
}
