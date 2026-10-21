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
    canonical_primitive_part::register(runner);
    content::register(runner);
    content_chained::register(runner);
    integer_matrix_mul::register(runner);
    mul::register(runner);
    mul_integer_matrix::register(runner);
    mul_vector::register(runner);
    neg::register(runner);
    scalar_mul::register(runner);
    shl::register(runner);
    shr::register(runner);
    sub::register(runner);
    trace::register(runner);
    vector_mul::register(runner);
}

mod add;
mod canonical_primitive_part;
mod content;
mod content_chained;
mod integer_matrix_mul;
mod mul;
mod mul_integer_matrix;
mod mul_vector;
mod neg;
mod scalar_mul;
mod shl;
mod shr;
mod sub;
mod trace;
mod vector_mul;
