use orbtk::prelude::*;

pub use self::main_state::*;
pub use self::main_view::*;
pub use self::task_list::*;

mod main_state;
mod main_view;
mod task_list;

fn main() {
    Application::new()
        .window(move |ctx| {
            Window::create()
                .title("Do it")
                .position((100.0, 100.0))
                .size(372.0, 768.0)
                .child(MainView::create().margin(8.0).build(ctx))
                .build(ctx)
        })
        .run();
}
