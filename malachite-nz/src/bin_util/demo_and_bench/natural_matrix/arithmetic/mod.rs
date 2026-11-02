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
    content::register(runner);
    content_chained::register(runner);
    div_exact::register(runner);
    entrywise_shl_round::register(runner);
    entrywise_shr_round::register(runner);
    kronecker_product::register(runner);
    mul::register(runner);
    mul_vector::register(runner);
    pow::register(runner);
    scalar_mul::register(runner);
    shl::register(runner);
    shr::register(runner);
    square::register(runner);
    vector_mul::register(runner);
}

mod add;
mod add_mul;
mod canonical_primitive_part;
mod content;
mod content_chained;
mod div_exact;
mod entrywise_shl_round;
mod entrywise_shr_round;
mod kronecker_product;
mod mul;
mod mul_vector;
mod pow;
mod scalar_mul;
mod shl;
mod shr;
mod square;
mod vector_mul;
