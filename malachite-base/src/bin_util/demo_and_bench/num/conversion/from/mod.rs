// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    convertible_from::register(runner);
    overflowing_from::register(runner);
    rounding_from::register(runner);
    saturating_from::register(runner);
    try_from_and_exact_from::register(runner);
    wrapping_from::register(runner);
}

mod convertible_from;
mod overflowing_from;
mod rounding_from;
mod saturating_from;
mod try_from_and_exact_from;
mod wrapping_from;
