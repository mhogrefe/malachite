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
    balanced_mod::register(runner);
    balanced_mod_power_of_2::register(runner);
    canonical_primitive_part::register(runner);
    content::register(runner);
    content_chained::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    mul_vector::register(runner);
    neg::register(runner);
    scalar_mul::register(runner);
    shl::register(runner);
    sub::register(runner);
    vector_mul::register(runner);
}

mod add;
mod balanced_mod;
mod balanced_mod_power_of_2;
mod canonical_primitive_part;
mod content;
mod content_chained;
mod mod_op;
mod mod_power_of_2;
mod mul_vector;
mod neg;
mod scalar_mul;
mod shl;
mod sub;
mod vector_mul;
