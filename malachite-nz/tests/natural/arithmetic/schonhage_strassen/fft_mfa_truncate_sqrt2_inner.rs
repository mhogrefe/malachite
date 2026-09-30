// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_mfa_truncate_sqrt2::*;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_mfa_truncate_sqrt2_inner::*;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::ifft_mfa_truncate_sqrt2::*;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1_basecase::*;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_47;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_mfa_truncate_sqrt2_inner() {
    let test = |ii: &[&[Limb]],
                jj: Option<&[&[Limb]]>,
                n: usize,
                w: u64,
                n1: usize,
                trunc: usize,
                out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let mut jj: Option<Vec<Vec<Limb>>> = jj.map(|jj| jj.iter().map(|x| x.to_vec()).collect());
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut tt = vec![0; limbs_mul_mod_2expp1_basecase_scratch_len(limbs)];
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
        assert_eq!(ii.concat(), out);
    };
    let ii: &[&[Limb]] = &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 0]];
    let jj: &[&[Limb]] = &[&[8, 0], &[7, 0], &[6, 0], &[5, 1], &[4, 0], &[3, 0], &[2, 0], &[1, 0]];
    // - if let Some(jj) = jj.as_deref_mut()
    // - if let Some(jj) = jj.as_deref_mut()
    // - (1usize << depth) >= n2
    // - (1usize << depth) < n2
    test(
        ii,
        Some(jj),
        2,
        32,
        2,
        8,
        &[
            45,
            1,
            45,
            18446744073709551615,
            69,
            1,
            71,
            18446744073709551615,
            77,
            1,
            77,
            18446744073709551615,
            45,
            1,
            45,
            18446744073709551615,
        ],
    );
    test(
        ii,
        None,
        2,
        32,
        2,
        8,
        &[10, 0, 8, 0, 50, 0, 48, 0, 122, 0, 120, 0, 226, 0, 224, 0],
    );
    test(
        ii,
        Some(jj),
        2,
        32,
        2,
        8,
        &[
            45,
            1,
            45,
            18446744073709551615,
            69,
            1,
            71,
            18446744073709551615,
            77,
            1,
            77,
            18446744073709551615,
            45,
            1,
            45,
            18446744073709551615,
        ],
    );
}

#[test]
fn fft_mfa_truncate_sqrt2_inner_properties() {
    large_type_gen_var_47().test_properties_with_limit(1000, |(ii, jj, n, w, n1, trunc)| {
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        let mut tt = vec![0; limbs_mul_mod_2expp1_basecase_scratch_len(limbs)];
        // Squaring is the same as multiplying by a copy.
        if jj.is_none() {
            let mut out = ii.clone();
            fft_mfa_truncate_sqrt2_inner(
                &mut out, None, n, w, &mut t1, &mut t2, n1, trunc, &mut tt,
            );
            let mut out_alt = ii.clone();
            let mut copy = ii.clone();
            fft_mfa_truncate_sqrt2_inner(
                &mut out_alt,
                Some(&mut copy),
                n,
                w,
                &mut t1,
                &mut t2,
                n1,
                trunc,
                &mut tt,
            );
            assert_eq!(out, out_alt);
        }
        // Between the outer transforms, the inner step convolves: the first `trunc` outputs are the
        // cyclic convolution of the inputs, provided that the product is zero past `trunc`.
        let len1 = (trunc + 1) >> 1;
        let len2 = trunc + 1 - len1;
        let zero_past = |mut xs: Vec<Vec<Limb>>, len: usize| {
            for x in &mut xs[len..] {
                x.fill(0);
            }
            xs
        };
        let mut ii = zero_past(ii, len1);
        let mut jj = jj.map(|jj| zero_past(jj, len2));
        let xs = residues_mod(&ii, limbs);
        let ys = jj
            .as_ref()
            .map_or_else(|| xs.clone(), |jj| residues_mod(jj, limbs));
        fft_mfa_truncate_sqrt2_outer(&mut ii, n, w, &mut t1, &mut t2, &mut temp, n1, trunc);
        if let Some(jj) = jj.as_mut() {
            fft_mfa_truncate_sqrt2_outer(jj, n, w, &mut t1, &mut t2, &mut temp, n1, trunc);
        }
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
        ifft_mfa_truncate_sqrt2_outer(&mut ii, n, w, &mut t1, &mut t2, &mut temp, n1, trunc);
        let product = fermat_cyclic_convolution(&xs, &ys, limbs);
        for (x, y) in ii[..trunc].iter().zip(product.iter()) {
            assert_eq!(&residue_mod(x, limbs), y);
        }
    });
}
