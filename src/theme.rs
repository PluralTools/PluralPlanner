use orbtk::{
    theme_default::prelude::*,
    theme_redox::prelude::*,
    theming::{config::ThemeConfig, prelude::*},
};

// -- [START] Default theme --

static THEME_DEFAULT_EXT: &str =
    include_str!("../assets/themes/theme_default/theme_default_ext.ron");
static THEME_DEFAULT_COLORS_DARK_EXT: &str =
    include_str!("../assets/themes/theme_default/theme_default_colors_dark_ext.ron");
static THEME_DEFAULT_COLORS_LIGHT_EXT: &str =
    include_str!("../assets/themes/theme_default/theme_default_colors_light_ext.ron");

/// Generates the default dark theme of planner.
pub fn planner_theme_default_dark() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(THEME_DEFAULT)
            .extend(ThemeConfig::from(THEME_DEFAULT_EXT))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK_EXT))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

/// Generates the default light theme of planner.
pub fn planner_theme_default_light() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(THEME_DEFAULT)
            .extend(ThemeConfig::from(THEME_DEFAULT_EXT))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_LIGHT))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_LIGHT_EXT))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

/// Generates the default theme of planner
pub fn planner_theme_default() -> Theme {
    planner_theme_default_dark()
}

// -- [END] Default theme --

// -- [START] Redox theme --

static THEME_REDOX_EXT: &str = include_str!("../assets/themes/theme_redox/theme_redox_ext.ron");
static THEME_REDOX_COLORS_EXT: &str =
    include_str!("../assets/themes/theme_redox/theme_redox_colors_ext.ron");

/// Generates the redox theme of planner.
pub fn planner_theme_redox() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(THEME_REDOX)
            .extend(ThemeConfig::from(THEME_REDOX_EXT))
            .extend(ThemeConfig::from(THEME_REDOX_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_REDOX_COLORS_EXT))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

// -- [END] Redox theme --
