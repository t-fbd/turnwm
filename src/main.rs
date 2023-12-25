use penrose::{
    core::{
        bindings::parse_keybindings_with_xmodmap,
        Config, WindowManager,
    },
    x11rb::RustConn,
    Result,
};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

use turnwm::{
    bindings::raw_key_bindings,
    layouts::layouts,
};

fn main() -> Result<()> {
    // Set up logging to stdout
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    // Create a new connection to the X server
    let conn = RustConn::new()?;

    // Set up keybindings
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;

    // Set up the WindowManager config
    let config = Config {
        default_layouts: layouts(),
        ..Default::default()
    };

    // Initialise the WindowManager with our config and keybindings
    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;

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
