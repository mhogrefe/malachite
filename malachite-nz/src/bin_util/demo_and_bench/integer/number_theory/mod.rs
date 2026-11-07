// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    binomial_coefficient::register(runner);
    extended_gcd::register(runner);
    falling_factorial::register(runner);
    gcd::register(runner);
    kronecker_symbol::register(runner);
    rising_factorial::register(runner);
    stirling_number_of_the_first_kind::register(runner);
}

mod binomial_coefficient;
mod extended_gcd;
mod falling_factorial;
mod gcd;
mod kronecker_symbol;
mod rising_factorial;
mod stirling_number_of_the_first_kind;
