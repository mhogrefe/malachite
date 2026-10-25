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
    determinant::register(runner);
    determinant_sign::register(runner);
    integer_matrix_mul::register(runner);
    inverse::register(runner);
    kronecker_product::register(runner);
    mul::register(runner);
    mul_integer_matrix::register(runner);
    mul_vector::register(runner);
    neg::register(runner);
    pow::register(runner);
    reconstruct::register(runner);
    reduced_row_echelon_form::register(runner);
    scalar_mul::register(runner);
    shl::register(runner);
    shr::register(runner);
    solve::register(runner);
    square::register(runner);
    sub::register(runner);
    sub_mul::register(runner);
    trace::register(runner);
    vector_mul::register(runner);
}

mod add;
mod add_mul;
mod canonical_primitive_part;
mod content;
mod content_chained;
mod determinant;
mod determinant_sign;
mod integer_matrix_mul;
mod inverse;
mod kronecker_product;
mod mul;
mod mul_integer_matrix;
mod mul_vector;
mod neg;
mod pow;
mod reconstruct;
mod reduced_row_echelon_form;
mod scalar_mul;
mod shl;
mod shr;
mod solve;
mod square;
mod sub;
mod sub_mul;
mod trace;
mod vector_mul;
