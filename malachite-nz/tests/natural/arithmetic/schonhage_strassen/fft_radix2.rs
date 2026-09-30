// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAdd, ModSub};
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_radix2::*;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{large_type_gen_var_37, large_type_gen_var_40};
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_fft_butterfly() {
    let test = |i1: &[Limb],
                i2: &[Limb],
                i: usize,
                limbs: usize,
                w: u64,
                s_out: &[Limb],
                t_out: &[Limb]| {
        let mut s = vec![0; limbs + 1];
        let mut t = vec![0; limbs + 1];
        limbs_fft_butterfly(&mut s, &mut t, i1, i2, i, limbs, w);
        assert_eq!(s, s_out);
        assert_eq!(t, t_out);
    };
    test(
        &[1, 0],
        &[2, 0],
        0,
        1,
        1,
        &[3, 0],
        &[18446744073709551615, 18446744073709551615],
    );
    test(
        &[1, 0],
        &[2, 0],
        5,
        1,
        1,
        &[3, 0],
        &[18446744073709551585, 0],
    );
    test(
        &[1, 2, 0],
        &[3, 4, 1],
        1,
        2,
        64,
        &[3, 6, 0],
        &[2, 18446744073709551615, 18446744073709551615],
    );
    test(
        &[1, 2, 3],
        &[4, 5, 18446744073709551614],
        3,
        2,
        37,
        &[4, 7, 0],
        &[422212465065985, 18445618173802708992, 0],
    );
    test(
        &[18446744073709551615, 18446744073709551615, 1],
        &[1, 0, 0],
        2,
        2,
        50,
        &[18446744073709551615, 18446744073709551615, 0],
        &[1, 18446743798831644672, 0],
    );
}

#[test]
fn limbs_fft_butterfly_properties() {
    large_type_gen_var_37().test_properties(|(i1, i2, i, limbs, w)| {
        let p = fermat_modulus(limbs);
        let a = residue_mod(&i1, limbs);
        let b = residue_mod(&i2, limbs);
        let mut s = vec![0; limbs + 1];
        let mut t = vec![0; limbs + 1];
        limbs_fft_butterfly(&mut s, &mut t, &i1, &i2, i, limbs, w);
        assert_eq!(residue_mod(&s, limbs), (&a).mod_add(&b, &p));
        assert_eq!(
            residue_mod(&t, limbs),
            fermat_mul_power_of_2(&a.mod_sub(&b, &p), u64::try_from(i).unwrap() * w, limbs)
        );
    });
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_radix2() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        fft_radix2(&mut ii, n, w, &mut t1, &mut t2);
        assert_eq!(ii.concat(), out);
    };
    // - n == 1
    test(
        &[&[1, 0], &[2, 0]],
        1,
        64,
        &[3, 0, 18446744073709551615, 18446744073709551615],
    );
    // - n != 1
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 1]],
        2,
        32,
        &[
            10,
            1,
            18446744073709551614,
            18446744073709551614,
            18446744069414584319,
            0,
            4294967293,
            18446744073709551615,
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
            18446744073709551612,
            18446744073709551611,
            18446744073709551615,
            0,
            18446744073709551606,
            18446744073709551615,
            18446744073709551608,
            1,
            0,
        ],
    );
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 3]],
        4,
        16,
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
            18445618190982512636,
            18446744073709551615,
            1125917086777338,
            18446744073709551614,
        ],
    );
}

#[test]
fn fft_radix2_properties() {
    large_type_gen_var_40().test_properties(|(mut ii, n, w)| {
        let limbs = ii[0].len() - 1;
        let xs = residues_mod(&ii, limbs);
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        fft_radix2(&mut ii, n, w, &mut t1, &mut t2);
        let ys = fermat_dft(&xs, w, false, limbs);
        let bits = u64::try_from(n << 1).unwrap().trailing_zeros().into();
        for (k, y) in ys.iter().enumerate() {
            assert_eq!(&residue_mod(&ii[revbin(k, bits)], limbs), y);
        }
    });
}
