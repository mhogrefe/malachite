// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CeilingLogBase2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1::*;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1_basecase::*;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    large_type_gen_var_33, large_type_gen_var_49, large_type_gen_var_50, large_type_gen_var_59,
};
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_naive_convolution_1() {
    let test = |ii: &[Limb], jj: &[Limb], out: &[Limb]| {
        let mut r = vec![0; ii.len()];
        fft_naive_convolution_1(&mut r, ii, jj, ii.len());
        assert_eq!(r, out);
    };
    test(&[3], &[5], &[15]);
    test(&[1, 2], &[3, 4], &[18446744073709551611, 10]);
    test(
        &[1, 2, 3],
        &[4, 5, 6],
        &[18446744073709551593, 18446744073709551611, 28],
    );
    test(
        &[18446744073709551615, 2],
        &[18446744073709551615, 3],
        &[18446744073709551611, 18446744073709551611],
    );
}

#[test]
fn fft_naive_convolution_1_properties() {
    large_type_gen_var_33().test_properties(|(ii, jj)| {
        let mut r = vec![0; ii.len()];
        fft_naive_convolution_1(&mut r, &ii, &jj, ii.len());
        assert_eq!(r, limbs_negacyclic_convolution_naive(&ii, &jj));
    });
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_mulmod_2expp1() {
    let test = |r: &[Limb], i2: Option<&[Limb]>, n: usize, w: u64, out: &[Limb]| {
        let mut r = r.to_vec();
        let mut tt = vec![0; limbs_mul_mod_2expp1_basecase_scratch_len(r.len() - 1)];
        fft_mulmod_2expp1(&mut r, i2, n, w, &mut tt);
        assert_eq!(r, out);
    };
    // - c & 1 == 0
    // - c & 2 == 0
    // - limbs <= FFT_MULMOD_2EXPP1_CUTOFF
    test(&[3, 0], Some(&[5, 0]), 64, 1, &[15, 0]);
    // - c & 2 != 0
    // - if let Some(i2) = i2
    test(&[0, 1], Some(&[5, 0]), 64, 1, &[18446744073709551612, 0]);
    // - c & 1 != 0
    test(&[5, 0], Some(&[0, 1]), 32, 2, &[18446744073709551612, 0]);
    test(&[0, 1], Some(&[0, 1]), 64, 1, &[1, 0]);
    test(
        &[18446744073709551615, 18446744073709551615, 0],
        Some(&[18446744073709551615, 7, 0]),
        64,
        2,
        &[3, 18446744073709551600, 0],
    );
    test(
        &[18446744073709551615, 18446744073709551615, 0],
        None,
        64,
        2,
        &[4, 0, 0],
    );
    test(&[0, 0, 1], None, 64, 2, &[1, 0, 0]);
}

#[test]
fn fft_mulmod_2expp1_properties() {
    large_type_gen_var_49().test_properties(|(mut r, i2, n, w)| {
        let limbs = r.len() - 1;
        let expected = fermat_mul_naive(&r, i2.as_ref().unwrap_or(&r), limbs);
        let mut tt = vec![0; limbs_mul_mod_2expp1_basecase_scratch_len(r.len() - 1)];
        fft_mulmod_2expp1(&mut r, i2.as_deref(), n, w, &mut tt);
        assert_eq!(r, expected);
    });
}

#[test]
fn fft_mulmod_2expp1_negacyclic_properties() {
    large_type_gen_var_50().test_properties(|(mut r1, i2, r_limbs, depth, w)| {
        let expected = fermat_mul_naive(&r1, i2.as_ref().unwrap_or(&r1), r_limbs);
        fft_mulmod_2expp1_negacyclic(&mut r1, i2.as_deref(), r_limbs, depth, w);
        assert_eq!(r1, expected);
    });
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_adjust_limbs() {
    let test = |limbs: usize, out: usize| {
        assert_eq!(fft_adjust_limbs(limbs), out);
    };
    // - limbs <= FFT_MULMOD_2EXPP1_CUTOFF
    test(0, 0);
    test(1, 1);
    test(128, 128);
    // - depth >= 12
    // - limbs > FFT_MULMOD_2EXPP1_CUTOFF
    test(129, 144);
    test(1000, 1024);
    test(1 << 20, 1048576);
    test((1 << 20) + 1, 1114112);
}

#[test]
fn fft_adjust_limbs_properties() {
    large_type_gen_var_59().test_properties(|limbs| {
        let adjusted = fft_adjust_limbs(limbs);
        assert!(adjusted >= limbs);
        // 128 is FFT_MULMOD_2EXPP1_CUTOFF.
        if limbs <= 128 {
            assert_eq!(adjusted, limbs);
        } else {
            assert!(adjusted <= limbs.next_power_of_two());
            assert_eq!(fft_adjust_limbs(adjusted), adjusted);
        }
        assert!(adjusted < usize::power_of_2(62));
    });
}

// Above `FFT_MULMOD_2EXPP1_CUTOFF` limbs, `fft_mulmod_2expp1` multiplies with a negacyclic
// convolution. The inputs are too large to write out, so the results are checked against the
// product modulo $2^N + 1$.
#[test]
fn test_fft_mulmod_2expp1_large() {
    let test = |limbs: usize, square: bool| {
        let mut rs = pseudorandom_residues(2, limbs, limbs as u64);
        for r in &mut rs {
            r[limbs] = 0;
        }
        let r = rs[0].clone();
        let i2 = if square { None } else { Some(rs[1].clone()) };
        let expected = fermat_mul_naive(&r, i2.as_ref().unwrap_or(&r), limbs);
        let (n, w) = (limbs, Limb::WIDTH);
        let mut out = r.clone();
        let mut tt = vec![0; limbs_mul_mod_2expp1_basecase_scratch_len(limbs)];
        fft_mulmod_2expp1(&mut out, i2.as_deref(), n, w, &mut tt);
        assert_eq!(out, expected);
        let (depth, w) = mulmod_depth_and_w(limbs);
        let mut out = r;
        fft_mulmod_2expp1_negacyclic(&mut out, i2.as_deref(), limbs, depth, w);
        assert_eq!(out, expected);
    };
    // - if let Some(i2) = i2
    // - if let Some((jj, _)) = &mut jj_and_jj0
    // - !cy2
    // - r[j] == 0
    // - SignedLimb::wrapping_from(ii[j][limbs]) >= 0
    // - SignedLimb::wrapping_from(ii[j][limbs]) < 0
    // - !(r[j] != 0 || SignedLimb::wrapping_from(ii[j][limbs]) < 0)
    // - limb_add != 0
    // - c & 1 == 0
    // - c & 2 == 0
    // - limbs > FFT_MULMOD_2EXPP1_CUTOFF
    // - (1u64 << depth) >= bits
    // - (1u64 << depth) < bits
    test(fft_adjust_limbs(129), false);
    test(fft_adjust_limbs(129), true);
    test(fft_adjust_limbs(1000), false);
    test(fft_adjust_limbs(1000), true);
}

// The `depth` and `w` that `fft_mulmod_2expp1` passes to `fft_mulmod_2expp1_negacyclic`.
fn mulmod_depth_and_w(limbs: usize) -> (u64, u64) {
    let bits = u64::try_from(limbs).unwrap() << Limb::LOG_WIDTH;
    let depth = bits.ceiling_log_base_2().max(1);
    let off = [4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 1, 1]
        [usize::try_from(depth.clamp(12, 30) - 12).unwrap()];
    let depth1 = (depth >> 1) - off;
    (depth1, bits >> (depth1 << 1))
}

// The negacyclic convolution splits each input into `2 << depth` coefficients of `bits1` bits. The
// inputs here are powers of 2 that put a single 1 or 2 into one coefficient, so that one
// coefficient of the convolution is -1 or -2, which the correction steps treat specially.
#[test]
fn test_fft_mulmod_2expp1_negacyclic() {
    let test = |limbs: usize, e1: u64, e2: Option<u64>| {
        let r1 = normalized_residue(&Natural::power_of_2(e1), limbs);
        let i2 = e2.map(|e2| normalized_residue(&Natural::power_of_2(e2), limbs));
        let expected = fermat_mul_naive(&r1, i2.as_ref().unwrap_or(&r1), limbs);
        let (depth, w) = mulmod_depth_and_w(limbs);
        let mut out = r1;
        fft_mulmod_2expp1_negacyclic(&mut out, i2.as_deref(), limbs, depth, w);
        assert_eq!(out, expected);
    };
    let limbs = fft_adjust_limbs(129);
    let depth = mulmod_depth_and_w(limbs).0;
    let bits1 = (u64::try_from(limbs).unwrap() << Limb::LOG_WIDTH) >> (depth + 1);
    let last = bits1 * ((2 << depth) - 1);
    // - r[j] != 0: coefficient 0 is -1
    test(limbs, bits1, Some(last));
    // - coefficient 2 * n - 2 is -1
    test(limbs, last, None);
    // - cy2: coefficient 0 is -2
    test(limbs, bits1 + 1, Some(last));
}
