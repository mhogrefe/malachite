// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::convolution::fft_convolution;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{large_type_gen_var_55, large_type_gen_var_56};
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[allow(clippy::type_complexity)]
fn convolution_helper(
    mut ii: Vec<Vec<Limb>>,
    mut jj: Option<Vec<Vec<Limb>>>,
    depth: u64,
    limbs: usize,
    trunc: usize,
) -> Vec<Vec<Limb>> {
    let size = limbs + 1;
    let mut t1 = vec![0; size];
    let mut t2 = vec![0; size];
    let mut s1 = vec![0; size];
    let mut tt = vec![0; size << 1];
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
    ii
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_convolution() {
    let test = |ii: &[&[Limb]], jj: Option<&[&[Limb]]>, depth: u64, trunc: usize, out: &[Limb]| {
        let ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let jj: Option<Vec<Vec<Limb>>> = jj.map(|jj| jj.iter().map(|x| x.to_vec()).collect());
        let limbs = ii[0].len() - 1;
        let out_alt = convolution_helper(ii, jj, depth, limbs, trunc);
        assert_eq!(out_alt.concat(), out);
    };
    // The product of inputs of lengths 2 and 2 has length 3.
    let ii: &[&[Limb]] = &[&[1, 0], &[2, 0], &[0, 0], &[0, 0]];
    let jj: &[&[Limb]] = &[&[4, 0], &[5, 0], &[0, 0], &[0, 0]];
    // - depth <= 6
    // - if let Some(jj) = jj.as_deref_mut()
    // - if let Some(jj) = jj.as_deref_mut()
    test(ii, Some(jj), 0, 3, &[4, 0, 13, 0, 10, 0, 0, 0]);
    test(ii, None, 0, 3, &[1, 0, 4, 0, 4, 0, 0, 0]);
    // The product of inputs of lengths 3 and 3 has length 5.
    let ii: &[&[Limb]] = &[&[1, 0], &[2, 0], &[3, 0], &[0, 0], &[0, 0], &[0, 0], &[0, 0], &[0, 0]];
    let jj: &[&[Limb]] = &[&[6, 0], &[5, 0], &[4, 1], &[0, 0], &[0, 0], &[0, 0], &[0, 0], &[0, 0]];
    test(
        ii,
        Some(jj),
        1,
        5,
        &[6, 0, 17, 0, 31, 0, 21, 0, 9, 0, 0, 0, 532575944704, 0, 23643898043695104, 0],
    );
}

fn convolution_properties_helper(
    (ii, jj, depth, limbs, trunc): (Vec<Vec<Limb>>, Option<Vec<Vec<Limb>>>, u64, usize, usize),
) {
    let xs = residues_mod(&ii, limbs);
    let ys = jj
        .as_ref()
        .map_or_else(|| xs.clone(), |jj| residues_mod(jj, limbs));
    let out = convolution_helper(ii.clone(), jj.clone(), depth, limbs, trunc);
    let product = fermat_cyclic_convolution(&xs, &ys, limbs);
    for (x, y) in out[..trunc].iter().zip(product.iter()) {
        assert!(residue_is_normalized(x, limbs));
        assert_eq!(&residue_mod(x, limbs), y);
    }
    if jj.is_none() {
        let out_alt = convolution_helper(ii.clone(), Some(ii), depth, limbs, trunc);
        assert_eq!(out_alt, out);
    }
}

#[test]
fn fft_convolution_properties() {
    large_type_gen_var_55().test_properties_with_limit(2000, convolution_properties_helper);
}

#[test]
fn fft_convolution_matrix_fourier_properties() {
    large_type_gen_var_56().test_properties_with_limit(20, convolution_properties_helper);
}

// The matrix Fourier path, `depth > 6`. The inputs are too large to write out, so the result is
// checked against the cyclic convolution. `w` is odd when `limbs * Limb::WIDTH` is `1 << depth`.
#[test]
fn test_fft_convolution_matrix_fourier() {
    let test = |depth: u64, limbs: usize, len1: usize, len2: Option<usize>| {
        let count = 4 << depth;
        let mut ii = pseudorandom_residues(count, limbs, 4);
        for x in &mut ii[len1..] {
            x.fill(0);
        }
        let jj = len2.map(|len2| {
            let mut jj = pseudorandom_residues(count, limbs, 5);
            for x in &mut jj[len2..] {
                x.fill(0);
            }
            jj
        });
        let trunc = len1 + len2.unwrap_or(len1) - 1;
        let xs = residues_mod(&ii, limbs);
        let ys = jj
            .as_ref()
            .map_or_else(|| xs.clone(), |jj| residues_mod(jj, limbs));
        let out = convolution_helper(ii, jj, depth, limbs, trunc);
        let product = fermat_cyclic_convolution(&xs, &ys, limbs);
        for (x, y) in out[..trunc].iter().zip(product.iter()) {
            assert_eq!(&residue_mod(x, limbs), y);
        }
    };
    let limbs = (128 / Limb::WIDTH) as usize;
    // - depth > 6
    // - if let Some(jj) = jj.as_deref_mut()
    test(7, limbs, 300, Some(100));
    test(7, limbs, 150, None);
    test(7, limbs << 1, 256, Some(200));
}
