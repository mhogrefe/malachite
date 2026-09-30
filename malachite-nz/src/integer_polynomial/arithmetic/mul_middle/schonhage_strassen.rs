// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008-2011 William Hart
//
//      Copyright © 2022, 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::mul_middle::classical::mul_middle_to_out_classical;
use crate::integer_polynomial::arithmetic::mul_middle::truncate_mul_middle_inputs;
use crate::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::mul::schonhage_strassen::convolution::fft_convolution;
use crate::natural::arithmetic::mul::schonhage_strassen::limbs_neg_to_out;
use crate::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1::*;
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::natural::{LIMB_HIGH_BIT, Natural};
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::swap;
use core::ptr;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

// Writes the coefficients `xs` into the residues `coeffs_f`, each of `limbs + 1` limbs, in two's
// complement.
//
// This is `_fmpz_vec_get_fft` from `fmpz_vec/get_fft.c`, FLINT 3.6.0, without the threading.
crate_test_fn! {vec_get_fft(coeffs_f: &mut [Vec<Limb>], xs: &[Integer], limbs: usize) {
    let size_f = limbs + 1;
    for (f, x) in coeffs_f.iter_mut().zip(xs.iter()) {
        let coeff = x.unsigned_abs_ref().as_limbs_asc();
        let size_j = coeff.len();
        let f = &mut f[..size_f];
        if *x < 0u32 {
            // write out FFT coefficient, ensuring sign is correct
            limbs_neg_to_out(f, coeff);
            f[size_j..].fill(Limb::MAX);
        } else {
            f[..size_j].copy_from_slice(coeff);
            f[size_j..].fill(0);
        }
    }
}}

// Reads `out.len()` coefficients from the normalized residues `coeffs_f`, each of `limbs + 1`
// limbs. If `sign` is set, residues in the upper half of the range stand for negative coefficients.
//
// This is `_fmpz_vec_set_fft` from `fmpz_vec/set_fft.c`, FLINT 3.6.0, without the threading, and
// with a fix to the test for negative coefficients.
crate_test_fn! {vec_set_fft(out: &mut [Integer], coeffs_f: &[Vec<Limb>], limbs: usize, sign: bool) {
    for (x, f) in out.iter_mut().zip(coeffs_f.iter()) {
        // FLINT tests `f[limbs - 1] > LIMB_HIGH_BIT` here, which misreads a negative coefficient
        // whose absolute value is within $2^{N-\text{W}}$ of $2^{N-1}$, where $N$ is `limbs *
        // Limb::WIDTH`, as positive: its residue is then just above $2^{N-1}$, so its top limb is
        // `LIMB_HIGH_BIT`. Such coefficients fit the bound that `res_bits` sets, and FLINT 3.6.0's
        // `_fmpz_poly_mul_SS` returns wrong results for them. Every coefficient is less than
        // $2^{N-1}$ in absolute value, so a nonnegative one has a top limb below `LIMB_HIGH_BIT`.
        *x = if sign && (f[limbs - 1] >= LIMB_HIGH_BIT || f[limbs] != 0) {
            let mut data = f[..limbs].to_vec();
            limbs_neg_in_place(&mut data);
            limbs_slice_add_limb_in_place(&mut data, 1);
            -Integer::from(Natural::from_owned_limbs_asc(data))
        } else {
            Integer::from(Natural::from_limbs_asc(&f[..limbs]))
        };
    }
}}

// Sets `out` to coefficients `nlo` (inclusive) through `nhi` (exclusive) of the product of the
// polynomials with coefficients `xs` and `ys`, both nonempty, by Schönhage–Strassen
// multiplication: a truncated convolution of residues modulo $2^{nw} + 1$, computed with transforms
// that need no multiplications. `out` must have length `nhi - nlo`, and `nhi` must be less than
// `xs.len() + ys.len()`.
//
// This is equivalent to `_fmpz_poly_mulmid_SS` from `fmpz_poly/mulmid_SS.c`, FLINT 3.6.0.
crate_test_fn! {mul_middle_to_out_schonhage_strassen(
    out: &mut [Integer],
    xs: &[Integer],
    ys: &[Integer],
    nlo: usize,
    nhi: usize,
) {
    assert_ne!(xs.len(), 0);
    assert_ne!(ys.len(), 0);
    assert!(nlo < nhi);
    assert!(nhi < xs.len() + ys.len());
    let (mut xs, mut ys, nlo, nhi) = truncate_mul_middle_inputs(xs, ys, nlo, nhi);
    // The algorithm requires trunc >= 3
    if nhi <= 2 {
        mul_middle_to_out_classical(out, xs, ys, nlo, nhi);
        return;
    }
    if xs.len() < ys.len() {
        swap(&mut xs, &mut ys);
    }
    let trunc = nhi;
    let xs = &xs[..min(xs.len(), trunc)];
    let ys = &ys[..min(ys.len(), trunc)];
    let square = ptr::eq(xs, ys);
    let len1 = xs.len();
    let len2 = ys.len();
    let len_out = len1 + len2 - 1;
    let loglen = u64::exact_from(len_out).ceiling_log_base_2();
    let loglen2 = u64::exact_from(len2).ceiling_log_base_2();
    let n = usize::power_of_2(loglen - 2);
    let (bits1, negative1) = vec_max_bits(xs);
    let (bits2, negative2) = if square {
        (bits1, negative1)
    } else {
        vec_max_bits(ys)
    };
    let size1 = bits1.div_ceil(Limb::WIDTH);
    let size2 = bits2.div_ceil(Limb::WIDTH);
    // Start with an upper bound on the number of bits needed
    let res_bits = ((size1 + size2) << Limb::LOG_WIDTH) + loglen2 + 1;
    // round up for sqrt2 trick
    let res_bits = (((res_bits - 1) >> (loglen - 2)) + 1) << (loglen - 2);
    // initial size of FFT coeffs
    let mut limbs = usize::exact_from((res_bits - 1) >> Limb::LOG_WIDTH) + 1;
    if limbs > FFT_MULMOD_2EXPP1_CUTOFF {
        // can't be worse than next power of 2 limbs
        limbs = usize::power_of_2(u64::exact_from(limbs).ceiling_log_base_2());
    }
    let size = limbs + 1;
    // allocate space for ffts: the residues, followed by the two scratch residues that the
    // transforms swap with them
    let mut residues: Vec<Vec<Limb>> = vec![vec![0; size]; (n << 2) + 2];
    let (ii, t) = residues.split_at_mut(n << 2);
    let (t1, t2) = t.split_at_mut(1);
    let (t1, t2) = (&mut t1[0], &mut t2[0]);
    let mut scratch = vec![0; size * 3];
    let (s1, tt) = scratch.split_at_mut(size);
    // put coefficients into FFT vecs
    vec_get_fft(ii, xs, limbs);
    let mut jj = if square {
        None
    } else {
        let mut jj: Vec<Vec<Limb>> = vec![vec![0; size]; n << 2];
        vec_get_fft(&mut jj, ys, limbs);
        Some(jj)
    };
    let sign = negative1 || negative2;
    // Recompute the number of bits/limbs now that we know how large everything is
    let res_bits = bits1 + bits2 + loglen2 + u64::from(sign);
    if res_bits == 0 {
        // Every coefficient is zero and `ys` has length 1. FLINT's signed arithmetic goes on to use
        // residues of one limb, even when that makes $w = 0$; the product is zero either way.
        out[..trunc - nlo].fill(Integer::ZERO);
        return;
    }
    // round up res bits for sqrt2
    let res_bits = (((res_bits - 1) >> (loglen - 2)) + 1) << (loglen - 2);
    let limbs = usize::exact_from((res_bits - 1) >> Limb::LOG_WIDTH) + 1;
    let limbs = fft_adjust_limbs(limbs); // round up limbs for Nussbaumer
    fft_convolution(ii, jj.as_deref_mut(), loglen - 2, limbs, len_out, t1, t2, s1, tt);
    vec_set_fft(&mut out[..trunc - nlo], &ii[nlo..], limbs, sign); // write res
}}
