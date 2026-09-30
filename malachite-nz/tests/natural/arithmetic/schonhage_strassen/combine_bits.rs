// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::combine_bits::*;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::split_bits::*;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{large_type_gen_var_51, large_type_gen_var_52};

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_combine_limbs() {
    let test =
        |res: &[Limb], poly: &[&[Limb]], coeff_limbs: usize, output_limbs: usize, out: &[Limb]| {
            let mut res = res.to_vec();
            let poly: Vec<Vec<Limb>> = poly.iter().map(|x| x.to_vec()).collect();
            let total_limbs = res.len();
            fft_combine_limbs(
                &mut res,
                &poly,
                poly.len(),
                coeff_limbs,
                output_limbs,
                total_limbs,
            );
            assert_eq!(res, out);
        };
    // - !(i < length && skip + output_limbs < total_limbs)
    // - i < length && skip + output_limbs < total_limbs
    // - !(skip < total_limbs && i < length)
    // - skip < total_limbs && i < length
    test(&[0, 0, 0], &[&[1, 0], &[2, 0], &[3, 0]], 1, 1, &[1, 2, 3]);
    test(
        &[0, 0, 0, 0],
        &[&[1, 2, 0], &[3, 4, 0]],
        1,
        2,
        &[1, 5, 4, 0],
    );
    test(
        &[18446744073709551615, 0, 0],
        &[&[1, 0], &[18446744073709551615, 0]],
        1,
        1,
        &[0, 0, 1],
    );
    test(
        &[5, 6],
        &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]],
        2,
        2,
        &[6, 8],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_combine_bits() {
    let test = |res: &[Limb], poly: &[&[Limb]], bits: u64, output_limbs: usize, out: &[Limb]| {
        let mut res = res.to_vec();
        let poly: Vec<Vec<Limb>> = poly.iter().map(|x| x.to_vec()).collect();
        let total_limbs = res.len();
        fft_combine_bits(&mut res, &poly, poly.len(), bits, output_limbs, total_limbs);
        assert_eq!(res, out);
    };
    // - top_bits != 0
    // - !(i < length && limb_ptr + output_limbs + 1 < end)
    // - !(limb_ptr < end && i < length)
    // - limb_ptr < end && i < length
    // - shift_bits == 0
    // - shift_bits != 0
    // - shift_bits < Limb::WIDTH
    test(
        &[0, 0],
        &[&[1, 0], &[2, 0], &[3, 0]],
        16,
        1,
        &[12885032961, 0],
    );
    // - i < length && limb_ptr + output_limbs + 1 < end
    // - shift_bits == 0
    // - shift_bits < Limb::WIDTH
    // - shift_bits >= Limb::WIDTH
    test(
        &[0, 0, 0],
        &[&[18446744073709551615, 1], &[18446744073709551615, 1]],
        100,
        1,
        &[18446744073709551615, 18446744004990074880, 137438953471],
    );
    test(
        &[0, 0, 0, 0],
        &[&[1, 2, 0], &[3, 4, 0], &[5, 6, 0]],
        70,
        2,
        &[1, 194, 20736, 24576],
    );
    test(&[18446744073709551615], &[&[1, 0], &[1, 0]], 1, 1, &[2]);
    test(
        &[0, 0],
        &[&[1, 0], &[2, 0], &[3, 0]],
        48,
        1,
        &[562949953421313, 12884901888],
    );
    // - top_bits == 0
    test(&[0, 0, 0], &[&[1, 0], &[2, 0]], 64, 1, &[1, 2, 0]);
    // - shift_bits != 0
    // - shift_bits >= Limb::WIDTH
    test(
        &[0, 0, 0, 0, 0, 0],
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0]],
        48,
        1,
        &[562949953421313, 12884901888, 262144, 0, 0, 0],
    );
}

#[test]
fn fft_combine_limbs_properties() {
    // Combining the coefficients that splitting makes gives back the original limbs.
    large_type_gen_var_51().test_properties(|(limbs, coeff_limbs, _)| {
        let total_limbs = limbs.len();
        let output_limbs = coeff_limbs;
        let length = (total_limbs - 1) / coeff_limbs + 1;
        let mut poly = vec![vec![0; output_limbs + 1]; length];
        fft_split_limbs(&mut poly, &limbs, total_limbs, coeff_limbs, output_limbs);
        let mut out = vec![0; total_limbs];
        fft_combine_limbs(
            &mut out,
            &poly,
            length,
            coeff_limbs,
            output_limbs,
            total_limbs,
        );
        assert_eq!(out, limbs);
    });
}

#[test]
fn fft_combine_bits_properties() {
    // Combining the coefficients that splitting makes gives back the original limbs, as in FLINT's
    // tests, which give each coefficient room for the product of two.
    large_type_gen_var_52().test_properties(|(limbs, bits, _)| {
        let total_limbs = limbs.len();
        let output_limbs = usize::try_from(((bits << 1) - 1) >> Limb::LOG_WIDTH).unwrap() + 1;
        let length = usize::try_from(
            ((u64::try_from(total_limbs).unwrap() << Limb::LOG_WIDTH) - 1) / bits + 1,
        )
        .unwrap();
        let mut poly = vec![vec![0; output_limbs + 1]; length];
        fft_split_bits(&mut poly, &limbs, total_limbs, bits, output_limbs);
        let mut out = vec![0; total_limbs];
        fft_combine_bits(&mut out, &poly, length, bits, output_limbs, total_limbs);
        assert_eq!(out, limbs);
    });
}
