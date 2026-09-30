// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModMul;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_radix2::fft_radix2;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_truncate::fft_truncate;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::ifft_truncate::*;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_41;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_ifft_truncate1() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, trunc: usize, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        ifft_truncate1(&mut ii, n, w, &mut t1, &mut t2, trunc);
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
        &[3, 0, 18446744073709551610, 18446744073709551614, 3, 0, 4, 1],
    );
    test(
        &[&[1, 2, 0], &[3, 4, 1], &[5, 6, 0], &[7, 8, 18446744073709551615]],
        2,
        64,
        2,
        &[
            3,
            6,
            2,
            18446744073709551605,
            18446744073709551603,
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
            47244640287,
            18446744073709551615,
            562980018192380,
            18446744073709551614,
            18446744073709551601,
            18446744073709551615,
            18446744065119617016,
            18446744073709551614,
            18446744026464911349,
            0,
            18446181102281293824,
            18446744073709551615,
            18446744026464911361,
            0,
            18445055223849353217,
            0,
        ],
    );
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 3]],
        4,
        16,
        2,
        &[
            18446744073709551613,
            18446744073709551615,
            18446744073709551594,
            18446744073709551612,
            5,
            0,
            9223372036854775814,
            1,
            5,
            0,
            6,
            0,
            7,
            0,
            8,
            3,
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_ifft_truncate() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, trunc: usize, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        ifft_truncate(&mut ii, n, w, &mut t1, &mut t2, trunc);
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
        &[6, 0, 18446744073709551614, 18446744073709551615, 3, 0, 4, 1],
    );
    test(
        &[&[1, 2, 0], &[3, 4, 1], &[5, 6, 0], &[7, 8, 18446744073709551615]],
        2,
        64,
        2,
        &[
            8,
            12,
            2,
            18446744073709551612,
            18446744073709551611,
            18446744073709551613,
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
            17179869215,
            18446744073709551615,
            562958543355900,
            18446744073709551614,
            18446744073709551608,
            18446744073709551615,
            18446744065119617024,
            1,
            18446744056529682421,
            0,
            18446181123756130304,
            18446744073709551615,
            18446744056529682433,
            0,
            18446462598732906496,
            18446744073709551615,
        ],
    );
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 3]],
        4,
        16,
        2,
        &[12, 0, 18446744073709551612, 18446744073709551615, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 3],
    );
}

#[test]
fn ifft_truncate_properties() {
    large_type_gen_var_41().test_properties(|(ii, n, w, trunc)| {
        let limbs = ii[0].len() - 1;
        let p = fermat_modulus(limbs);
        let scale = Natural::from(n << 1);
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let xs = residues_mod(&ii, limbs);
        let scaled: Vec<Natural> = xs.iter().map(|x| x.mod_mul(&scale, &p)).collect();
        // ifft_truncate1 takes the first `trunc` outputs of a transform and the rest of its inputs,
        // times `2 * n`.
        let mut out = ii.clone();
        fft_radix2(&mut out, n, w, &mut t1, &mut t2);
        for (o, x) in out[trunc..].iter_mut().zip(scaled[trunc..].iter()) {
            *o = normalized_residue(x, limbs);
        }
        ifft_truncate1(&mut out, n, w, &mut t1, &mut t2, trunc);
        assert_eq!(residues_mod(&out[..trunc], limbs), &scaled[..trunc]);
        // ifft_truncate inverts fft_truncate, whose inputs past `trunc` are zero.
        let mut out = ii;
        for x in &mut out[trunc..] {
            x.fill(0);
        }
        fft_truncate(&mut out, n, w, &mut t1, &mut t2, trunc);
        ifft_truncate(&mut out, n, w, &mut t1, &mut t2, trunc);
        assert_eq!(residues_mod(&out[..trunc], limbs), &scaled[..trunc]);
    });
}
