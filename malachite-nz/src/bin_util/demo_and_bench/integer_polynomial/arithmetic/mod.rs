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
    add_truncated::register(runner);
    balanced_mod::register(runner);
    canonicalize_unit::register(runner);
    content::register(runner);
    content_chained::register(runner);
    div_exact::register(runner);
    evaluate::register(runner);
    height::register(runner);
    is_unit::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    neg::register(runner);
    scalar_add_mul::register(runner);
    scalar_mul::register(runner);
    sub::register(runner);
    sub_truncated::register(runner);
}

mod add;
mod add_truncated;
mod balanced_mod;
mod canonicalize_unit;
mod content;
mod content_chained;
mod div_exact;
mod evaluate;
mod height;
mod is_unit;
mod mod_op;
mod mod_power_of_2;
mod neg;
mod scalar_add_mul;
mod scalar_mul;
mod sub;
mod sub_truncated;
