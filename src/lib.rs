use penrose::{core::bindings::KeyEventHandler, x11rb::RustConn};

pub mod bar;
pub mod bindings;
pub mod layouts;
pub mod dzen_wrapper;
pub mod rofi_wrapper;

pub type KeyHandler = Box<dyn KeyEventHandler<RustConn>>;

pub const FONT: &str = "Hasklug Nerd Font Mono Regular 10";

pub const BLACK: u32 = 0x282828ff;
pub const WHITE: u32 = 0xebdbb2ff;
pub const GREY: u32 = 0x3c3836ff;
pub const BLUE: u32 = 0x458588ff;
pub const RED: u32 = 0xcc241dff;
pub const GREEN: u32 = 0x98971aff;
pub const YELLOW: u32 = 0xd79921ff;
pub const PURPLE: u32 = 0xb16286ff;
pub const ORANGE: u32 = 0xd65d0eff;

pub const SCREEN_WIDTH: u32 = 1920;
pub const SCREEN_HEIGHT: u32 = 1080;
// subtract width of current dzen instance / 2
pub const DZEN_CENTER_X: u32 = SCREEN_WIDTH / 2;
pub const DZEN_CENTER_Y: u32 = SCREEN_HEIGHT / 2;
// pub const DZEN_CENTER_Y_BAR: u32 = DZEN_CENTER_Y - BAR_HEIGHT_PX;

pub const DELTA: i32 = 10;
pub const MAX_ACTIVE_WINDOW_CHARS: usize = 40;
pub const BAR_HEIGHT_PX: u32 = 14;
pub const MAX_MAIN: u32 = 1;
pub const RATIO: f32 = 0.5;
pub const RATIO_STEP: f32 = 0.05;
pub const OUTER_PX: u32 = 5;
pub const INNER_PX: u32 = 5;

pub static TERM: &str = "st";
pub static LAUNCHER: &str = "rofi -show drun";
pub static EDITOR: &str = "st -e nvim";
pub static BROWSER: &str = "Vieb";
