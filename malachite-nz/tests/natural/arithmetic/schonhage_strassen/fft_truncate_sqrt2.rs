// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAdd, ModSub};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_truncate_sqrt2::*;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::ifft_truncate_sqrt2::*;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{large_type_gen_var_38, large_type_gen_var_42};
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_fft_butterfly_sqrt2() {
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
        limbs_fft_butterfly_sqrt2(&mut s, &mut t, i1, i2, i, limbs, w, &mut temp);
        assert_eq!(s, s_out);
        assert_eq!(t, t_out);
    };
    // - !negate
    test(
        &[1, 0],
        &[2, 0],
        1,
        1,
        1,
        &[3, 0],
        &[18446462598732906496, 18446744073709551615],
    );
    // - negate
    test(&[1, 0], &[2, 0], 101, 1, 1, &[3, 0], &[17179869180, 0]);
    test(
        &[1, 2, 3, 0],
        &[4, 5, 6, 1],
        7,
        3,
        3,
        &[5, 7, 9, 1],
        &[576460752504750080, 864691128656461824, 864691128320917504, 0],
    );
    test(
        &[18446744073709551615, 18446744073709551615, 18446744073709551615, 2],
        &[1, 0, 0, 18446744073709551613],
        127,
        3,
        3,
        &[0, 0, 0, 0],
        &[18446181123756130304, 18446744073709551615, 131071, 0],
    );
}

#[test]
fn limbs_fft_butterfly_sqrt2_properties() {
    large_type_gen_var_38().test_properties(|(i1, i2, i, limbs, w)| {
        let p = fermat_modulus(limbs);
        let a = residue_mod(&i1, limbs);
        let b = residue_mod(&i2, limbs);
        let mut s = vec![0; limbs + 1];
        let mut t = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        limbs_fft_butterfly_sqrt2(&mut s, &mut t, &i1, &i2, i, limbs, w, &mut temp);
        assert_eq!(residue_mod(&s, limbs), (&a).mod_add(&b, &p));
        assert_eq!(
            residue_mod(&t, limbs),
            fermat_mul_sqrt_2_power(&a.mod_sub(&b, &p), u64::try_from(i).unwrap() * w, limbs)
        );
    });
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_truncate_sqrt2() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, trunc: usize, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_truncate_sqrt2(&mut ii, n, w, &mut t1, &mut t2, &mut temp, trunc);
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
            18446744073709551614,
            18446744073709551615,
            18446744065119617023,
            0,
            8589934589,
            18446744073709551615,
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
            18446744069414584319,
            0,
            4294967293,
            18446744073709551615,
        ],
    );
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 3]],
        2,
        32,
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
}

#[test]
fn fft_truncate_sqrt2_properties() {
    large_type_gen_var_42().test_properties_with_limit(1000, |(mut ii, n, w, trunc)| {
        let limbs = ii[0].len() - 1;
        for x in &mut ii[trunc..] {
            x.fill(0);
        }
        let xs = residues_mod(&ii, limbs);
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_truncate_sqrt2(&mut ii, n, w, &mut t1, &mut t2, &mut temp, trunc);
        let ys = fermat_dft(&xs, w, true, limbs);
        let bits = u64::from((n << 2).trailing_zeros());
        for (k, y) in ys.iter().enumerate() {
            let i = revbin(k, bits);
            if i < trunc {
                assert_eq!(&residue_mod(&ii[i], limbs), y);
            }
        }
    });
}

// With `n >= Limb::WIDTH`, `w` may be odd, which takes the square-root-of-2 paths. The inputs are
// too large to write out, so the transform is checked against its inverse.
#[test]
fn test_fft_truncate_sqrt2_odd_w() {
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
