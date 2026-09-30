// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModMul;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_negacyclic::fft_negacyclic;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::ifft_negacyclic::ifft_negacyclic;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_43;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_ifft_negacyclic() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        ifft_negacyclic(&mut ii, n, w, &mut t1, &mut t2, &mut temp);
        assert_eq!(ii.concat(), out);
    };
    // - !w.odd()
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0]],
        2,
        32,
        &[
            10,
            0,
            281474976776191,
            18446744073709551615,
            17179869183,
            18446744073709551615,
            281474976776191,
            18446744073709551615,
        ],
    );
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 1]],
        2,
        32,
        &[
            10,
            1,
            281474976710655,
            18446744073709551615,
            12884901887,
            18446744073709551615,
            65536,
            0,
        ],
    );
    test(
        &[&[1, 2, 0], &[3, 4, 1], &[5, 6, 0], &[7, 8, 18446744073709551615]],
        2,
        64,
        &[
            16,
            20,
            0,
            4294967295,
            12884901888,
            18446744073709551615,
            18446744073709551607,
            9,
            18446744073709551615,
            18446744069414584319,
            21474836479,
            18446744073709551615,
        ],
    );
}

#[test]
fn ifft_negacyclic_properties() {
    large_type_gen_var_43().test_properties(|(mut ii, n, w)| {
        let limbs = ii[0].len() - 1;
        let xs = residues_mod(&ii, limbs);
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_negacyclic(&mut ii, n, w, &mut t1, &mut t2, &mut temp);
        ifft_negacyclic(&mut ii, n, w, &mut t1, &mut t2, &mut temp);
        let p = fermat_modulus(limbs);
        let scale = Natural::from(n << 1);
        for (x, y) in xs.iter().zip(ii.iter()) {
            assert_eq!(residue_mod(y, limbs), x.mod_mul(&scale, &p));
        }
    });
}

// With `n >= Limb::WIDTH`, `w` may be odd, which takes the square-root-of-2 paths. The inputs are
// too large to write out, so the transform is checked against its inverse.
#[test]
fn test_ifft_negacyclic_odd_w() {
    let test = |n: usize, w: u64| {
        let limbs = usize::try_from((u64::try_from(n).unwrap() * w) >> Limb::LOG_WIDTH).unwrap();
        let mut ii = pseudorandom_residues(n << 1, limbs, 2);
        let xs = residues_mod(&ii, limbs);
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_negacyclic(&mut ii, n, w, &mut t1, &mut t2, &mut temp);
        ifft_negacyclic(&mut ii, n, w, &mut t1, &mut t2, &mut temp);
        for (x, y) in xs.iter().zip(ii.iter()) {
            assert_eq!(
                residue_mod(y, limbs),
                fermat_mul_power_of_2(x, u64::from((n << 1).trailing_zeros()), limbs)
            );
        }
    };
    // - w.odd()
    test(Limb::WIDTH as usize, 1);
}
