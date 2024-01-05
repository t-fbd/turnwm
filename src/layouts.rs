use crate::{MAX_MAIN, RATIO, RATIO_STEP, OUTER_PX, INNER_PX};

use penrose::{
    stack,
    builtin::layout::{MainAndStack, Monocle, transformers::Gaps},
    core::layout::LayoutStack,
};

pub fn layouts() -> LayoutStack {
    stack!(
        // when using a bar
        // MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        // MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),
        // Monocle::boxed()
        // below for when no bar is used
        Gaps::wrap(MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP), OUTER_PX, INNER_PX),
        Gaps::wrap(MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP), OUTER_PX, INNER_PX),
        Gaps::wrap(Monocle::boxed(), OUTER_PX, INNER_PX)
    )
}
