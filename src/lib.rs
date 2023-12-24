use penrose::{core::bindings::KeyEventHandler, x11rb::RustConn};

pub mod actions;
pub mod bindings;
pub mod layouts;

pub type KeyHandler = Box<dyn KeyEventHandler<RustConn>>;

pub const MAX_MAIN: u32 = 1;
pub const RATIO: f32 = 0.5;
pub const RATIO_STEP: f32 = 0.05;
pub const OUTER_PX: u32 = 10;
pub const INNER_PX: u32 = 5;
