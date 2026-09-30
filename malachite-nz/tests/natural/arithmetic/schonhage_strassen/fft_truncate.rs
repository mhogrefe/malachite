// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_radix2::fft_radix2;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_truncate::*;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_41;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_truncate1() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, trunc: usize, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        fft_truncate1(&mut ii, n, w, &mut t1, &mut t2, trunc);
        assert_eq!(ii.concat(), out);
    };
    // - trunc == n << 1
    test(
        &[&[1, 0], &[2, 0]],
        1,
        64,
        2,
        &[3, 0, 18446744073709551615, 18446744073709551615],
    );
    // - trunc != n << 1
    // - trunc <= n
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 1]],
        2,
        32,
        2,
        &[10, 1, 18446744073709551614, 18446744073709551614, 3, 0, 4, 1],
    );
    test(
        &[&[1, 2, 0], &[3, 4, 1], &[5, 6, 0], &[7, 8, 18446744073709551615]],
        2,
        64,
        2,
        &[
            16,
            20,
            0,
            18446744073709551612,
            18446744073709551611,
            18446744073709551615,
            5,
            6,
            0,
            7,
            8,
            18446744073709551615,
        ],
    );
    // - trunc > n
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 3]],
        4,
        16,
        6,
        &[
            36,
            3,
            18446744073709551612,
            18446744073709551612,
            18446744069414584317,
            0,
            4294967291,
            18446744073709551615,
            18446462581552709631,
            2,
            281457797103611,
            18446744073709551615,
            18446744056529682433,
            0,
            18446462598732840961,
            0,
        ],
    );
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 3]],
        4,
        16,
        2,
        &[36, 3, 18446744073709551612, 18446744073709551612, 10, 0, 12, 3, 5, 0, 6, 0, 7, 0, 8, 3],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_truncate() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, trunc: usize, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        fft_truncate(&mut ii, n, w, &mut t1, &mut t2, trunc);
        assert_eq!(ii.concat(), out);
    };
    // - trunc == n << 1
    test(
        &[&[1, 0], &[2, 0]],
        1,
        64,
        2,
        &[3, 0, 18446744073709551615, 18446744073709551615],
    );
    // - trunc != n << 1
    // - trunc <= n
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 1]],
        2,
        32,
        2,
        &[3, 0, 18446744073709551615, 18446744073709551615, 3, 0, 4, 1],
    );
    test(
        &[&[1, 2, 0], &[3, 4, 1], &[5, 6, 0], &[7, 8, 18446744073709551615]],
        2,
        64,
        2,
        &[
            4,
            6,
            1,
            18446744073709551614,
            18446744073709551613,
            18446744073709551614,
            5,
            6,
            0,
            7,
            8,
            18446744073709551615,
        ],
    );
    // - trunc > n
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 3]],
        4,
        16,
        6,
        &[
            21,
            0,
            18446744073709551613,
            18446744073709551615,
            17179869187,
            0,
            18446744056529682435,
            18446744073709551615,
            1125912791482365,
            1,
            18445618186687873019,
            18446744073709551614,
            12884901888,
            0,
            1125899906842624,
            0,
        ],
    );
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 3]],
        4,
        16,
        2,
        &[3, 0, 18446744073709551615, 18446744073709551615, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 3],
    );
}

#[test]
fn fft_truncate_properties() {
    large_type_gen_var_41().test_properties(|(ii, n, w, trunc)| {
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        // fft_truncate1 uses all the inputs.
        let mut full = ii.clone();
        fft_radix2(&mut full, n, w, &mut t1, &mut t2);
        let mut out = ii.clone();
        fft_truncate1(&mut out, n, w, &mut t1, &mut t2, trunc);
        assert_eq!(
            residues_mod(&out[..trunc], limbs),
            residues_mod(&full[..trunc], limbs)
        );
        // fft_truncate treats the inputs past `trunc` as zero.
        let mut zeroed = ii.clone();
        for x in &mut zeroed[trunc..] {
            x.fill(0);
        }
        let mut full = zeroed.clone();
        fft_radix2(&mut full, n, w, &mut t1, &mut t2);
        let mut out = ii;
        fft_truncate(&mut out, n, w, &mut t1, &mut t2, trunc);
        assert_eq!(
            residues_mod(&out[..trunc], limbs),
            residues_mod(&full[..trunc], limbs)
        );
    });
}
