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
    from_numerators_and_denominator::register(runner);
    serde::register(runner);
    string::register(runner);
    to_elements::register(runner);
    to_numerators_and_denominator::register(runner);
}

mod from_elements;
mod from_numerators_and_denominator;
mod serde;
mod string;
mod to_elements;
mod to_numerators_and_denominator;
