use crate::actions::{dzen_clients, dzen_handler, power_menu};
use crate::{DELTA, DZEN_CENTER_X};
use penrose::{
    builtin::actions::{
        floating::{float_focused, reposition, resize, sink_all, sink_focused},
        modify_with, send_layout_message, spawn,
    },
    builtin::layout::messages::{ExpandMain, IncMain, ShrinkMain},
    core::bindings::KeyEventHandler,
    map,
    x11rb::RustConn,
};
use std::collections::HashMap;

pub fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        // bar
        // "M-b" => toggle_bar(),

        // client management
        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-space" => modify_with(|cs| cs.swap_focus_and_head()),
        "M-S-space" => modify_with(|cs| cs.rotate_focus_to_head()),
        "M-w" => modify_with(|cs| cs.kill_focused()),

        // workspace management
        "M-Tab" => modify_with(|cs| {
            cs.toggle_tag();
            dzen_handler(
                cs.current_tag(),
                1,
                0,
                0, 
                15, 
                15, 
                Some(&["-ta c"])
            );
        }),

        // layouts
        "M-bracketright" => modify_with(|cs| {
            cs.next_layout();
            dzen_handler(
                cs.current_workspace()
                    .layout_name()
                    .as_str(), 
                1, 
                0, 
                0, 
                60, 
                15, 
                Some(&["-ta c"])
            );
        }),
        "M-bracketleft" => modify_with(|cs| {
            cs.previous_layout();
            dzen_handler(
                cs.current_workspace()
                    .layout_name()
                    .as_str(), 
                1, 
                0, 
                0, 
                60, 
                15, 
                Some(&["-ta c"])
            );
        }),
        "M-u" => send_layout_message(|| IncMain(1)),
        "M-d" => send_layout_message(|| IncMain(-1)),
        "M-l" => send_layout_message(|| ExpandMain),
        "M-h" => send_layout_message(|| ShrinkMain),

        // launchers
        "M-r" => spawn("rofi -show drun"),
        "M-Return" => spawn("alacritty"),
        "M-S-Return" => spawn("/home/turn/localbuilds/Vieb/dist/linux-unpacked/vieb"),
        "M-S-e" => spawn("alacritty -e nvim"),
        "M-Escape" => power_menu(),

        //time
        "M-a" => modify_with(|_|
            dzen_handler(
                "echo $(date +'%a %d %b %H:%M')",
                2,
                DZEN_CENTER_X - 100,
                0,
                200,
                15,
                Some(&["-ta c"])
            )
        ),

        //ram
        "M-S-a" => modify_with(|_|
            dzen_handler(
                "echo $(cat /proc/meminfo | head -4 | awk '{print $2}' | tr '\n' ' ' | awk '{print int(($1 - $3 - $4)/1024)}') MB",
                2,
                DZEN_CENTER_X - 50,
                0,
                100,
                15,
                Some(&["-ta c"])
            )
        ),


        // current layout
        "M-z" => modify_with(|cs| 
            dzen_handler(
                cs.current_workspace()
                    .layout_name()
                    .as_str(), 
                1, 
                0, 
                0, 
                60, 
                15, 
                Some(&["-ta c"])
            )
        ),

        "M-S-z" => dzen_clients(),
        "M-C-z" => modify_with(|_| 
            dzen_handler(
                "{echo Procs; ps -a; sleep 5}",
                0, 
                0, 
                0, 
                300, 
                15, 
                Some(
                    &[
                    "-l", 
                    "10", 
                    "-sa", 
                    "l", 
                    "-m",
                    "-e", 
                    "'button1=togglecollapse;button2=exit;button3=exit;button4=scrollup:3;button5=scrolldown:3;entertitle=uncollapse;leaveslave=collapse'"
                    ]
                )
            )
        ),

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
                modify_with(move |client_set| {
                    dzen_handler(
                        tag, 
                        1, 
                        0, 
                        0, 
                        15, 
                        15, 
                        Some(&["-ta c"])
                    );
                    client_set.pull_tag_to_screen(tag)
                }),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| {
                    dzen_handler(
                        tag, 
                        1, 
                        0, 
                        0, 
                        15, 
                        15, 
                        Some(&["-ta c"])
                    );
                    client_set.move_focused_to_tag(tag)
                }),
            ),
        ]);
    }

    raw_bindings
}
