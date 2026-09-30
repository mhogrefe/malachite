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
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_mfa_truncate_sqrt2::*;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::ifft_mfa_truncate_sqrt2::*;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    large_type_gen_var_39, large_type_gen_var_44, large_type_gen_var_45, large_type_gen_var_46,
};
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

// Column `c` of a matrix with `n1` columns.
fn column(ii: &[Vec<Limb>], n1: usize, c: usize) -> Vec<Vec<Limb>> {
    ii.iter().skip(c).step_by(n1).cloned().collect()
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_ifft_butterfly_twiddle() {
    let test =
        |s: &[Limb], t: &[Limb], limbs: usize, b1: u64, b2: u64, u_out: &[Limb], v_out: &[Limb]| {
            let mut u = vec![0; limbs + 1];
            let mut v = vec![0; limbs + 1];
            let mut s = s.to_vec();
            let mut t = t.to_vec();
            limbs_ifft_butterfly_twiddle(&mut u, &mut v, &mut s, &mut t, limbs, b1, b2);
            assert_eq!(u, u_out);
            assert_eq!(v, v_out);
        };
    // - !negate1
    // - !negate2
    // - !negate1
    // - !negate2
    test(
        &[1, 0],
        &[2, 0],
        1,
        0,
        0,
        &[3, 0],
        &[18446744073709551615, 18446744073709551615],
    );
    // - negate2
    // - negate2
    test(
        &[1, 0],
        &[2, 0],
        1,
        3,
        70,
        &[16717361816799281151, 18446744073709551614],
        &[15564440312192434177, 0],
    );
    // - negate1
    // - negate1
    test(
        &[1, 2, 0],
        &[3, 4, 1],
        2,
        130,
        255,
        &[9223372036854775810, 4611686018427387911, 18446744073709551614],
        &[9223372036854775804, 4611686018427387895, 0],
    );
    test(
        &[18446744073709551615, 18446744073709551615, 1],
        &[1, 0, 18446744073709551615],
        2,
        64,
        128,
        &[18446744073709551614, 2, 0],
        &[0, 3, 18446744073709551614],
    );
}

#[test]
fn limbs_ifft_butterfly_twiddle_properties() {
    large_type_gen_var_39().test_properties(|(mut s, mut t, limbs, b1, b2)| {
        let p = fermat_modulus(limbs);
        let a = residue_mod(&s, limbs);
        let b = residue_mod(&t, limbs);
        let mut u = vec![0; limbs + 1];
        let mut v = vec![0; limbs + 1];
        limbs_ifft_butterfly_twiddle(&mut u, &mut v, &mut s, &mut t, limbs, b1, b2);
        let x = fermat_div_power_of_2(&a, b1, limbs);
        let y = fermat_div_power_of_2(&b, b2, limbs);
        assert_eq!(residue_mod(&u, limbs), (&x).mod_add(&y, &p));
        assert_eq!(residue_mod(&v, limbs), x.mod_sub(y, p));
    });
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_ifft_radix2_twiddle() {
    let test = |ii: &[&[Limb]], n1: usize, n2: usize, w: u64, c: usize, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        ifft_radix2_twiddle(
            &mut ii[c..],
            n1,
            n2 >> 1,
            w * u64::try_from(n1).unwrap(),
            &mut t1,
            &mut t2,
            w,
            0,
            c,
            1,
        );
        assert_eq!(ii.concat(), out);
    };
    // - n == 1
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0]],
        2,
        2,
        32,
        1,
        &[1, 0, 18446744056529682434, 18446744073709551615, 3, 0, 17179869186, 0],
    );
    // - n != 1
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 1], &[5, 0], &[6, 0], &[7, 0], &[8, 0]],
        2,
        4,
        16,
        1,
        &[
            1,
            0,
            18445055210963861506,
            18446744073709551615,
            3,
            0,
            18444492286780375042,
            18446744073709551615,
            5,
            0,
            1688836975886338,
            0,
            7,
            0,
            2251812698980354,
            0,
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_ifft_truncate1_twiddle() {
    let test =
        |ii: &[&[Limb]], n1: usize, n2: usize, w: u64, c: usize, trunc: usize, out: &[Limb]| {
            let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
            let limbs = ii[0].len() - 1;
            let mut t1 = vec![0; limbs + 1];
            let mut t2 = vec![0; limbs + 1];
            ifft_truncate1_twiddle(
                &mut ii[c..],
                n1,
                n2 >> 1,
                w * u64::try_from(n1).unwrap(),
                &mut t1,
                &mut t2,
                w,
                0,
                c,
                1,
                trunc,
            );
            assert_eq!(ii.concat(), out);
        };
    // - trunc == n << 1
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0]],
        2,
        2,
        32,
        1,
        2,
        &[1, 0, 18446744056529682434, 18446744073709551615, 3, 0, 17179869186, 0],
    );
    // - trunc != n << 1
    // - trunc <= n
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 1], &[5, 0], &[6, 0], &[7, 0], &[8, 0]],
        2,
        4,
        16,
        1,
        2,
        &[
            1,
            0,
            18446744047939747838,
            18446744073709551615,
            3,
            0,
            25769803772,
            0,
            5,
            0,
            6,
            0,
            7,
            0,
            8,
            0,
        ],
    );
    // - trunc > n
    test(
        &[
            &[1, 0],
            &[2, 0],
            &[3, 0],
            &[4, 0],
            &[5, 0],
            &[6, 0],
            &[7, 0],
            &[8, 0],
            &[9, 0],
            &[10, 0],
            &[11, 0],
            &[12, 0],
            &[13, 0],
            &[14, 0],
            &[15, 0],
            &[16, 0],
        ],
        2,
        8,
        8,
        1,
        6,
        &[
            1,
            0,
            17001651577233997822,
            18446744073709551615,
            3,
            0,
            18442781511112595462,
            18446744073709551615,
            5,
            0,
            3377665361838070,
            0,
            7,
            0,
            4503633987895284,
            0,
            9,
            0,
            1441714762394238982,
            0,
            11,
            0,
            18446203071038089214,
            18446744073709551615,
            13,
            0,
            2251748273684484,
            0,
            15,
            0,
            18442803389675601915,
            0,
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_ifft_mfa_truncate_sqrt2_outer() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, n1: usize, trunc: usize, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        ifft_mfa_truncate_sqrt2_outer(&mut ii, n, w, &mut t1, &mut t2, &mut temp, n1, trunc);
        assert_eq!(ii.concat(), out);
    };
    // - (1usize << depth) >= n2
    // - (1usize << depth) < n2
    // - (1usize << depth2) >= n1
    // - (1usize << depth2) < n1
    // - j >= s
    // - j >= s
    // - !w.odd()
    // - j >= trunc - two_n
    // - j < trunc - two_n
    // - j >= two_n
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 0]],
        2,
        32,
        2,
        8,
        &[
            2,
            0,
            13834846946902081537,
            0,
            4611686019501129728,
            0,
            13834776582452887553,
            0,
            0,
            1,
            13835269159367278593,
            0,
            4611686017353646080,
            0,
            13835339532406407169,
            0,
        ],
    );
    // - j < s
    // - j < s
    // - !w.odd()
    // - j < two_n
    test(
        &[
            &[1, 0],
            &[2, 0],
            &[3, 0],
            &[4, 0],
            &[5, 0],
            &[6, 0],
            &[7, 0],
            &[8, 0],
            &[9, 0],
            &[10, 0],
            &[11, 0],
            &[12, 0],
            &[0, 0],
            &[0, 0],
            &[0, 0],
            &[0, 0],
        ],
        4,
        16,
        2,
        12,
        &[
            16140901065569599491,
            0,
            4521402917499944960,
            0,
            9223055379653459968,
            0,
            9223159557306466305,
            0,
            9223372036854775808,
            0,
            13835198789549359105,
            0,
            9223372034707292160,
            0,
            13835339533480132609,
            0,
            2305843008139952128,
            0,
            9313514397072572417,
            0,
            316659348799488,
            0,
            4611617026220179456,
            0,
            18446744056529682433,
            0,
            576462951259571712,
            0,
            18445618173802971137,
            0,
            144106391882169344,
            0,
        ],
    );
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 1], &[5, 0], &[6, 0], &[7, 0], &[0, 0]],
        2,
        32,
        2,
        8,
        &[
            2,
            0,
            13834846947439017985,
            0,
            4611686019501129728,
            0,
            13835058056892727297,
            0,
            0,
            1,
            13835269159904083969,
            0,
            4611686017353646080,
            0,
            13835058056892825601,
            0,
        ],
    );
}

#[test]
fn ifft_radix2_twiddle_properties() {
    large_type_gen_var_44().test_properties(|(mut ii, n1, n2, w, c)| {
        let limbs = ii[0].len() - 1;
        let xs = residues_mod(&column(&ii, n1, c), limbs);
        let wn1 = w * u64::try_from(n1).unwrap();
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        fft_radix2_twiddle(&mut ii[c..], n1, n2 >> 1, wn1, &mut t1, &mut t2, w, 0, c, 1);
        ifft_radix2_twiddle(&mut ii[c..], n1, n2 >> 1, wn1, &mut t1, &mut t2, w, 0, c, 1);
        let p = fermat_modulus(limbs);
        let scale = Natural::from(n2);
        for (x, y) in xs.iter().zip(column(&ii, n1, c).iter()) {
            assert_eq!(residue_mod(y, limbs), x.mod_mul(&scale, &p));
        }
    });
}

#[test]
fn ifft_truncate1_twiddle_properties() {
    large_type_gen_var_45().test_properties(|(ii, n1, n2, w, c, trunc)| {
        let limbs = ii[0].len() - 1;
        let p = fermat_modulus(limbs);
        let scale = Natural::from(n2);
        let xs = residues_mod(&column(&ii, n1, c), limbs);
        let scaled: Vec<Natural> = xs.iter().map(|x| x.mod_mul(&scale, &p)).collect();
        let wn1 = w * u64::try_from(n1).unwrap();
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut out = ii;
        fft_radix2_twiddle(
            &mut out[c..],
            n1,
            n2 >> 1,
            wn1,
            &mut t1,
            &mut t2,
            w,
            0,
            c,
            1,
        );
        for (j, x) in scaled.iter().enumerate().skip(trunc) {
            out[c + j * n1] = normalized_residue(x, limbs);
        }
        ifft_truncate1_twiddle(
            &mut out[c..],
            n1,
            n2 >> 1,
            wn1,
            &mut t1,
            &mut t2,
            w,
            0,
            c,
            1,
            trunc,
        );
        assert_eq!(
            residues_mod(&column(&out, n1, c)[..trunc], limbs),
            &scaled[..trunc]
        );
    });
}

#[test]
fn ifft_mfa_truncate_sqrt2_outer_properties() {
    large_type_gen_var_46().test_properties(|(ii, n, w, n1, trunc)| {
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        let mut out = ii;
        ifft_mfa_truncate_sqrt2_outer(&mut out, n, w, &mut t1, &mut t2, &mut temp, n1, trunc);
        for x in &out[..trunc] {
            assert!(residue_is_normalized(x, limbs));
        }
    });
}

// With `n >= Limb::WIDTH`, `w` may be odd, which takes the square-root-of-2 paths. The inputs are
// too large to write out, so the transform is checked against its inverse, which, without the row
// transforms in between, divides by `n1`.
#[test]
fn test_ifft_mfa_truncate_sqrt2_outer_odd_w() {
    let test = |n: usize, w: u64, n1: usize, trunc: usize| {
        let limbs = usize::try_from((u64::try_from(n).unwrap() * w) >> Limb::LOG_WIDTH).unwrap();
        let mut ii = pseudorandom_residues(n << 2, limbs, 3);
        for x in &mut ii[trunc..] {
            x.fill(0);
        }
        let xs = residues_mod(&ii, limbs);
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_mfa_truncate_sqrt2_outer(&mut ii, n, w, &mut t1, &mut t2, &mut temp, n1, trunc);
        ifft_mfa_truncate_sqrt2_outer(&mut ii, n, w, &mut t1, &mut t2, &mut temp, n1, trunc);
        for (x, y) in xs.iter().zip(ii[..trunc].iter()) {
            assert_eq!(
                residue_mod(y, limbs),
                fermat_div_power_of_2(x, u64::from(n1.trailing_zeros()), limbs)
            );
        }
    };
    let n = Limb::WIDTH as usize;
    // - (1usize << depth) >= n2
    // - (1usize << depth) < n2
    // - (1usize << depth2) >= n1
    // - (1usize << depth2) < n1
    // - j >= s
    // - j < s
    // - j >= s
    // - j < s
    // - w.odd()
    // - !i.odd()
    // - i.odd()
    // - w.odd()
    // - j >= trunc - two_n
    // - j < trunc - two_n
    // - !j.odd()
    // - j.odd()
    // - j >= two_n
    // - j < two_n
    test(n, 1, 8, (n << 1) + 48);
    test(n, 1, 8, n << 2);
}
