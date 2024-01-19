use crate::dzen_wrapper::{Dzen, dzen_clients, DzenBuilder};
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
            Dzen::new(0, 0, 15, 15).set_p(1).set_title_align('c').build().run(
                format!("echo '{}'", cs.current_workspace().tag()).as_str(),
                "zsh"
            )
        }),

        // layouts
        "M-bracketright" => modify_with(|cs| {
            cs.next_layout();
            Dzen::new(0, 0, 15, 60).set_p(1).set_title_align('c').build().run(
                format!("echo '{}'", cs.current_workspace()
                    .layout_name()
                    .as_str()).as_str(),
                "zsh"
            )
        }),
        "M-bracketleft" => modify_with(|cs| {
            cs.previous_layout();
            Dzen::new(0, 0, 15, 60).set_p(1).set_title_align('c').build().run(
                format!("echo '{}'", cs.current_workspace()
                    .layout_name()
                    .as_str()).as_str(),
                "zsh"
            )
        }),
        "M-u" => send_layout_message(|| IncMain(1)),
        "M-d" => send_layout_message(|| IncMain(-1)),
        "M-l" => send_layout_message(|| ExpandMain),
        "M-h" => send_layout_message(|| ShrinkMain),

        // launchers
        "M-r" => spawn("rofi -show drun"),
        "M-Return" => spawn("alacritty"),
        "M-p" => spawn("/home/turn/localbuilds/Vieb/dist/linux-unpacked/vieb"),
        "M-S-Return" => spawn("alacritty -e tmux new-session -A -s master -n main"),
        "M-S-e" => spawn("alacritty -e nvim"),
        "M-S-question" => modify_with(|_| 
            Dzen::new(
                480,
                0,
                15,
                960
            ).set_p(0)
            .set_title_align('c')
            .set_slave_align('l')
            .set_lines(10)
            .add_menu()
            .set_e_easy()
            .build()
            .run(
                // remove terminal escape sequences
                r"(echo 'WM Log'; cat /home/turn/localbuilds/logs/turnwm.log | sed -u 's/\x1b\[[0-9;]*m//g'; sleep 5; done)",
                "zsh"
            )
        ),
        "M-Escape" => penrose::builtin::actions::exit(),

        //time
        "M-a" => modify_with(|_|
            Dzen::new(
                DZEN_CENTER_X - 100,
                0,
                15,
                200
            ).set_p(0)
            .set_title_align('c')
            .set_e("button1=exit;button3=exit")
            .build().run(
                "(while true; do echo $(date +'%a %d %b %H:%M'); sleep 60; done)",
                "zsh"
            )
        ),

        //ram
        "M-S-a" => modify_with(|_|
            Dzen::new(
                DZEN_CENTER_X - 50,
                0,
                15,
                100
            ).set_p(0)
            .set_title_align('c')
            .set_e("button1=exit;button3=exit")
            .build().run(
                "(while true; do echo $(cat /proc/meminfo | head -4 | awk '{print $2}' | tr '\n' ' ' | awk '{print int(($1 - $3 - $4)/1024)}') MB; sleep 10; done)",
                "zsh"
            )
        ),


        // current layout
        "M-z" => modify_with(|cs| 
            Dzen::new(
                DZEN_CENTER_X - 30,
                0,
                15,
                60
            ).set_p(1).set_title_align('c').build().run(
                format!("echo '{}'", cs.current_workspace()
                    .layout_name()
                    .as_str()).as_str(),
                "zsh"
            )
        ),

        "M-S-z" => {
            dzen_clients()
        },
        "M-C-z" => modify_with(|_| 
            Dzen::new(
                0,
                0,
                15,
                300
            ).set_p(0)
            .set_title_align('c')
            .set_slave_align('l')
            .set_lines(10)
            .add_menu()
            .set_e_easy()
            .build()
            .run(
                "{echo Procs; ps -a; sleep 5}",
                "zsh"
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
                    Dzen::new(
                        0,
                        0,
                        15,
                        15
                    ).set_p(1).set_title_align('c').build().run(
                        format!("echo '{}'", tag).as_str(),
                        "zsh"
                    );
                    client_set.pull_tag_to_screen(tag)
                }),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| {
                    Dzen::new(
                        0,
                        0,
                        15,
                        15
                    ).set_p(1).set_title_align('c').build().run(
                        format!("echo '{}'", tag).as_str(),
                        "zsh"
                    );
                    client_set.move_focused_to_tag(tag)
                }),
            ),
        ]);
    }

    raw_bindings
}
