use penrose::{
    builtin::{actions::key_handler, hooks::SpacingHook},
    core::bindings::KeyEventHandler,
    extensions::util::dmenu::{DMenu, DMenuConfig, MenuMatch},
    x::XConnExt,
    x11rb::RustConn,
};

use crate::{
    KeyHandler,
    bar::status_bar,
    BAR_HEIGHT_PX, INNER_PX, OUTER_PX,
    dzen_wrapper::{Dzen, DzenBuilder},
};

use penrose_ui::{bar::startup_hook, StatusBar};

use std::process::exit;
use tracing::info;


// dzen call to display all currently running clients and their tags
pub fn dzen_clients() -> KeyHandler {
    key_handler(move |state, x: &RustConn| {
        let mut text = String::new();

        for w in state.client_set.workspaces() {
            let tag = w.tag();
            let clients: Vec<_> = w.clients().collect::<Vec<_>>();
            for xid in clients {
                let name = x.window_title(*xid).unwrap();
                text.push_str(&format!("{}: {} [{}]\n", tag, name, xid));
            }
        }
        if text.is_empty() {
            let dzen = Dzen::new(
                0,
                0,
                15,
                300
            ).set_p(1).set_title_align('c');
            text.push_str("echo 'No clients running'");
            dzen.build().run(&text, "zsh");
            Ok(())
        } else {
            let mut lines = text.lines().count();

            if lines > 10 {
                lines = 10;
            }

            let dzen = Dzen::new(
                0,
                0,
                15,
                300
            ).set_p(0)
                .set_title_align('c')
                .set_slave_align('l')
                .set_lines(lines as u32)
                .add_menu()
                .set_e("button1=togglecollapse;button2=exit;button3=exit;button4=scrollup:3;button5=scrolldown:3;entertitle=uncollapse;leaveslave=collapse");

            
            let text = "CLIENTS>>>\n".to_owned() + &text;
            info!("text: {}", text);

            dzen.build().run(format!("echo '{}'", text).as_str(), "zsh");
            Ok(())

        }
    })
}

// implement more menu options eventually
pub fn power_menu() -> KeyHandler {
    key_handler(|state, _| {
        let options = vec!["logout"];
        let screen_index = state.client_set.current_screen().index();
        let menu = DMenu::new(&DMenuConfig::with_prompt("POWER MENU >>> "), screen_index);

        if let Ok(MenuMatch::Line(_, choice)) = menu.build_menu(options) {
            match choice.as_ref() {
                "logout" => exit(0),
                _ => unimplemented!(),
            }
        } else {
            Ok(())
        }
    })
}

pub fn toggle_bar() -> Box<dyn KeyEventHandler<RustConn>> {
    key_handler(|state, x: &RustConn| {
        if state.extension::<StatusBar<RustConn>>().is_ok() {
            info!("Hiding status bar");
            let layout_hook = SpacingHook {
                inner_px: INNER_PX,
                outer_px: OUTER_PX,
                top_px: 0,
                bottom_px: 0,
            };

            state.remove_extension::<StatusBar<RustConn>>();
            state.config.layout_hook = Some(Box::new(layout_hook));
        } else {
            info!("Showing status bar");
            let layout_hook = SpacingHook {
                inner_px: INNER_PX,
                outer_px: OUTER_PX,
                top_px: 0,
                bottom_px: BAR_HEIGHT_PX,
            };

            state.add_extension::<StatusBar<RustConn>>(status_bar().unwrap());
            state.config.layout_hook = Some(Box::new(layout_hook));
            // You need this to init the status bar window and state
            startup_hook(state, x)?;
        }

        x.refresh(state)
    })
}
