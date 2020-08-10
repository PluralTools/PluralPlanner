use orbtk::{
    prelude::*,
    theme::{COLORS_RON, FONTS_RON},
    theming::config::ThemeConfig,
};

#[cfg(not(feature = "light"))]
use orbtk::theme::DARK_THEME_RON;

#[cfg(feature = "light")]
use orbtk::theme::LIGHT_THEME_RON;

pub mod base_state;
pub mod data;
pub mod keys;
pub mod main_view;
pub mod overview_state;
pub mod overview_view;
pub mod task_state;
pub mod task_view;

// --- THEME --

#[cfg(not(feature = "light"))]
static DARK_EXT: &str = include_str!("../assets/dark_theme.ron");

#[cfg(not(feature = "light"))]
fn theme() -> Theme {
    Theme::from_config(
        ThemeConfig::from(DARK_THEME_RON)
            .extend(ThemeConfig::from(DARK_EXT))
            .extend(ThemeConfig::from(COLORS_RON))
            .extend(ThemeConfig::from(FONTS_RON)),
    )
}

#[cfg(feature = "light")]
static LIGHT_EXT: &str = include_str!("../assets/light_theme.ron");

#[cfg(feature = "light")]
fn theme() -> Theme {
    Theme::from_config(
        ThemeConfig::from(LIGHT_THEME_RON)
            .extend(ThemeConfig::from(LIGHT_EXT))
            .extend(ThemeConfig::from(COLORS_RON))
            .extend(ThemeConfig::from(FONTS_RON)),
    )
}

// --- THEME --

fn main() {
    Application::from_name(keys::APPLICATION)
        .theme(theme())
        .window(move |ctx| {
            Window::new()
                .title("PluralTasks")
                .position((100, 100))
                .size(372, 768)
                .resizeable(true)
                .child(main_view::MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
