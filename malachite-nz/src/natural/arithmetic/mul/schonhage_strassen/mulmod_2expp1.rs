// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2011 William Hart
//
//      Copyright © 2024 Albin Ahlbäck
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::mul::schonhage_strassen::combine_bits::fft_combine_bits;
use crate::natural::arithmetic::mul::schonhage_strassen::div_2expmod_2expp1::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_negacyclic::fft_negacyclic;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_negacyclic::ifft_negacyclic;
use crate::natural::arithmetic::mul::schonhage_strassen::limbs_add_signed_limb_mod_2expp1;
use crate::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1_basecase::*;
use crate::natural::arithmetic::mul::schonhage_strassen::normmod_2expp1::limbs_norm_mod_2expp1;
use crate::natural::arithmetic::mul::schonhage_strassen::split_bits::fft_split_bits;
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::natural::arithmetic::sub::{
    limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
};
use crate::platform::{Limb, SignedLimb};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, Parity, PowerOf2, WrappingAddAssign, WrappingAddMulAssign,
    WrappingSubMulAssign, XXAddYYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};

// The amount subtracted from half the depth of the outer transform to get the depth of the inner
// transform, indexed by the outer depth minus 12.
//
// This is MULMOD_TAB from flint-mparam.h, FLINT 3.6.0.
const MULMOD_2EXPP1_TABLE_N: [u64; FFT_N_NUM] =
    [4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 1, 1];

// This is FFT_N_NUM from flint-mparam.h, FLINT 3.6.0.
const FFT_N_NUM: usize = 19;

// The largest depth that has its own entry in `MULMOD_2EXPP1_TABLE_N`.
const MAX_TABLE_DEPTH: u64 = FFT_N_NUM as u64 + 11;

// The number of limbs at or below which `fft_mulmod_2expp1` uses the basecase.
//
// This is FFT_MULMOD_2EXPP1_CUTOFF from flint-mparam.h, FLINT 3.6.0.
pub(crate) const FFT_MULMOD_2EXPP1_CUTOFF: usize = 128;

// Sets `r[..m]` to the negacyclic convolution of `ii[..m]` and `jj[..m]` modulo $2^\text{W}$, where
// W is `Limb::WIDTH`.
//
// This is fft_naive_convolution_1 from fft/mulmod_2expp1.c, FLINT 3.6.0.
crate_test_fn! {fft_naive_convolution_1(r: &mut [Limb], ii: &[Limb], jj: &[Limb], m: usize) {
    for i in 0..m {
        r[i] = ii[0].wrapping_mul(jj[i]);
    }
    for i in 1..m {
        for j in 0..m - i {
            r[i + j].wrapping_add_mul_assign(ii[i], jj[j]);
        }
        for j in m - i..m {
            r[i + j - m].wrapping_sub_mul_assign(ii[i], jj[j]);
        }
    }
}}

fn mulmod_2expp1_table_n(depth: u64) -> u64 {
    if depth < 12 {
        // More than `FFT_MULMOD_2EXPP1_CUTOFF` limbs make the depth at least 13.
        fail_on_untested_path("mulmod_2expp1_table_n, depth < 12");
        MULMOD_2EXPP1_TABLE_N[0]
    } else {
        MULMOD_2EXPP1_TABLE_N[usize::exact_from(depth.min(MAX_TABLE_DEPTH) - 12)]
    }
}

// Sets `r1[..=r_limbs]` to `r1[..=r_limbs]` times `i2[..=r_limbs]` (or `r1[..=r_limbs]` squared, if
// `i2` is `None`) modulo $2^{\text{r\_limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`, using a
// negacyclic transform of length `2 << depth` whose residues are modulo $2^{(1 << \text{depth})w} +
// 1$.
//
// This is _fft_mulmod_2expp1 from fft/mulmod_2expp1.c, FLINT 3.6.0, where `r1` and `i1` are the
// same.
crate_test_fn! {fft_mulmod_2expp1_negacyclic(
    r1: &mut [Limb],
    i2: Option<&[Limb]>,
    r_limbs: usize,
    depth: u64,
    w: u64,
) {
    let n = usize::power_of_2(depth);
    let n2 = n << 1;
    let bits1 = (u64::exact_from(r_limbs) << Limb::LOG_WIDTH) / u64::exact_from(n2);
    let limbs = usize::exact_from((u64::exact_from(n) * w) >> Limb::LOG_WIDTH);
    let size = limbs + 1;
    // The residues, followed by the two scratch residues that the transforms swap with them.
    let mut residues: Vec<Vec<Limb>> = vec![vec![0; size]; n2 + 2];
    let (ii, t) = residues.split_at_mut(n2);
    let (t1, t2) = t.split_at_mut(1);
    let (t1, t2) = (&mut t1[0], &mut t2[0]);
    let mut scratch = vec![0; (n2 << 1) + size * 3];
    let (ii0, scratch) = scratch.split_at_mut(n2);
    let (r, scratch) = scratch.split_at_mut(n2);
    let (s1, tt) = scratch.split_at_mut(size);
    let j = fft_split_bits(ii, &r1[..r_limbs], r_limbs, bits1, limbs);
    for x in &mut ii[j..] {
        x[..=limbs].fill(0);
    }
    for (x0, x) in ii0.iter_mut().zip(ii.iter()) {
        *x0 = x[0];
    }
    fft_negacyclic(ii, n, w, t1, t2, s1);
    for x in &mut *ii {
        limbs_norm_mod_2expp1(x, limbs);
    }
    let mut jj_and_jj0 = None;
    if let Some(i2) = i2 {
        let mut jj: Vec<Vec<Limb>> = vec![vec![0; size]; n2];
        let mut jj0 = vec![0; n2];
        let j = fft_split_bits(&mut jj, &i2[..r_limbs], r_limbs, bits1, limbs);
        for x in &mut jj[j..] {
            x[..=limbs].fill(0);
        }
        for (x0, x) in jj0.iter_mut().zip(jj.iter()) {
            *x0 = x[0];
        }
        fft_negacyclic(&mut jj, n, w, t1, t2, s1);
        jj_and_jj0 = Some((jj, jj0));
    }
    let nw = u64::exact_from(n) * w;
    if let Some((jj, _)) = &mut jj_and_jj0 {
        for (x, y) in ii.iter_mut().zip(jj.iter_mut()) {
            limbs_norm_mod_2expp1(y, limbs);
            let c = (x[limbs] << 1) + y[limbs];
            x[limbs] = limbs_mul_mod_2expp1_basecase(x, Some(y), c, nw, tt);
        }
    } else {
        for x in &mut *ii {
            let c = (x[limbs] << 1) + x[limbs];
            x[limbs] = limbs_mul_mod_2expp1_basecase(x, None, c, nw, tt);
        }
    }
    ifft_negacyclic(ii, n, w, t1, t2, s1);
    let jj0 = jj_and_jj0.as_ref().map_or(&*ii0, |(_, jj0)| jj0);
    fft_naive_convolution_1(r, ii0, jj0, n2);
    for (x, r_j) in ii.iter_mut().zip(r.iter_mut()) {
        limbs_div_2exp_mod_2expp1_in_place(x, limbs, depth + 1);
        limbs_norm_mod_2expp1(x, limbs);
        let t = x[limbs];
        x[limbs] = r_j.wrapping_sub(x[0]);
        let x_limbs = x[limbs];
        let cy2 = limbs_slice_add_limb_in_place(&mut x[..=limbs], x_limbs);
        (*r_j, x[limbs]) = Limb::xx_add_yy_to_zz(0, x[limbs], 0, t);
        if cy2 {
            r_j.wrapping_add_assign(1);
        }
    }
    r1[..=r_limbs].fill(0);
    fft_combine_bits(r1, ii, n2 - 1, bits1, limbs + 1, r_limbs + 1);
    // as the negacyclic convolution has effectively done subtractions some of the coefficients will
    // be negative, so need to subtract p
    let mut ll = 0;
    let limb_add = usize::exact_from(bits1 >> Limb::LOG_WIDTH);
    for j in 0..n2 - 2 {
        if r[j] != 0 {
            limbs_sub_limb_in_place(&mut r1[ll + 1..=r_limbs], 1);
        } else if SignedLimb::wrapping_from(ii[j][limbs]) < 0 {
            // coefficient was -ve
            limbs_sub_limb_in_place(&mut r1[ll + 1..=r_limbs], 1);
            limbs_sub_limb_in_place(&mut r1[ll + limbs + 1..=r_limbs], 1);
        }
        ll += limb_add;
    }
    // penultimate coefficient, top bit was already ignored
    let j = n2 - 2;
    if r[j] != 0 || SignedLimb::wrapping_from(ii[j][limbs]) < 0 {
        // coefficient was -ve
        limbs_sub_limb_in_place(&mut r1[ll + 1..=r_limbs], 1);
    }
    // final coefficient wraps around
    let last = &ii[n2 - 1];
    if limb_add == 0 {
        // With the depth and `w` that `fft_mulmod_2expp1` chooses, `2 << depth` is less than
        // `r_limbs`.
        fail_on_untested_path("fft_mulmod_2expp1_negacyclic, limb_add == 0");
    } else {
        let (r1_lo, r1_hi) = r1.split_at_mut(r_limbs);
        let carry = limbs_slice_add_same_length_in_place_left(
            &mut r1_lo[r_limbs - limb_add..],
            &last[..limb_add],
        );
        r1_hi[0].wrapping_add_assign(Limb::from(carry));
    }
    let c = limbs_sub_same_length_in_place_left(&mut r1[..size - limb_add], &last[limb_add..size]);
    limbs_add_signed_limb_mod_2expp1(
        &mut r1[size - limb_add..],
        r_limbs - limbs - 1 + limb_add,
        Limb::from(c).wrapping_neg(),
    );
    limbs_norm_mod_2expp1(r1, r_limbs);
}}

// Sets `r[..=limbs]` to `r[..=limbs]` times `i2[..=limbs]` (or `r[..=limbs]` squared, if `i2` is
// `None`) modulo $2^{nw} + 1$, where `limbs` is `n * w / Limb::WIDTH`. The inputs must be
// normalized, and `tt` needs `2 * limbs` limbs.
//
// This is fft_mulmod_2expp1 from fft/mulmod_2expp1.c, FLINT 3.6.0, where `r` and `i1` are the same.
crate_test_fn! {fft_mulmod_2expp1(
    r: &mut [Limb],
    i2: Option<&[Limb]>,
    n: usize,
    w: u64,
    tt: &mut [Limb],
) {
    let bits = u64::exact_from(n) * w;
    let limbs = usize::exact_from(bits >> Limb::LOG_WIDTH);
    let i2_top = i2.map_or(r[limbs], |i2| i2[limbs]);
    let c = (r[limbs] << 1) + i2_top;
    if c.odd() {
        limbs_neg_in_place(&mut r[..=limbs]);
        limbs_norm_mod_2expp1(r, limbs);
        return;
    } else if c & 2 != 0 {
        if let Some(i2) = i2 {
            r[..=limbs].copy_from_slice(&i2[..=limbs]);
        }
        limbs_neg_in_place(&mut r[..=limbs]);
        limbs_norm_mod_2expp1(r, limbs);
        return;
    }
    if limbs <= FFT_MULMOD_2EXPP1_CUTOFF {
        r[limbs] = limbs_mul_mod_2expp1_basecase(r, i2, c, bits, tt);
        return;
    }
    let mut depth = 1;
    while u64::power_of_2(depth) < bits {
        depth += 1;
    }
    let off = mulmod_2expp1_table_n(depth);
    let depth1 = (depth >> 1) - off;
    let w1 = bits >> (depth1 << 1);
    fft_mulmod_2expp1_negacyclic(r, i2, limbs, depth1, w1);
}}

// Rounds `limbs` up to a size for which `fft_mulmod_2expp1` can use a transform, unless `limbs` is
// small enough for the basecase.
//
// This is fft_adjust_limbs from fft/mulmod_2expp1.c, FLINT 3.6.0.
crate_test_fn! {fft_adjust_limbs(limbs: usize) -> usize {
    if limbs <= FFT_MULMOD_2EXPP1_CUTOFF {
        return limbs;
    }
    let bits1 = u64::exact_from(limbs) << Limb::LOG_WIDTH;
    let depth = u64::exact_from(limbs).ceiling_log_base_2();
    let limbs2 = u64::power_of_2(depth); // within a factor of 2 of limbs
    let bits2 = limbs2 << Limb::LOG_WIDTH;
    let depth1 = bits1.ceiling_log_base_2();
    let depth1 = (depth1 >> 1) - mulmod_2expp1_table_n(depth1);
    let depth2 = bits2.ceiling_log_base_2();
    let depth2 = (depth2 >> 1) - mulmod_2expp1_table_n(depth2);
    let depth1 = depth1.max(depth2);
    let adj = u64::power_of_2(depth1 + 1);
    let limbs2 = adj * u64::exact_from(limbs).div_ceil(adj); // round up number of limbs
    let bits1 = limbs2 << Limb::LOG_WIDTH;
    let bits2 = u64::power_of_2(depth1 << 1);
    let bits1 = bits2 * bits1.div_ceil(bits2); // round up bits
    usize::exact_from(bits1 >> Limb::LOG_WIDTH)
}}
