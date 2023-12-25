use crate::KeyHandler;
use penrose::{
    builtin::actions::key_handler,
    extensions::util::dmenu::{DMenu, DMenuConfig, MenuMatch},
    util::spawn,
};

pub fn power_menu() -> KeyHandler {
    key_handler(|state, _| {
        let options = vec!["lock", "logout", "restart-wm", "shutdown", "reboot"];
        let screen_index = state.client_set.current_screen().index();
        let menu = DMenu::new(&DMenuConfig::with_prompt(">>> "), screen_index);

        if let Ok(MenuMatch::Line(_, choice)) = menu.build_menu(options) {
            match choice.as_ref() {
                "logout" => spawn("pkill -fi penrose"),
                "shutdown" => spawn("sudo shutdown -h now"),
                "reboot" => spawn("sudo reboot"),
                _ => unimplemented!(),
            }
        } else {
            Ok(())
        }
    })
}

