// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    bell_number::register(runner);
    binomial_coefficient::register(runner);
    coprime_with::register(runner);
    factorial::register(runner);
    falling_factorial::register(runner);
    fibonacci::register(runner);
    gcd::register(runner);
    kronecker_symbol::register(runner);
    landau_function::register(runner);
    lcm::register(runner);
    primorial::register(runner);
    rising_factorial::register(runner);
    stirling_number_of_the_second_kind::register(runner);
    unsigned_stirling_number_of_the_first_kind::register(runner);
}

mod bell_number;
mod binomial_coefficient;
mod coprime_with;
mod factorial;
mod falling_factorial;
mod fibonacci;
mod gcd;
mod kronecker_symbol;
mod landau_function;
mod lcm;
mod primorial;
mod rising_factorial;
mod stirling_number_of_the_second_kind;
mod unsigned_stirling_number_of_the_first_kind;
