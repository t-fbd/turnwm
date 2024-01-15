use crate::INNER_PX;
use crate::KeyHandler;
use crate::OUTER_PX;
use crate::BAR_HEIGHT_PX;
use crate::bar::status_bar;
use penrose::builtin::hooks::SpacingHook;
use penrose::core::State;
use penrose::x::XConn;
use penrose::{
    builtin::actions::key_handler,
    extensions::util::dmenu::{DMenu, DMenuConfig, MenuMatch},
};
use penrose_ui::StatusBar;
use std::process::exit;

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

// bar toggle shit

pub fn bar_off() -> KeyHandler {
    key_handler(|state, _| {
        let layout_hook = SpacingHook {
            inner_px: INNER_PX,
            outer_px: OUTER_PX,
            top_px: 0,
            bottom_px: 0,
        };
        toggle_off(state, layout_hook)?;
        Ok(())
    })
}


fn toggle_off<X: XConn + 'static>(state: &mut State<X>, hook: SpacingHook) -> penrose::Result<()> {
    state.remove_extension::<StatusBar<X>>();
    state.config.layout_hook = Some(Box::new(hook));
    Ok(())
}

pub fn bar_on() -> KeyHandler {
    key_handler(|state, _| {
        let layout_hook = SpacingHook {
            inner_px: INNER_PX,
            outer_px: OUTER_PX,
            top_px: 0,
            bottom_px: BAR_HEIGHT_PX,
        };
        toggle_on(state, layout_hook)?;
        Ok(())
    })
}

fn toggle_on<X: XConn + 'static>(state: &mut State<X>, hook: SpacingHook) -> penrose::Result<()> {
    state.config.layout_hook = Some(Box::new(hook));
    state.add_extension::<StatusBar<X>>(status_bar().unwrap());
    Ok(())
}
