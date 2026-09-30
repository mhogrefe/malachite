// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2011, 2020 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::mul::karatsuba::revbin;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_radix2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_radix2::ifft_radix2;
use crate::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1::fft_mulmod_2expp1;
use crate::natural::arithmetic::mul::schonhage_strassen::normmod_2expp1::limbs_norm_mod_2expp1;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::CeilingLogBase2;
use malachite_base::num::conversion::traits::ExactFrom;

// Transforms, multiplies, and inverse-transforms row `i` of the matrix of residues, as the inner
// step of the matrix Fourier convolution. If `jj` is `None`, the residues of `ii` are squared.
fn fft_mfa_row(
    ii: &mut [Vec<Limb>],
    mut jj: Option<&mut [Vec<Limb>]>,
    i: usize,
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    n1: usize,
    n2: usize,
    limbs: usize,
    tt: &mut [Limb],
) {
    let wn2 = w * u64::exact_from(n2);
    fft_radix2(&mut ii[i * n1..], n1 >> 1, wn2, t1, t2);
    if let Some(jj) = jj.as_deref_mut() {
        fft_radix2(&mut jj[i * n1..], n1 >> 1, wn2, t1, t2);
    }
    for j in 0..n1 {
        let t = i * n1 + j;
        limbs_norm_mod_2expp1(&mut ii[t], limbs);
        if let Some(jj) = jj.as_deref_mut() {
            limbs_norm_mod_2expp1(&mut jj[t], limbs);
            fft_mulmod_2expp1(&mut ii[t], Some(&jj[t]), n, w, tt);
        } else {
            fft_mulmod_2expp1(&mut ii[t], None, n, w, tt);
        }
    }
    ifft_radix2(&mut ii[i * n1..], n1 >> 1, wn2, t1, t2);
}

// The row transforms, pointwise products, and inverse row transforms of the matrix Fourier
// convolution of `ii` and `jj` (or of `ii` with itself, if `jj` is `None`), after
// `fft_mfa_truncate_sqrt2_outer` has been applied to both. The products are left in `ii`.
//
// This is fft_mfa_truncate_sqrt2_inner from fft/fft_mfa_truncate_sqrt2_inner.c, FLINT 3.6.0,
// without the threading.
crate_test_fn! {fft_mfa_truncate_sqrt2_inner(
    ii: &mut [Vec<Limb>],
    mut jj: Option<&mut [Vec<Limb>]>,
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    n1: usize,
    trunc: usize,
    tt: &mut [Limb],
) {
    let two_n = n << 1;
    let n2 = two_n / n1;
    let trunc2 = (trunc - two_n) / n1;
    let limbs = fft_limbs(n, w);
    let depth = n2.ceiling_log_base_2();
    // convolutions on relevant rows
    for s in 0..trunc2 {
        let i = revbin(s, depth);
        fft_mfa_row(
            &mut ii[two_n..],
            jj.as_deref_mut().map(|jj| &mut jj[two_n..]),
            i,
            n,
            w,
            t1,
            t2,
            n1,
            n2,
            limbs,
            tt,
        );
    }
    // convolutions on rows
    for i in 0..n2 {
        fft_mfa_row(ii, jj.as_deref_mut(), i, n, w, t1, t2, n1, n2, limbs, tt);
    }
}}
