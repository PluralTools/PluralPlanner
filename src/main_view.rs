use orbtk::prelude::*;

use crate::MainState;

widget!(MainView<MainState>);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Grid::create()
                .rows(Rows::create().row("*").row(32.0).build())
                .child(TextBox::create().attach(Grid::row(1)).build(ctx))
                .build(ctx),
        )
    }
}
