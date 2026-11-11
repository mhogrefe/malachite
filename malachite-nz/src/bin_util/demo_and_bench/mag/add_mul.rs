// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::mag_triple_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_mag_add_mul_assign);
}

fn demo_mag_add_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut z, x, y) in mag_triple_gen().get(gm, config).take(limit) {
        let old_z = z;
        z.add_mul_assign(x, y);
        println!("z := {old_z}; z.add_mul_assign({x}, {y}); z = {z}");
    }
}
