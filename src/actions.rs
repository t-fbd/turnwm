use crate::KeyHandler;
use penrose::{
    builtin::{actions::key_handler, hooks::SpacingHook},
    core::bindings::KeyEventHandler,
    extensions::util::dmenu::{DMenu, DMenuConfig, MenuMatch},
    x::XConnExt,
    x11rb::RustConn,
};

use crate::{bar::status_bar, BAR_HEIGHT_PX, INNER_PX, OUTER_PX};

use penrose_ui::{bar::startup_hook, StatusBar};

use std::process::exit;
use std::process::Command;
use tracing::info;

pub fn dzen_handler(text: &str, p: u32, x: u32, y: u32, w: u32, h: u32, args: Option<&[&str]>) {
    //kill any existing dzen2 processes
    Command::new("pkill")
        .arg("dzen2")
        .spawn()
        .expect("failed to execute process");

    if text.contains("echo") && args.is_none() {
        Command::new("zsh")
            .arg("-c")
            .arg(format!(
                "{} | dzen2 -p {} -x {} -y {} -w {} -h {}",
                text, p, x, y, w, h
            ))
            .spawn()
            .expect("failed to execute process");
        return;
    } else if text.contains("echo") && args.is_some() {
        Command::new("zsh")
            .arg("-c")
            .arg(format!(
                "{} | dzen2 -p {} -x {} -y {} -w {} -h {} {}",
                text,
                p,
                x,
                y,
                w,
                h,
                args.unwrap().join(" ")
            ))
            .spawn()
            .expect("failed to execute process");
        return;
    }

    if args.is_none() {
        Command::new("zsh")
            .arg("-c")
            .arg(format!(
                "echo '{}' | dzen2 -p {} -x {} -y {} -w {} -h {}",
                text, p, x, y, w, h
            ))
            .spawn()
            .expect("failed to execute process");
        return;
    } else {
        Command::new("zsh")
            .arg("-c")
            .arg(format!(
                "echo '{}' | dzen2 -p {} -x {} -y {} -w {} -h {} {}",
                text,
                p,
                x,
                y,
                w,
                h,
                args.unwrap().join(" ")
            ))
            .spawn()
            .expect("failed to execute process");
    }
}

// dzen call to display all currently running clients and their tags
pub fn dzen_clients() -> KeyHandler {
    key_handler(|state, x: &RustConn| {
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
            text.push_str("No clients running");
            dzen_handler(
                &text, 
                1, 
                0, 
                0, 
                200, 
                15, 
                None
            );
        } else {
            let mut lines = text.lines().count();

            if lines > 10 {
                lines = 10;
            }

            let lines = lines.to_string();
            let mut new_text = String::new();
            info!("text: {}", text);

            for line in text.lines() {
                if line.len() > 50 {
                    let mut new_line = line[0..47].to_string();
                    new_line.push_str("...");
                    new_text.push_str(&new_line);
                } else {
                    new_text.push_str(line);
                }
            }
            
            let text = "CLIENTS>>>\n".to_owned() + &new_text;

            dzen_handler(
                &text.as_str(), 
                0, 
                0, 
                0, 
                200, 
                15, 
                Some(&["-l", &lines, "-sa", "l", "-m", "-e", "'button1=togglecollapse;button2=exit;button3=exit;button4=scrollup:3;button5=scrolldown:3;entertitle=uncollapse;leaveslave=collapse'"]));
        }
        Ok(())
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
