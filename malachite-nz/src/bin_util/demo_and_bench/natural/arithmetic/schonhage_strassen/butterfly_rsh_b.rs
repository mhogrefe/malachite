// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::butterfly_rsh_b::*;
use malachite_nz::test_util::generators::large_type_gen_var_36;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_butterfly_rsh_b);
}

fn demo_limbs_butterfly_rsh_b(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut i1, mut i2, limbs, x, y) in large_type_gen_var_36().get(gm, config).take(limit) {
        let i1_old = i1.clone();
        let i2_old = i2.clone();
        let mut t = vec![0; limbs + 1];
        let mut u = vec![0; limbs + 1];
        limbs_butterfly_rsh_b(&mut t, &mut u, &mut i1, &mut i2, limbs, x, y);
        println!(
            "limbs_butterfly_rsh_b(_, _, {i1_old:?}, {i2_old:?}, {limbs}, {x}, {y}) = \
            ({t:?}, {u:?}, {i1:?}, {i2:?})"
        );
    }
}
