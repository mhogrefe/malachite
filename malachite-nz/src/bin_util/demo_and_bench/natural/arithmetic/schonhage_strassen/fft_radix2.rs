// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_radix2::*;
use malachite_nz::test_util::generators::{large_type_gen_var_37, large_type_gen_var_40};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_fft_butterfly);
    register_demo!(runner, demo_fft_radix2);
}

fn demo_limbs_fft_butterfly(gm: GenMode, config: &GenConfig, limit: usize) {
    for (i1, i2, i, limbs, w) in large_type_gen_var_37().get(gm, config).take(limit) {
        let mut s = vec![0; limbs + 1];
        let mut t = vec![0; limbs + 1];
        limbs_fft_butterfly(&mut s, &mut t, &i1, &i2, i, limbs, w);
        println!("limbs_fft_butterfly(_, _, {i1:?}, {i2:?}, {i}, {limbs}, {w}) = ({s:?}, {t:?})");
    }
}

fn demo_fft_radix2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ii, n, w) in large_type_gen_var_40().get(gm, config).take(limit) {
        let ii_old = ii.clone();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        fft_radix2(&mut ii, n, w, &mut t1, &mut t2);
        println!("fft_radix2({ii_old:?}, {n}, {w}, _, _) = {ii:?}");
    }
}
