use crate::{
    dzen_wrapper::{
        dzen_clients, Dzen, DzenBuilder
    }, BROWSER, DELTA, DZEN_CENTER_X, EDITOR, LAUNCHER, SCREEN_WIDTH, TERM
    // rofi_wrapper::rofi_clients,
    // simpletext::{self, SimpleText, SimpleTextOption, SimpleTextBuilder},
};
use penrose::{
    builtin::{actions::{
        floating::{float_focused, reposition, resize, sink_all, sink_focused, MouseDragHandler, MouseResizeHandler},
        modify_with, send_layout_message, spawn,
    }, layout::messages::{
            ExpandMain, IncMain, ShrinkMain
        }},
    core::bindings::{KeyEventHandler, MouseEventHandler, MouseState, click_handler},
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
        "M-r" => spawn(LAUNCHER),
        "M-Return" => spawn(TERM),
        "M-p" => spawn(BROWSER),
        "M-S-Return" => spawn("st -e tmux new-session -A -s master -n main"),
        "M-S-e" => spawn(EDITOR),
        "M-S-l" => modify_with(|_| 
            Dzen::new(
                0,
                0,
                15,
                SCREEN_WIDTH
            ).set_p(0)
            .set_title_align('c')
            .set_slave_align('l')
            .set_lines(10)
            .add_menu()
            .set_e_easy()
            .build()
            .run(
                // remove terminal escape sequences
                r"(while true; do echo 'WM Log'; tail -f /home/turn/.local/builds/logs/turnwm.log | sed -u 's/\x1b\[[0-9;]*m//g'; done)",
                "zsh"
            )
        ),
        "M-Escape" => penrose::builtin::actions::exit(),
        // "M-S-Escape" => key_handler(|_, _| {
        //     let mut t = SimpleText::new();
        //     t.set_option(SimpleTextOption::Content("turnwm"))
        //         .set_option(SimpleTextOption::Persist(10));
        //     simpletext::draw(t)
        // }),

        "M-S-question" => modify_with(|_|
            Dzen::new(
                DZEN_CENTER_X - 200,
                0,
                15,
                400
            ).set_p(0)
            .set_title_align('c')
            .set_slave_align('l')
            .set_lines(30)
            .add_menu()
            .set_e_easy()
            .build()
            .run("echo 'Bindings
M-j: focus down
M-k: focus up
M-S-j: swap down
M-S-k: swap up
M-space: swap focus and head
M-S-space: rotate focus to head
M-w: kill focused
M-Tab: toggle tag
M-bracketright: next layout
M-bracketleft: previous layout
M-u: increase main size
M-d: decrease main size
M-l: expand main
M-h: shrink main
M-r: launcher
M-Return: terminal
M-p: browser
M-S-Return: new terminal session
M-S-e: editor
M-S-l: log
M-S-z: clients
M-C-z: procs
M-Escape: exit
M-S-question: keybindings

Mouse bindings
M-S-Left: drag
M-S-Right: resize
M-S-Middle: sink'",
                "zsh"
            )
        ),


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
            // rofi_clients()
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
                modify_with(move |cs| {
                    Dzen::new(
                        DZEN_CENTER_X - 100,
                        0,
                        15,
                        240
                    ).set_p(1).set_title_align('c').build().run(
                        format!("echo 'Client Moved: {} ==> {}'", cs.current_workspace().tag(), tag).as_str(),
                        "zsh"
                    );
                    cs.move_focused_to_tag(tag)
                }),
            ),
        ]);
    }

    raw_bindings
}

pub fn mouse_bindings() -> HashMap<MouseState, Box<dyn MouseEventHandler<RustConn>>> {
    use penrose::core::bindings::{
        ModifierKey::{Meta, Shift},
        MouseButton::{Left, Middle, Right},
    };

    map! {
        map_keys: |(button, modifiers)| MouseState { button, modifiers };

        (Left, vec![Shift, Meta]) => MouseDragHandler::boxed_default(),
        (Right, vec![Shift, Meta]) => MouseResizeHandler::boxed_default(),
        (Middle, vec![Shift, Meta]) => click_handler(sink_focused()),
    }
}

