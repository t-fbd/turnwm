use penrose::{
    // builtin::hooks::SpacingHook,
    core::{bindings::parse_keybindings_with_xmodmap, Config, WindowManager},
    // extensions::hooks::add_ewmh_hooks,
    extensions::hooks::startup::SpawnOnStartup,
    x11rb::RustConn,
};

use std::collections::HashMap;
use tracing_subscriber::prelude::*;

use turnwm::{
    bindings::raw_key_bindings,
    layouts::layouts,
    BLACK, GREY
    // bar::status_bar,
    // INNER_PX, OUTER_PX, BAR_HEIGHT_PX,
};

fn main() -> penrose::Result<()> {
    // Set up tracing
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    // Set up the spacing hook to add gaps around windows when using a bar
    // let layout_hook = SpacingHook {
    //     inner_px: INNER_PX,
    //     outer_px: OUTER_PX,
    //     top_px: 0,
    //     bottom_px: 0,
    // };

    let conn = RustConn::new()?;

    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;

    let config = Config {
        default_layouts: layouts(),
        startup_hook: Some(SpawnOnStartup::boxed("/home/turn/.config/turnwm/scripts/auto.sh")),
        // layout_hook: Some(Box::new(layout_hook)),
        normal_border: BLACK.into(),
        focused_border: GREY.into(),
        tags: vec![String::from("1"), String::from("2")],
        ..Default::default()
    };

    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;

    wm.run()?;

    Ok(())
}
