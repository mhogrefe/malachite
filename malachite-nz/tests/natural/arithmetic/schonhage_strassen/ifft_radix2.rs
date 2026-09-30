// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAdd, ModMul, ModSub};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_radix2::fft_radix2;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::ifft_radix2::*;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{large_type_gen_var_37, large_type_gen_var_40};
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_ifft_butterfly() {
    let test = |i1: &[Limb],
                i2: &[Limb],
                i: usize,
                limbs: usize,
                w: u64,
                s_out: &[Limb],
                t_out: &[Limb]| {
        let mut s = vec![0; limbs + 1];
        let mut t = vec![0; limbs + 1];
        let mut i1 = i1.to_vec();
        let mut i2 = i2.to_vec();
        limbs_ifft_butterfly(&mut s, &mut t, &mut i1, &mut i2, i, limbs, w);
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
        &[17293822569102704641, 18446744073709551615],
        &[1152921504606846977, 0],
    );
    test(
        &[1, 2, 0],
        &[3, 4, 1],
        1,
        2,
        64,
        &[5, 0, 0],
        &[18446744073709551613, 3, 0],
    );
    test(
        &[1, 2, 3],
        &[4, 5, 18446744073709551614],
        3,
        2,
        37,
        &[18446744073708765185, 18446744073708896257, 2],
        &[786433, 655362, 3],
    );
    test(
        &[18446744073709551615, 18446744073709551615, 1],
        &[1, 0, 0],
        2,
        2,
        50,
        &[18446744073441116159, 18446744073709551615, 1],
        &[268435455, 0, 2],
    );
}

#[test]
fn limbs_ifft_butterfly_properties() {
    large_type_gen_var_37().test_properties(|(mut i1, mut i2, i, limbs, w)| {
        let p = fermat_modulus(limbs);
        let a = residue_mod(&i1, limbs);
        let b = residue_mod(&i2, limbs);
        let mut s = vec![0; limbs + 1];
        let mut t = vec![0; limbs + 1];
        limbs_ifft_butterfly(&mut s, &mut t, &mut i1, &mut i2, i, limbs, w);
        let c = fermat_div_power_of_2(&b, u64::try_from(i).unwrap() * w, limbs);
        assert_eq!(residue_mod(&s, limbs), (&a).mod_add(&c, &p));
        assert_eq!(residue_mod(&t, limbs), a.mod_sub(c, p));
    });
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_ifft_radix2() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        ifft_radix2(&mut ii, n, w, &mut t1, &mut t2);
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
            18446744073709551612,
            18446744073709551614,
            0,
            1,
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
            18446744073709551611,
            0,
            18446744073709551614,
            18446744073709551608,
            18446744073709551607,
            1,
            1,
            18446744073709551611,
            18446744073709551615,
        ],
    );
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 3]],
        4,
        16,
        &[
            36,
            3,
            281479271546877,
            18446744073709551614,
            4294967291,
            18446744073709551615,
            18446181119461228544,
            0,
            18446744073709551600,
            18446744073709551612,
            18446462603027939327,
            18446744073709551615,
            18446744069414584317,
            0,
            562945658388480,
            1,
        ],
    );
}

#[test]
fn ifft_radix2_properties() {
    large_type_gen_var_40().test_properties(|(mut ii, n, w)| {
        let limbs = ii[0].len() - 1;
        let xs = residues_mod(&ii, limbs);
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        fft_radix2(&mut ii, n, w, &mut t1, &mut t2);
        ifft_radix2(&mut ii, n, w, &mut t1, &mut t2);
        let p = fermat_modulus(limbs);
        let scale = Natural::from(n << 1);
        for (x, y) in xs.iter().zip(ii.iter()) {
            assert_eq!(residue_mod(y, limbs), x.mod_mul(&scale, &p));
        }
    });
}
