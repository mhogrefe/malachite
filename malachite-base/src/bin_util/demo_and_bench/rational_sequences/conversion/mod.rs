// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    clone::register(runner);
    from_vec::register(runner);
    from_vecs::register(runner);
    to_vecs::register(runner);
}

mod clone;
mod from_vec;
mod from_vecs;
mod to_vecs;
