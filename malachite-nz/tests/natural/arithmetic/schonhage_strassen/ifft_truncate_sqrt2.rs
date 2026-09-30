// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAdd, ModMul, ModSub};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_truncate_sqrt2::*;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::ifft_truncate_sqrt2::*;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{large_type_gen_var_38, large_type_gen_var_42};
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_ifft_butterfly_sqrt2() {
    let test = |i1: &[Limb],
                i2: &[Limb],
                i: usize,
                limbs: usize,
                w: u64,
                s_out: &[Limb],
                t_out: &[Limb]| {
        let mut s = vec![0; limbs + 1];
        let mut t = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        let mut i1 = i1.to_vec();
        let mut i2 = i2.to_vec();
        limbs_ifft_butterfly_sqrt2(&mut s, &mut t, &mut i1, &mut i2, i, limbs, w, &mut temp);
        assert_eq!(s, s_out);
        assert_eq!(t, t_out);
    };
    // - !negate
    // - b1 != 0
    test(
        &[1, 0],
        &[2, 0],
        1,
        1,
        1,
        &[281474976645120, 18446744073709551615],
        &[18446462598732906498, 0],
    );
    // - negate
    test(
        &[1, 0],
        &[2, 0],
        101,
        1,
        1,
        &[13835058056355905537, 18446744073709551615],
        &[4611686017353646081, 0],
    );
    test(
        &[1, 2, 3, 0],
        &[4, 5, 6, 1],
        7,
        3,
        3,
        &[18446743661392691041, 18446743386514784065, 18446743249075830882, 18446744073709551615],
        &[412316860577, 687194767554, 824633720739, 0],
    );
    test(
        &[18446744073709551615, 18446744073709551615, 18446744073709551615, 2],
        &[1, 0, 0, 18446744073709551613],
        127,
        3,
        3,
        &[2251799813685247, 0, 18446744073709027328, 2],
        &[18444492273895866367, 18446744073709551615, 524287, 3],
    );
    // - b1 == 0
    test(
        &[1, 0],
        &[2, 0],
        31,
        1,
        1,
        &[8589934590, 18446744073709551615],
        &[18446744065119617028, 0],
    );
}

#[test]
fn limbs_ifft_butterfly_sqrt2_properties() {
    large_type_gen_var_38().test_properties(|(mut i1, mut i2, i, limbs, w)| {
        let p = fermat_modulus(limbs);
        let a = residue_mod(&i1, limbs);
        let b = residue_mod(&i2, limbs);
        let mut s = vec![0; limbs + 1];
        let mut t = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        limbs_ifft_butterfly_sqrt2(&mut s, &mut t, &mut i1, &mut i2, i, limbs, w, &mut temp);
        let c = fermat_mul_sqrt_2_power(
            &b,
            (fermat_bits(limbs) << 2) - u64::try_from(i).unwrap() * w,
            limbs,
        );
        assert_eq!(residue_mod(&s, limbs), (&a).mod_add(&c, &p));
        assert_eq!(residue_mod(&t, limbs), a.mod_sub(c, p));
    });
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_ifft_truncate_sqrt2() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, trunc: usize, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        ifft_truncate_sqrt2(&mut ii, n, w, &mut t1, &mut t2, &mut temp, trunc);
        assert_eq!(ii.concat(), out);
    };
    // - w.even()
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0]],
        1,
        64,
        4,
        &[
            10,
            0,
            4294967294,
            18446744073709551615,
            18446744073709551612,
            18446744073709551615,
            18446744069414584320,
            0,
        ],
    );
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 1]],
        1,
        64,
        4,
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
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 3]],
        2,
        32,
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
}

#[test]
fn ifft_truncate_sqrt2_properties() {
    large_type_gen_var_42().test_properties(|(mut ii, n, w, trunc)| {
        let limbs = ii[0].len() - 1;
        for x in &mut ii[trunc..] {
            x.fill(0);
        }
        let xs = residues_mod(&ii, limbs);
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_truncate_sqrt2(&mut ii, n, w, &mut t1, &mut t2, &mut temp, trunc);
        ifft_truncate_sqrt2(&mut ii, n, w, &mut t1, &mut t2, &mut temp, trunc);
        let p = fermat_modulus(limbs);
        let scale = Natural::from(n << 2);
        for (x, y) in xs[..trunc].iter().zip(ii.iter()) {
            assert_eq!(residue_mod(y, limbs), x.mod_mul(&scale, &p));
        }
    });
}

// With `n >= Limb::WIDTH`, `w` may be odd, which takes the square-root-of-2 paths. The inputs are
// too large to write out, so the transform is checked against its inverse.
#[test]
fn test_ifft_truncate_sqrt2_odd_w() {
    let test = |n: usize, w: u64, trunc: usize| {
        let limbs = usize::try_from((u64::try_from(n).unwrap() * w) >> Limb::LOG_WIDTH).unwrap();
        let mut ii = pseudorandom_residues(n << 2, limbs, 1);
        for x in &mut ii[trunc..] {
            x.fill(0);
        }
        let xs = residues_mod(&ii, limbs);
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_truncate_sqrt2(&mut ii, n, w, &mut t1, &mut t2, &mut temp, trunc);
        ifft_truncate_sqrt2(&mut ii, n, w, &mut t1, &mut t2, &mut temp, trunc);
        for (x, y) in xs.iter().zip(ii[..trunc].iter()) {
            assert_eq!(
                residue_mod(y, limbs),
                fermat_mul_power_of_2(x, u64::from((n << 2).trailing_zeros()), limbs)
            );
        }
    };
    // - !w.even()
    test(Limb::WIDTH as usize, 1, (Limb::WIDTH as usize) * 3 + 10);
    test(Limb::WIDTH as usize, 3, (Limb::WIDTH as usize) << 2);
}
