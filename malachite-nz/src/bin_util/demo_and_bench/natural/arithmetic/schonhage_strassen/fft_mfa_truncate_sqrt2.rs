// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_mfa_truncate_sqrt2::*;
use malachite_nz::test_util::generators::{
    large_type_gen_var_39, large_type_gen_var_44, large_type_gen_var_45, large_type_gen_var_46,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_fft_butterfly_twiddle);
    register_demo!(runner, demo_fft_radix2_twiddle);
    register_demo!(runner, demo_fft_truncate1_twiddle);
    register_demo!(runner, demo_fft_mfa_truncate_sqrt2_outer);
}

fn demo_limbs_fft_butterfly_twiddle(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, t, limbs, b1, b2) in large_type_gen_var_39().get(gm, config).take(limit) {
        let mut u = vec![0; limbs + 1];
        let mut v = vec![0; limbs + 1];
        limbs_fft_butterfly_twiddle(&mut u, &mut v, &s, &t, limbs, b1, b2);
        println!(
            "limbs_fft_butterfly_twiddle(_, _, {s:?}, {t:?}, {limbs}, {b1}, {b2}) = ({u:?}, {v:?})"
        );
    }
}

fn demo_fft_radix2_twiddle(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ii, n1, n2, w, c) in large_type_gen_var_44().get(gm, config).take(limit) {
        let ii_old = ii.clone();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let wn1 = w * u64::exact_from(n1);
        fft_radix2_twiddle(&mut ii[c..], n1, n2 >> 1, wn1, &mut t1, &mut t2, w, 0, c, 1);
        println!(
            "fft_radix2_twiddle({ii_old:?}, {c}, {n1}, {}, {wn1}, _, _, {w}, 0, {c}, 1) = {ii:?}",
            n2 >> 1
        );
    }
}

fn demo_fft_truncate1_twiddle(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ii, n1, n2, w, c, trunc) in large_type_gen_var_45().get(gm, config).take(limit) {
        let ii_old = ii.clone();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let wn1 = w * u64::exact_from(n1);
        fft_truncate1_twiddle(
            &mut ii[c..],
            n1,
            n2 >> 1,
            wn1,
            &mut t1,
            &mut t2,
            w,
            0,
            c,
            1,
            trunc,
        );
        println!(
            "fft_truncate1_twiddle({ii_old:?}, {c}, {n1}, {}, {wn1}, _, _, {w}, 0, {c}, 1, \
            {trunc}) = {ii:?}",
            n2 >> 1
        );
    }
}

fn demo_fft_mfa_truncate_sqrt2_outer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ii, n, w, n1, trunc) in large_type_gen_var_46().get(gm, config).take(limit) {
        let ii_old = ii.clone();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_mfa_truncate_sqrt2_outer(&mut ii, n, w, &mut t1, &mut t2, &mut temp, n1, trunc);
        println!(
            "fft_mfa_truncate_sqrt2_outer({ii_old:?}, {n}, {w}, _, _, _, {n1}, {trunc}) = {ii:?}"
        );
    }
}
