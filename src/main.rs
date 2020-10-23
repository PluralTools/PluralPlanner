use orbtk::{
    prelude::*,
    theme_default::{THEME_DEFAULT, THEME_DEFAULT_FONTS},
    theming::config::ThemeConfig,
};

#[cfg(not(feature = "light"))]
use orbtk::theme_default::THEME_DEFAULT_COLORS_DARK;

#[cfg(feature = "light")]
use orbtk::theme_default::THEME_DEFAULT_COLORS_LIGHT;

pub mod base_state;
pub mod data;
pub mod keys;
pub mod main_view;
pub mod overview_state;
pub mod overview_view;
pub mod task_state;
pub mod task_view;

// --- THEME --

static THEME_DEFAULT_EXT: &str = include_str!("../assets/theme_default_ext.ron");

#[cfg(not(feature = "light"))]
static THEME_DEFAULT_COLORS_DARK_EXT: &str =
    include_str!("../assets/theme_default_colors_dark_ext.ron");

#[cfg(not(feature = "light"))]
fn theme() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(THEME_DEFAULT)
            .extend(ThemeConfig::from(THEME_DEFAULT_EXT))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK_EXT))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

#[cfg(feature = "light")]
static THEME_DEFAULT_COLORS_LIGHT_EXT: &str =
    include_str!("../assets/theme_default_colors_light_ext.ron");

#[cfg(feature = "light")]
fn theme() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(THEME_DEFAULT_EXT)
            .extend(ThemeConfig::from(THEME_DEFAULT))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_LIGHT_EXT))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

// --- THEME --

fn main() {
    Application::from_name(keys::APPLICATION)
        .theme(theme())
        .window(move |ctx| {
            Window::new()
                .title("PluralPlanner")
                .position((200, 200))
                .size(1000, 768)
                .resizeable(true)
                .child(main_view::MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
