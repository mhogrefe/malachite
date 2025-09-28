// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    factor::register(runner);
    is_power::register(runner);
    is_prime::register(runner);
    is_square::register(runner);
    primes::register(runner);
    prime_sieve::register(runner);
    primitive_root_prime::register(runner);
}

mod factor;
mod is_power;
mod is_prime;
mod is_square;
mod prime_sieve;
mod primes;
mod primitive_root_prime;
