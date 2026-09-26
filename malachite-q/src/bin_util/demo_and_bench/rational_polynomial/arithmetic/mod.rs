// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    content::register(runner);
    evaluate::register(runner);
    height::register(runner);
    make_monic::register(runner);
    is_unit::register(runner);
}

mod content;
mod evaluate;
mod height;
mod is_unit;
mod make_monic;
