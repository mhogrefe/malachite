// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    canonical_primitive_part::register(runner);
    content::register(runner);
    content_chained::register(runner);
    div_exact::register(runner);
    entrywise_div_round::register(runner);
    entrywise_shr_round::register(runner);
    height::register(runner);
    scalar_div::register(runner);
    shr::register(runner);
}

mod canonical_primitive_part;
mod content;
mod content_chained;
mod div_exact;
mod entrywise_div_round;
mod entrywise_shr_round;
mod height;
mod scalar_div;
mod shr;
