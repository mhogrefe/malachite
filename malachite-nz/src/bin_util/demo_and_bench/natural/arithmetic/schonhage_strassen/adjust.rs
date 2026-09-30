// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::adjust::limbs_fft_adjust;
use malachite_nz::test_util::generators::large_type_gen_var_34;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_fft_adjust);
}

fn demo_limbs_fft_adjust(gm: GenMode, config: &GenConfig, limit: usize) {
    for (i1, i, limbs, w) in large_type_gen_var_34().get(gm, config).take(limit) {
        let mut r = vec![0; limbs + 1];
        limbs_fft_adjust(&mut r, &i1, i, limbs, w);
        println!("limbs_fft_adjust(_, {i1:?}, {i}, {limbs}, {w}) = {r:?}");
    }
}
