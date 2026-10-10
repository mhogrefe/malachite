// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    add::register(runner);
    add_mul::register(runner);
    canonical_primitive_part::register(runner);
    canonicalize_sign::register(runner);
    content::register(runner);
    entrywise_abs::register(runner);
    height::register(runner);
    l1_norm::register(runner);
    neg::register(runner);
    scalar_div::register(runner);
    scalar_mul::register(runner);
    shl::register(runner);
    shr::register(runner);
    sub::register(runner);
    sub_mul::register(runner);
}

mod add;
mod add_mul;
mod canonical_primitive_part;
mod canonicalize_sign;
mod content;
mod entrywise_abs;
mod height;
mod l1_norm;
mod neg;
mod scalar_div;
mod scalar_mul;
mod shl;
mod shr;
mod sub;
mod sub_mul;
