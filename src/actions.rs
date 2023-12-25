use crate::KeyHandler;
use penrose::{
    builtin::actions::key_handler,
    extensions::util::dmenu::{DMenu, DMenuConfig, MenuMatch},
};
use std::process::exit;

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

