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
    crt::register(runner);
    kronecker_product::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    mul::register(runner);
    mul_vector::register(runner);
    multi_crt::register(runner);
    multi_mod::register(runner);
    pow::register(runner);
    neg::register(runner);
    scalar_mul::register(runner);
    shl::register(runner);
    square::register(runner);
    sub::register(runner);
    trace::register(runner);
    vector_mul::register(runner);
}

mod add;
mod balanced_mod;
mod balanced_mod_power_of_2;
mod canonical_primitive_part;
mod content;
mod content_chained;
mod crt;
mod kronecker_product;
mod mod_op;
mod mod_power_of_2;
mod mul;
mod mul_vector;
mod multi_crt;
mod multi_mod;
mod neg;
mod pow;
mod scalar_mul;
mod shl;
mod square;
mod sub;
mod trace;
mod vector_mul;
