use orbtk::{
    prelude::*,
    theme::{COLORS_RON, DARK_THEME_RON, FONTS_RON},
    theming::config::ThemeConfig,
};

pub mod base_state;
pub mod data;
pub mod keys;
pub mod main_view;
pub mod overview_state;
pub mod overview_view;
pub mod task_state;
pub mod task_view;

static DARK_EXT: &'static str = include_str!("../assets/dark_theme.ron");

fn theme() -> Theme {
    Theme::from_config(
        ThemeConfig::from(DARK_THEME_RON)
            .extend(ThemeConfig::from(DARK_EXT))
            .extend(ThemeConfig::from(COLORS_RON))
            .extend(ThemeConfig::from(FONTS_RON)),
    )
}

fn main() {
    Application::from_name(keys::APPLICATION)
        .theme(theme())
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
