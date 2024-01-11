use crate::{MAX_MAIN, RATIO, RATIO_STEP,
    //OUTER_PX, INNER_PX
};

use penrose::{
    stack,
    // add builtin::layout::Gaps for when no bar is used
    builtin::layout::{MainAndStack, Monocle, CenteredMain},
    core::layout::{LayoutStack, Layout},
    extensions::layout::Conditional,
};

pub fn layouts() -> LayoutStack {
    stack!(
        // when using a bar
        MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),
        flex_tall(),
        flex_wide(),
        Monocle::boxed()
        // below for when no bar is used
        // Gaps::wrap(MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP), OUTER_PX, INNER_PX),
        // Gaps::wrap(MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP), OUTER_PX, INNER_PX),
        // Gaps::wrap(Monocle::boxed(), OUTER_PX, INNER_PX),
        // true fullscreen
        // Monocle::boxed()
    )
}

fn flex_tall() -> Box<dyn Layout> {
    Conditional::boxed(
        "FlexTall",
        MainAndStack::side_unboxed(MAX_MAIN, RATIO, RATIO_STEP, false),
        CenteredMain::vertical_unboxed(MAX_MAIN, RATIO, RATIO_STEP),
        |_, r| r.w <= 1400,
    )
}

fn flex_wide() -> Box<dyn Layout> {
    Conditional::boxed(
        "FlexWide",
        MainAndStack::bottom_unboxed(MAX_MAIN, RATIO, RATIO_STEP, false),
        CenteredMain::horizontal_unboxed(MAX_MAIN, RATIO, RATIO_STEP),
        |_, r| r.w <= 1400,
    )
}
