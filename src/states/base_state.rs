use orbtk::prelude::*;

use crate::{data::TaskOverview, keys::*};

/// Provides shared operations of `OverviewState` and `TaskState`.
pub trait BaseState {
    /// Fetches the text of a widget.
    fn fetch_text(&self, ctx: &mut Context, entity: Entity) -> Option<String> {
        let text = TextBox::text_clone(&ctx.get_widget(entity));

        if text.is_empty() {
            return None;
        }

        TextBox::text_set(&mut ctx.get_widget(entity), String::default());
        Some(text)
    }

    // Save the data.
    fn save(&self, registry: &mut Registry, ctx: &mut Context) {
        registry
            .get::<Settings>("settings")
            .save(
                PROP_TASK_OVERVIEW,
                ctx.widget().get::<TaskOverview>(PROP_TASK_OVERVIEW),
            )
            .unwrap();
    }
}
