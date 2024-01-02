use crate::{MAX_MAIN, RATIO, RATIO_STEP};

use penrose::{
    stack,
    builtin::layout::{MainAndStack, Monocle},
    core::layout::LayoutStack,
};

pub fn layouts() -> LayoutStack {
    stack!(
        MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),
        Monocle::boxed()
        // below for when no bar is used
        // Gaps::wrap(MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP), OUTER_PX, INNER_PX),
        // Gaps::wrap(MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP), OUTER_PX, INNER_PX),
        // Gaps::wrap(Monocle::boxed(), OUTER_PX, INNER_PX)
    )
}
