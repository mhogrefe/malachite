// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_elements::register(runner);
    from_integer_vector::register(runner);
    from_natural_vector::register(runner);
    from_numerators_and_denominator::register(runner);
    from_unsigned_vector::register(runner);
    integer_vector_from_rational_vector::register(runner);
    natural_vector_from_rational_vector::register(runner);
    serde::register(runner);
    string::register(runner);
    to_elements::register(runner);
    to_numerators_and_denominator::register(runner);
    unsigned_vector_from_rational_vector::register(runner);
}

mod from_elements;
mod from_integer_vector;
mod from_natural_vector;
mod from_numerators_and_denominator;
mod from_unsigned_vector;
mod integer_vector_from_rational_vector;
mod natural_vector_from_rational_vector;
mod serde;
mod string;
mod to_elements;
mod to_numerators_and_denominator;
mod unsigned_vector_from_rational_vector;
