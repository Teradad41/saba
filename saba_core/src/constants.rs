// レイアウトに関する定数
pub static WINDOW_WIDTH: i64 = 600;
pub static WINDOW_HEIGHT: i64 = 400;
pub static WINDOW_PADDING: i64 = 5;

// 色に関する定数
pub static WHITE: u32 = 0xFFFFFFFF;
pub static LIGHT_GRAY: u32 = 0xFFd3d3d3;
pub static GRAY: u32 = 0xFF808080;
pub static DARK_GRAY: u32 = 0xFF5A5A5A;
pub static BLACK: u32 = 0xFF000000;

pub static ADDRESS_BAR_HEIGHT: i64 = 20;

pub static WINDOW_INIT_X_POS: i64 = 30;
pub static WINDOW_INIT_Y_POS: i64 = 50;

// noli ライブラリに定義されている定数
pub static TOOLBAR_HEIGHT: i64 = 26;
pub static TITLE_BAR_HEIGHT: i64 = 24;

pub static CONTENT_AREA_WIDTH: i64 = WINDOW_WIDTH - WINDOW_PADDING * 2;
pub static CONTENT_AREA_HEIGHT: i64 =
    WINDOW_HEIGHT - TITLE_BAR_HEIGHT - TOOLBAR_HEIGHT - WINDOW_PADDING * 2;

pub static CHAR_WIDTH: i64 = 8;
pub static CHAR_HEIGHT: i64 = 16;
pub static CHAR_WITH_PADDING: i64 = CHAR_HEIGHT + 4;
