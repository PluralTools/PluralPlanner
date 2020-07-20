use orbtk::prelude::*;

pub mod base_state;
pub mod data;
pub mod keys;
pub mod main_view;
pub mod overview_state;
pub mod overview_view;
pub mod task_state;
pub mod task_view;

fn main() {
    Application::from_name("flovanco.doit")
        .window(move |ctx| {
            Window::new()
                .title("Do it")
                .position((100, 100))
                .size(372, 768)
                .resizeable(true)
                .child(main_view::MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
