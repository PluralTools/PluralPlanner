use orbtk::{
    prelude::*,
    theme_default::{THEME_DEFAULT, THEME_DEFAULT_FONTS},
    theming::config::ThemeConfig,
};

#[cfg(not(feature = "light"))]
use orbtk::theme_default::THEME_DEFAULT_COLORS_DARK as THEME_DEFAULT_COLORS;

#[cfg(feature = "light")]
use orbtk::theme_default::THEME_DEFAULT_COLORS_LIGHT as THEME_DEFAULT_COLORS;

pub mod data;
pub mod keys;
pub mod states;
pub mod views;

// --- THEME --

static THEME_DEFAULT_EXT: &str = include_str!("../assets/theme_default_ext.ron");

#[cfg(not(feature = "light"))]
static THEME_DEFAULT_COLORS_EXT: &str = include_str!("../assets/theme_default_colors_dark_ext.ron");

#[cfg(feature = "light")]
static THEME_DEFAULT_COLORS_EXT: &str =
    include_str!("../assets/theme_default_colors_light_ext.ron");

fn theme() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(THEME_DEFAULT)
            .extend(ThemeConfig::from(THEME_DEFAULT_EXT))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_EXT))
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
                .child(views::MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
