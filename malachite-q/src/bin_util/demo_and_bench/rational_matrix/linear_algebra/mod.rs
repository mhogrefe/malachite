// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    characteristic_polynomial::register(runner);
    determinant::register(runner);
    determinant_sign::register(runner);
    gram_schmidt::register(runner);
    inverse::register(runner);
    permanent::register(runner);
    reduced_row_echelon_form::register(runner);
    similarity_transform::register(runner);
    solve::register(runner);
    trace::register(runner);
}

mod characteristic_polynomial;
mod determinant;
mod determinant_sign;
mod gram_schmidt;
mod inverse;
mod permanent;
mod reduced_row_echelon_form;
mod similarity_transform;
mod solve;
mod trace;
