// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    integer_mantissa_and_exponent::register(runner);
    raw_mantissa_and_exponent::register(runner);
    sci_mantissa_and_exponent::register(runner);
}

mod integer_mantissa_and_exponent;
mod raw_mantissa_and_exponent;
mod sci_mantissa_and_exponent;
