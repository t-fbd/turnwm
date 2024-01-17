use crate::{BAR_HEIGHT_PX, BLACK, FONT, GREY, MAX_ACTIVE_WINDOW_CHARS, RED, WHITE};
use penrose::{x::XConn, Color};
use penrose_ui::{
    bar::{
        widgets::{current_date_and_time, ActiveWindowName, CurrentLayout, Workspaces},
        Position, StatusBar,
    },
    core::TextStyle,
};

// Mostly the example dwm bar from the main repo but recreated here so it's easier to tinker
// with and add in debug widgets when needed.
pub fn status_bar<X: XConn>() -> penrose_ui::Result<StatusBar<X>> {
    let highlight: Color = RED.into();
    let title_highlight: Color = BLACK.into();
    let empty_ws: Color = GREY.into();

    let style = TextStyle {
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2, 2),
    };

    let padded_style = TextStyle {
        padding: (2, 2),
        ..style
    };

    StatusBar::try_new(
        Position::Bottom,
        BAR_HEIGHT_PX,
        style.bg.unwrap_or_else(|| 0x000000.into()),
        FONT,
        8,
        vec![
            Box::new(Workspaces::new(style, highlight, empty_ws)),
            Box::new(CurrentLayout::new(style)),
            Box::new(ActiveWindowName::new(
                MAX_ACTIVE_WINDOW_CHARS,
                TextStyle {
                    bg: Some(title_highlight),
                    padding: (4, 4),
                    ..style
                },
                true,
                false,
            )),
            Box::new(current_date_and_time(padded_style)),
        ],
    )
}
