// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_coefficients_asc::register(runner);
    from_integer::register(runner);
    from_natural_polynomial::register(runner);
    from_u64_polynomial::register(runner);
    serde::register(runner);
    string::register(runner);
}

mod from_coefficients_asc;
mod from_integer;
mod from_natural_polynomial;
mod from_u64_polynomial;
mod serde;
mod string;
