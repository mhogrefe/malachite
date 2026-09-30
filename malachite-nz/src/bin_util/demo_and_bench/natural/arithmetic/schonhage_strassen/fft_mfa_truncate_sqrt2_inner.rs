// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_mfa_truncate_sqrt2_inner::*;
use malachite_nz::test_util::generators::large_type_gen_var_47;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_fft_mfa_truncate_sqrt2_inner);
}

fn demo_fft_mfa_truncate_sqrt2_inner(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ii, mut jj, n, w, n1, trunc) in large_type_gen_var_47().get(gm, config).take(limit) {
        let ii_old = ii.clone();
        let jj_old = jj.clone();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut tt = vec![0; (limbs + 1) << 1];
        fft_mfa_truncate_sqrt2_inner(
            &mut ii,
            jj.as_deref_mut(),
            n,
            w,
            &mut t1,
            &mut t2,
            n1,
            trunc,
            &mut tt,
        );
        println!(
            "fft_mfa_truncate_sqrt2_inner({ii_old:?}, {jj_old:?}, {n}, {w}, _, _, _, {n1}, \
            {trunc}, _) = ({ii:?}, {jj:?})"
        );
    }
}
