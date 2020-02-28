use orbtk::prelude::*;

/// Provides shared operations of `OverviewState` and `TaskState`.
pub trait BaseState {
    /// Removes the focus of a text box
    fn remove_focus(&self, text_box: Entity, ctx: &mut Context) {
        ctx.get_widget(text_box)
            .set("visibility", Visibility::Collapsed);
    
        ctx.window().get_mut::<Global>("global").focused_widget = None;
        ctx.get_widget(text_box).set("focused", false);
        ctx.get_widget(text_box).update_theme_by_state(false);
    }

    /// Set the given text box to edit mode.
    fn edit_entry(&self, text_box: Entity, ctx: &mut Context) {
        if *ctx.get_widget(text_box).get::<Visibility>("visibility") == Visibility::Visible {
            self.remove_focus(text_box, ctx);
            return; 
        }
        ctx.get_widget(text_box)
            .set("visibility", Visibility::Visible);

        if let Some(old_focused_element) = ctx.window().get::<Global>("global").focused_widget {
            let mut old_focused_element = ctx.get_widget(old_focused_element);
            old_focused_element.set("focused", false);
            old_focused_element.update_theme_by_state(false);
        }
        ctx.window().get_mut::<Global>("global").focused_widget = Some(text_box);
        ctx.get_widget(text_box).set("focused", true);
        ctx.get_widget(text_box).update_theme_by_state(false);
    }

    /// Navigates to the given entity.
    fn navigate(&self, to: Entity, ctx: &mut Context) {
        if let Some(old_focused_element) = ctx.window().get::<Global>("global").focused_widget {
            let mut old_focused_element = ctx.get_widget(old_focused_element);
            old_focused_element.set("focused", false);
            old_focused_element.update_theme_by_state(false);
        }
        ctx.window().get_mut::<Global>("global").focused_widget = None;
        ctx.widget().set("visibility", Visibility::Collapsed);
        ctx.get_widget(to).set("visibility", Visibility::Visible);
    }

    /// Fetches the text of a widget.
    fn fetch_text(&self, ctx: &mut Context, entity: Entity) -> Option<String> {
        let mut widget = ctx.get_widget(entity);

        let entry = widget.get_mut::<String16>("text");
        if entry.is_empty() {
            return None;
        }

        let copy = entry.to_string();
        entry.clear();
        Some(copy)
    }
}
