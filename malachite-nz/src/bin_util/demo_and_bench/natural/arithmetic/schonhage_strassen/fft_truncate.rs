// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_truncate::*;
use malachite_nz::test_util::generators::large_type_gen_var_41;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_fft_truncate1);
    register_demo!(runner, demo_fft_truncate);
}

fn demo_fft_truncate1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ii, n, w, trunc) in large_type_gen_var_41().get(gm, config).take(limit) {
        let ii_old = ii.clone();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        fft_truncate1(&mut ii, n, w, &mut t1, &mut t2, trunc);
        println!("fft_truncate1({ii_old:?}, {n}, {w}, _, _, {trunc}) = {ii:?}");
    }
}

fn demo_fft_truncate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ii, n, w, trunc) in large_type_gen_var_41().get(gm, config).take(limit) {
        let ii_old = ii.clone();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        fft_truncate(&mut ii, n, w, &mut t1, &mut t2, trunc);
        println!("fft_truncate({ii_old:?}, {n}, {w}, _, _, {trunc}) = {ii:?}");
    }
}
