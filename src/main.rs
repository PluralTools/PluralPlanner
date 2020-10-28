use orbtk::prelude::*;

pub use self::theme::*;

pub mod data;
pub mod keys;
pub mod states;
pub mod theme;
pub mod views;

fn main() {
    Application::from_name(keys::APPLICATION)
        .theme(planner_theme_default())
        .window(move |ctx| {
            Window::new()
                .title("PluralPlanner")
                .position((200, 200))
                .size(1000, 768)
                .resizeable(true)
                .child(views::MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
