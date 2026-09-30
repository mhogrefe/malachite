// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::ifft_truncate_sqrt2::*;
use malachite_nz::test_util::generators::{large_type_gen_var_38, large_type_gen_var_42};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_ifft_butterfly_sqrt2);
    register_demo!(runner, demo_ifft_truncate_sqrt2);
}

fn demo_limbs_ifft_butterfly_sqrt2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut i1, mut i2, i, limbs, w) in large_type_gen_var_38().get(gm, config).take(limit) {
        let i1_old = i1.clone();
        let i2_old = i2.clone();
        let mut s = vec![0; limbs + 1];
        let mut t = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        limbs_ifft_butterfly_sqrt2(&mut s, &mut t, &mut i1, &mut i2, i, limbs, w, &mut temp);
        println!(
            "limbs_ifft_butterfly_sqrt2(_, _, {i1_old:?}, {i2_old:?}, {i}, {limbs}, {w}, _) = \
            ({s:?}, {t:?}, {i1:?}, {i2:?})"
        );
    }
}

fn demo_ifft_truncate_sqrt2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ii, n, w, trunc) in large_type_gen_var_42().get(gm, config).take(limit) {
        let ii_old = ii.clone();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        ifft_truncate_sqrt2(&mut ii, n, w, &mut t1, &mut t2, &mut temp, trunc);
        println!("ifft_truncate_sqrt2({ii_old:?}, {n}, {w}, _, _, _, {trunc}) = {ii:?}");
    }
}
