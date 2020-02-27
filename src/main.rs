use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;

// pub use self::main_state::*;
// pub use self::main_view::*;
// pub use self::task_list::*;
// pub use self::task_state::*;
// pub use self::task_view::*;

pub mod data;
pub mod keys;
pub mod main_state;
pub mod main_view;
pub mod overview_state;
pub mod overview_view;
pub mod task_state;
pub mod task_view;

static THEME: &str = include_str!("../theme/theme.css");

fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(THEME)
        .build()
}

fn main() {
    Application::from_name("flovanco.doit")
        .window(move |ctx| {
            Window::create()
                .title("Do it")
                .position((100.0, 100.0))
                .size(372.0, 768.0)
                .resizeable(true)
                .theme(get_theme())
                .child(main_view::MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
