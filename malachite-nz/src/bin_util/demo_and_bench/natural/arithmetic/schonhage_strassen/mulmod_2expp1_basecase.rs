// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1_basecase::*;
use malachite_nz::test_util::generators::large_type_gen_var_48;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_mul_mod_2expp1_basecase);
}

fn demo_limbs_mul_mod_2expp1_basecase(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys, c, b) in large_type_gen_var_48().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let mut tp = vec![0; xs.len() << 1];
        let carry = limbs_mul_mod_2expp1_basecase(&mut xs, ys.as_deref(), c, b, &mut tp);
        println!(
            "limbs_mul_mod_2expp1_basecase({xs_old:?}, {ys:?}, {c}, {b}, _) = ({carry}, {xs:?})"
        );
    }
}
