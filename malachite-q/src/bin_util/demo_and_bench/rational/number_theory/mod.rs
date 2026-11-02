// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    approximate::register(runner);
    dedekind_sum::register(runner);
    denominators_in_closed_interval::register(runner);
    farey_neighbors::register(runner);
    gcd::register(runner);
    harmonic_number::register(runner);
    simplest_rational_in_interval::register(runner);
}

mod approximate;
mod dedekind_sum;
mod denominators_in_closed_interval;
mod farey_neighbors;
mod gcd;
mod harmonic_number;
mod simplest_rational_in_interval;
