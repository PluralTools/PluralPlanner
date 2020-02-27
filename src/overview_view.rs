use orbtk::prelude::*;

use crate::{OverviewState};
// use crate::{Action, MainState, TaskList};


widget!(OverviewView<OverviewState> {

});

impl Template for OverviewView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Overview")
    }
}