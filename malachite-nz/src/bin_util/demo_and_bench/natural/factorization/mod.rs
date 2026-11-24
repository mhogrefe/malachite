// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    is_lucas_probable_prime::register(runner);
    is_power::register(runner);
    is_probable_prime::register(runner);
    is_square::register(runner);
    is_strong_probable_prime::register(runner);
    primes::register(runner);
}

mod is_lucas_probable_prime;
mod is_power;
mod is_probable_prime;
mod is_square;
mod is_strong_probable_prime;
mod primes;
