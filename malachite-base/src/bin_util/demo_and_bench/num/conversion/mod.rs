// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    digits::register(runner);
    from::register(runner);
    half::register(runner);
    is_integer::register(runner);
    mantissa_and_exponent::register(runner);
    slice::register(runner);
    string::register(runner);
}

mod digits;
mod from;
mod half;
mod is_integer;
mod mantissa_and_exponent;
mod slice;
mod string;
