use crate::{MAX_MAIN, RATIO, RATIO_STEP, OUTER_PX, INNER_PX};

use penrose::{
    stack,
    builtin::layout::{MainAndStack, Monocle, transformers::Gaps},
    core::layout::LayoutStack,
};

pub fn layouts() -> LayoutStack {
    stack!(
        Gaps::wrap(MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP), OUTER_PX, INNER_PX),
        Gaps::wrap(MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP), OUTER_PX, INNER_PX),
        Gaps::wrap(Monocle::boxed(), OUTER_PX, INNER_PX)
    )
}
