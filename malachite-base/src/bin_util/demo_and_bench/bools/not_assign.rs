// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::NotAssign;
use malachite_base::test_util::generators::bool_gen;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_bool_not_assign);
}

fn demo_bool_not_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut b in bool_gen().get(gm, config).take(limit) {
        let b_old = b;
        b.not_assign();
        println!("b := {b_old}; b.not_assign(); b = {b}");
    }
}
