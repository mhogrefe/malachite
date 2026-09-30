// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::split_bits::*;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{large_type_gen_var_51, large_type_gen_var_52};
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

fn split_limbs_helper(
    limbs: &[Limb],
    coeff_limbs: usize,
    output_limbs: usize,
) -> (usize, Vec<Vec<Limb>>) {
    let total_limbs = limbs.len();
    let length = (total_limbs - 1) / coeff_limbs + 1;
    let mut poly = vec![vec![0; output_limbs + 1]; length];
    let result = fft_split_limbs(&mut poly, limbs, total_limbs, coeff_limbs, output_limbs);
    (result, poly)
}

fn split_bits_helper(limbs: &[Limb], bits: u64, output_limbs: usize) -> (usize, Vec<Vec<Limb>>) {
    let total_limbs = limbs.len();
    let length =
        usize::try_from(((u64::try_from(total_limbs).unwrap() << Limb::LOG_WIDTH) - 1) / bits + 1)
            .unwrap();
    let mut poly = vec![vec![0; output_limbs + 1]; length];
    let result = fft_split_bits(&mut poly, limbs, total_limbs, bits, output_limbs);
    (result, poly)
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_split_limbs() {
    let test =
        |limbs: &[Limb], coeff_limbs: usize, output_limbs: usize, length: usize, out: &[Limb]| {
            let (result, poly) = split_limbs_helper(limbs, coeff_limbs, output_limbs);
            assert_eq!(result, length);
            assert_eq!(poly.concat(), out);
        };
    // - i >= length
    // - total_limbs <= skip
    test(&[1, 2, 3], 1, 1, 3, &[1, 0, 2, 0, 3, 0]);
    // - i < length
    // - total_limbs > skip
    test(&[1, 2, 3], 2, 2, 2, &[1, 2, 0, 3, 0, 0]);
    test(&[1, 2, 3, 4], 2, 1, 2, &[1, 2, 3, 4]);
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_split_bits() {
    let test = |limbs: &[Limb], bits: u64, output_limbs: usize, length: usize, out: &[Limb]| {
        let (result, poly) = split_bits_helper(limbs, bits, output_limbs);
        assert_eq!(result, length);
        assert_eq!(poly.concat(), out);
    };
    // - top_bits != 0
    // - shift_bits != 0
    // - shift_bits == 0
    // - shift_bits < Limb::WIDTH
    // - shift_bits != 0
    test(
        &[18446744073709551615],
        16,
        0,
        4,
        &[65535, 65535, 65535, 65535],
    );
    // - shift_bits >= Limb::WIDTH
    test(
        &[18446744073709551615, 5],
        48,
        1,
        3,
        &[281474976710655, 0, 393215, 0, 0, 0],
    );
    // - top_bits == 0
    test(&[1, 2, 3], 64, 1, 3, &[1, 0, 2, 0, 3, 0]);
    test(&[1, 2, 3], 100, 2, 2, &[1, 2, 0, 805306368, 0, 0]);
    test(
        &[18446744073709551615, 18446744073709551615, 18446744073709551615],
        70,
        1,
        3,
        &[18446744073709551615, 63, 18446744073709551615, 63, 4503599627370495, 0],
    );
    // - shift_bits == 0
    test(
        &[1, 2, 3, 4],
        96,
        2,
        3,
        &[1, 2, 0, 12884901888, 0, 0, 4, 0, 0],
    );
}

#[test]
fn fft_split_limbs_properties() {
    large_type_gen_var_51().test_properties(|(limbs, coeff_limbs, output_limbs)| {
        let total_limbs = limbs.len();
        let (length, poly) = split_limbs_helper(&limbs, coeff_limbs, output_limbs);
        assert_eq!(length, total_limbs.div_ceil(coeff_limbs));
        for (i, c) in poly.iter().enumerate() {
            let start = i * coeff_limbs;
            let end = (start + coeff_limbs).min(total_limbs);
            assert_eq!(
                Natural::from_limbs_asc(c),
                Natural::from_limbs_asc(&limbs[start..end])
            );
        }
    });
}

#[test]
fn fft_split_bits_properties() {
    large_type_gen_var_52().test_properties(|(limbs, bits, output_limbs)| {
        let (length, poly) = split_bits_helper(&limbs, bits, output_limbs);
        let total_bits = u64::try_from(limbs.len()).unwrap() << Limb::LOG_WIDTH;
        assert_eq!(u64::try_from(length).unwrap(), total_bits.div_ceil(bits));
        for (i, c) in poly.iter().enumerate() {
            let start = u64::try_from(i).unwrap() * bits;
            assert_eq!(
                Natural::from_limbs_asc(c),
                limbs_bit_field(&limbs, start, start + bits)
            );
        }
    });
}
