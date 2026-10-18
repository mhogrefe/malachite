// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_columns::register(runner);
    from_elements::register(runner);
    from_elements_column_major::register(runner);
    from_integer_matrix::register(runner);
    from_natural_matrix::register(runner);
    from_numerators_and_denominator::register(runner);
    from_rows::register(runner);
    from_unsigned_matrix::register(runner);
    integer_matrix_from_rational_matrix::register(runner);
    natural_matrix_from_rational_matrix::register(runner);
    #[cfg(feature = "serde")]
    serde::register(runner);
    string::register(runner);
    to_elements::register(runner);
    to_numerators_and_denominator::register(runner);
    unsigned_matrix_from_rational_matrix::register(runner);
}

mod from_columns;
mod from_elements;
mod from_elements_column_major;
mod from_integer_matrix;
mod from_natural_matrix;
mod from_numerators_and_denominator;
mod from_rows;
mod from_unsigned_matrix;
mod integer_matrix_from_rational_matrix;
mod natural_matrix_from_rational_matrix;
#[cfg(feature = "serde")]
mod serde;
mod string;
mod to_elements;
mod to_numerators_and_denominator;
mod unsigned_matrix_from_rational_matrix;
