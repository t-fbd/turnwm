use penrose::{
    builtin::hooks::SpacingHook,
    core::{
        bindings::parse_keybindings_with_xmodmap,
        Config, WindowManager,
    },
    extensions::hooks::add_ewmh_hooks,
    x11rb::RustConn,
};
use std::collections::HashMap;
use anyhow::Result;

use turnwm::{
    bindings::raw_key_bindings,
    layouts::layouts,
    bar::status_bar,
    OUTER_PX, INNER_PX, BAR_HEIGHT_PX,
};

fn main() -> Result<()> {
    // Set up the spacing hook to add gaps around windows when using a bar

    let layout_hook = SpacingHook {
        inner_px: INNER_PX,
        outer_px: OUTER_PX,
        top_px: 0,
        bottom_px: BAR_HEIGHT_PX,
    };

    // Create a new connection to the X server
    let conn = RustConn::new()?;

    // Set up keybindings
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;

    // Set up config

    let config = add_ewmh_hooks(Config {
        default_layouts: layouts(),
        layout_hook: Some(Box::new(layout_hook)),
        tags: vec![String::from("1"), String::from("2"), String::from("3")],
        ..Default::default()
    });

    // let config = Config {
    //     default_layouts: layouts(),
    //     ..Default::default()
    // };

    // Initialise the WindowManager with our config and keybindings
    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;
    let bar = status_bar()?;

    let wm = bar.add_to(wm);
    
    // Run the WindowManager main loop
    wm.run()?;

    Ok(())
}

// quick test to make sure that the keybindings are parsed correctly
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(raw_key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
