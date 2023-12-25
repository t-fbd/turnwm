use crate::actions::power_menu;
use penrose::{
    builtin::actions::{modify_with, send_layout_message, spawn},
    builtin::layout::messages::{ExpandMain, IncMain, ShrinkMain},
    core::bindings::KeyEventHandler,
    map,
    x11rb::RustConn,
};
use std::collections::HashMap;

pub fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-w" => modify_with(|cs| cs.kill_focused()),
        "M-Tab" => modify_with(|cs| cs.next_screen()),
        "M-S-Tab" => modify_with(|cs| cs.drag_workspace_forward()),
        "M-grave" => modify_with(|cs| {
            if cs.current_tag() == "1" {
                cs.move_focused_to_tag("2");
            } else {
                cs.move_focused_to_tag("1");
            }
        }),
        "M-bracketright" => modify_with(|cs| cs.next_layout()),
        "M-bracketleft" => modify_with(|cs| cs.previous_layout()),
        "M-u" => send_layout_message(|| IncMain(1)),
        "M-d" => send_layout_message(|| IncMain(-1)),
        "M-l" => send_layout_message(|| ExpandMain),
        "M-h" => send_layout_message(|| ShrinkMain),
        "M-r" => spawn("rofi -show drun"),
        "M-Return" => spawn("alacritty"),
        "M-A-Escape" => power_menu(),
    };

    raw_bindings
}
