// Styling
pub static STYLE_TOP_BAR: &str = "top_bar";
pub static STYLE_BOTTOM_BAR: &str = "bottom_bar";
pub static STYLE_HEADER: &str = "header";
pub static STYLE_BUTTON_ICON_ONLY: &str = "button_icon_only";
pub static STYLE_TRANSPARENT: &str = "transparent";
pub static STYLE_BUTTON_FLOAT: &str = "button_float";
pub static STYLE_BUTTON_TRANSPARENT: &str = "button_transparent";
pub static STYLE_LIST_VIEW_BORDER_LESS: &str = "list_view_border_less";
pub static STYLE_SEPARATOR: &str = "separator";
pub static STYLE_TEXT_BOX_INLINE: &str = "text_box_inline";
pub static STYLE_TEXT_BOX_HEADER: &str = "text_box_header";
pub static STYLE_OVERVIEW_GRID: &str = "overview_grid";
pub static STYLE_TASK_VIEW_GRID: &str = "task_view_grid";

// Ids
pub static ID_OVERVIEW_ADD_BUTTON: &str = "id_overview_add_button";
pub static ID_OVERVIEW_LIST_VIEW: &str = "id_overview_list_view";
pub static ID_OVERVIEW_TEXT_BOX: &str = "id_overview_text_box";
pub static ID_TASK_ITEMS_WIDGET: &str = "id_task_items_widget";
pub static ID_TASK_ADD_BUTTON: &str = "id_task_add_button";
pub static ID_TASK_TEXT_BOX: &str = "id_task_text_box";
pub static ID_TASK_HEADER_TEXT_BOX: &str = "id_task_header_text_box";

// Properties
pub static PROP_TASK_OVERVIEW: &str = "task_overview";
pub static PROP_COUNT: &str = "count";

// Key for application settings
#[cfg(debug_assertions)]
pub static APPLICATION: &str = "pluraltools.debug.pluralplanner";

#[cfg(not(debug_assertions))]
pub static APPLICATION: &str = "pluraltools.pluralplanner";
