use crate::actions::power_menu;
use penrose::{
    builtin::actions::{
        modify_with, send_layout_message, spawn,
        floating::{float_focused, reposition, resize, sink_all, sink_focused},
    },
    builtin::layout::messages::{ExpandMain, IncMain, ShrinkMain},
    core::bindings::KeyEventHandler,
    map,
    x11rb::RustConn,
};
use std::collections::HashMap;

const DELTA: i32 = 10;

pub fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        // client management
        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-space" => modify_with(|cs| cs.swap_focus_and_head()),
        "M-S-space" => modify_with(|cs| cs.rotate_focus_to_head()), 
        "M-w" => modify_with(|cs| cs.kill_focused()),

        // workspace management
        "M-Tab" => modify_with(|cs| cs.toggle_tag()),

        // layouts
        "M-bracketright" => modify_with(|cs| cs.next_layout()),
        "M-bracketleft" => modify_with(|cs| cs.previous_layout()),
        "M-u" => send_layout_message(|| IncMain(1)),
        "M-d" => send_layout_message(|| IncMain(-1)),
        "M-l" => send_layout_message(|| ExpandMain),
        "M-h" => send_layout_message(|| ShrinkMain),

        // launchers
        "M-r" => spawn("rofi -show drun"),
        "M-Return" => spawn("alacritty"),
        "M-A-Escape" => power_menu(),

        // Floating management
        "M-C-f" => float_focused(),
        "M-C-s" => sink_focused(),
        "M-C-S-s" => sink_all(),
        // Floating resize
        "M-C-Right" => resize(DELTA, 0),
        "M-C-Left" => resize(-DELTA, 0),
        "M-C-Up" => resize(0, -DELTA),
        "M-C-Down" => resize(0, DELTA),
        // Floating position
        "M-C-l" => reposition(DELTA, 0),
        "M-C-h" => reposition(-DELTA, 0),
        "M-C-k" => reposition(0, -DELTA),
        "M-C-j" => reposition(0, DELTA),
    };

    // more workspace management
    for tag in &["1", "2", "3"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.pull_tag_to_screen(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}
