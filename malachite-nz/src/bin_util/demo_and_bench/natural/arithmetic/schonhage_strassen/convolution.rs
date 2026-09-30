// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::convolution::fft_convolution;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1_basecase::*;
use malachite_nz::test_util::generators::{large_type_gen_var_55, large_type_gen_var_56};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_fft_convolution);
    register_demo!(runner, demo_fft_convolution_matrix_fourier);
}

fn demo_fft_convolution(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ii, mut jj, depth, limbs, trunc) in large_type_gen_var_55().get(gm, config).take(limit)
    {
        let ii_old = ii.clone();
        let jj_old = jj.clone();
        let size = limbs + 1;
        let mut t1 = vec![0; size];
        let mut t2 = vec![0; size];
        let mut s1 = vec![0; size];
        let mut tt = vec![0; limbs_mul_mod_2expp1_basecase_scratch_len(limbs)];
        fft_convolution(
            &mut ii,
            jj.as_deref_mut(),
            depth,
            limbs,
            trunc,
            &mut t1,
            &mut t2,
            &mut s1,
            &mut tt,
        );
        println!(
            "fft_convolution({ii_old:?}, {jj_old:?}, {depth}, {limbs}, {trunc}, _, _, _, _) = \
            ({ii:?}, {jj:?})"
        );
    }
}

fn demo_fft_convolution_matrix_fourier(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ii, mut jj, depth, limbs, trunc) in large_type_gen_var_56().get(gm, config).take(limit)
    {
        let ii_old = ii.clone();
        let jj_old = jj.clone();
        let size = limbs + 1;
        let mut t1 = vec![0; size];
        let mut t2 = vec![0; size];
        let mut s1 = vec![0; size];
        let mut tt = vec![0; limbs_mul_mod_2expp1_basecase_scratch_len(limbs)];
        fft_convolution(
            &mut ii,
            jj.as_deref_mut(),
            depth,
            limbs,
            trunc,
            &mut t1,
            &mut t2,
            &mut s1,
            &mut tt,
        );
        println!(
            "fft_convolution({ii_old:?}, {jj_old:?}, {depth}, {limbs}, {trunc}, _, _, _, _) = \
            ({ii:?}, {jj:?})"
        );
    }
}
