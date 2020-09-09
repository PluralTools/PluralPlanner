use orbtk::prelude::*;

use crate::{data::TaskOverview, keys::*};

/// Provides shared operations of `OverviewState` and `TaskState`.
pub trait BaseState {
    /// Navigates to the given entity.
    fn navigate(&self, to: Entity, ctx: &mut Context) {
        if let Some(old_focused_element) = ctx.window().get::<Global>("global").focused_widget {
            let mut old_focused_element = ctx.get_widget(old_focused_element);
            old_focused_element.set("focused", false);
            old_focused_element.update(false);
        }
        ctx.window().get_mut::<Global>("global").focused_widget = None;
        ctx.widget().set("visibility", Visibility::Collapsed);
        ctx.get_widget(to).set("visibility", Visibility::Visible);
    }

    /// Fetches the text of a widget.
    fn fetch_text(&self, ctx: &mut Context, entity: Entity) -> Option<String> {
        let mut widget = ctx.get_widget(entity);

        let entry = widget.get_mut::<String>("text");
        if entry.is_empty() {
            return None;
        }

        let copy = entry.to_string();
        entry.clear();
        Some(copy)
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
