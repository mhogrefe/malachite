// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::{
    limbs_add_signed_limb_mod_2expp1, limbs_sum_diff,
};
use malachite_nz::test_util::generators::{large_type_gen_var_32, large_type_gen_var_33};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_add_signed_limb_mod_2expp1);
    register_demo!(runner, demo_limbs_sum_diff);
    adjust::register(runner);
    adjust_sqrt2::register(runner);
    butterfly_lsh_b::register(runner);
    butterfly_rsh_b::register(runner);
    combine_bits::register(runner);
    convolution::register(runner);
    div_2expmod_2expp1::register(runner);
    fft_mfa_truncate_sqrt2::register(runner);
    fft_mfa_truncate_sqrt2_inner::register(runner);
    fft_negacyclic::register(runner);
    fft_radix2::register(runner);
    fft_truncate::register(runner);
    fft_truncate_sqrt2::register(runner);
    ifft_mfa_truncate_sqrt2::register(runner);
    ifft_negacyclic::register(runner);
    ifft_radix2::register(runner);
    ifft_truncate::register(runner);
    ifft_truncate_sqrt2::register(runner);
    mul_2expmod_2expp1::register(runner);
    mulmod_2expp1::register(runner);
    mulmod_2expp1_basecase::register(runner);
    negmod_2expp1::register(runner);
    normmod_2expp1::register(runner);
    split_bits::register(runner);
}

fn demo_limbs_add_signed_limb_mod_2expp1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut r, limbs, c) in large_type_gen_var_32().get(gm, config).take(limit) {
        let r_old = r.clone();
        limbs_add_signed_limb_mod_2expp1(&mut r, limbs, c);
        println!("limbs_add_signed_limb_mod_2expp1({r_old:?}, {limbs}, {c}) = {r:?}");
    }
}

fn demo_limbs_sum_diff(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in large_type_gen_var_33().get(gm, config).take(limit) {
        let n = x.len();
        let mut s = vec![0; n];
        let mut d = vec![0; n];
        let carry = limbs_sum_diff(&mut s, &mut d, &x, &y, n);
        println!("limbs_sum_diff(_, _, {x:?}, {y:?}, {n}) = ({carry}, {s:?}, {d:?})");
    }
}

mod adjust;
mod adjust_sqrt2;
mod butterfly_lsh_b;
mod butterfly_rsh_b;
mod combine_bits;
mod convolution;
mod div_2expmod_2expp1;
mod fft_mfa_truncate_sqrt2;
mod fft_mfa_truncate_sqrt2_inner;
mod fft_negacyclic;
mod fft_radix2;
mod fft_truncate;
mod fft_truncate_sqrt2;
mod ifft_mfa_truncate_sqrt2;
mod ifft_negacyclic;
mod ifft_radix2;
mod ifft_truncate;
mod ifft_truncate_sqrt2;
mod mul_2expmod_2expp1;
mod mulmod_2expp1;
mod mulmod_2expp1_basecase;
mod negmod_2expp1;
mod normmod_2expp1;
mod split_bits;
