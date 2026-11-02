// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    add_truncated::register(runner);
    compose_truncated::register(runner);
    compositional_inverse_truncated::register(runner);
    div_truncated::register(runner);
    inverse_truncated::register(runner);
    mul_truncated::register(runner);
    pow_truncated::register(runner);
    square_truncated::register(runner);
    sub_truncated::register(runner);
}

mod add_truncated;
mod compose_truncated;
mod compositional_inverse_truncated;
mod div_truncated;
mod inverse_truncated;
mod mul_truncated;
mod pow_truncated;
mod square_truncated;
mod sub_truncated;
