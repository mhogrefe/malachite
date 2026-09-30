// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAdd, ModSub};
use malachite_base::num::basic::integers::PrimitiveInt;
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
fn test_limbs_fft_butterfly_twiddle() {
    let test =
        |s: &[Limb], t: &[Limb], limbs: usize, b1: u64, b2: u64, u_out: &[Limb], v_out: &[Limb]| {
            let mut u = vec![0; limbs + 1];
            let mut v = vec![0; limbs + 1];
            limbs_fft_butterfly_twiddle(&mut u, &mut v, s, t, limbs, b1, b2);
            assert_eq!(u, u_out);
            assert_eq!(v, v_out);
        };
    // - !negate2
    // - !negate1
    // - !negate2
    // - !negate1
    test(
        &[1, 0],
        &[2, 0],
        1,
        0,
        0,
        &[3, 0],
        &[18446744073709551615, 18446744073709551615],
    );
    // - negate1
    // - negate1
    test(
        &[1, 0],
        &[2, 0],
        1,
        3,
        70,
        &[24, 0],
        &[63, 18446744073709551615],
    );
    // - negate2
    // - negate2
    test(
        &[1, 2, 0],
        &[3, 4, 1],
        2,
        130,
        255,
        &[18446744073709551604, 18446744073709551591, 18446744073709551615],
        &[18446744073709551615, 9223372036854775806, 18446744073709551615],
    );
    test(
        &[18446744073709551615, 18446744073709551615, 1],
        &[1, 0, 18446744073709551615],
        2,
        64,
        128,
        &[1, 18446744073709551615, 0],
        &[4, 0, 18446744073709551615],
    );
}

#[test]
fn limbs_fft_butterfly_twiddle_properties() {
    large_type_gen_var_39().test_properties(|(s, t, limbs, b1, b2)| {
        let p = fermat_modulus(limbs);
        let a = residue_mod(&s, limbs);
        let b = residue_mod(&t, limbs);
        let mut u = vec![0; limbs + 1];
        let mut v = vec![0; limbs + 1];
        limbs_fft_butterfly_twiddle(&mut u, &mut v, &s, &t, limbs, b1, b2);
        assert_eq!(
            residue_mod(&u, limbs),
            fermat_mul_power_of_2(&(&a).mod_add(&b, &p), b1, limbs)
        );
        assert_eq!(
            residue_mod(&v, limbs),
            fermat_mul_power_of_2(&a.mod_sub(b, p), b2, limbs)
        );
    });
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_radix2_twiddle() {
    let test = |ii: &[&[Limb]], n1: usize, n2: usize, w: u64, c: usize, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        fft_radix2_twiddle(
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
        &[1, 0, 6, 0, 3, 0, 18446744065119617025, 0],
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
            20,
            1,
            3,
            0,
            18446744060824649729,
            0,
            5,
            0,
            18445336698825736193,
            0,
            7,
            0,
            18445618173802381313,
            0,
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_truncate1_twiddle() {
    let test =
        |ii: &[&[Limb]], n1: usize, n2: usize, w: u64, c: usize, trunc: usize, out: &[Limb]| {
            let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
            let limbs = ii[0].len() - 1;
            let mut t1 = vec![0; limbs + 1];
            let mut t2 = vec![0; limbs + 1];
            fft_truncate1_twiddle(
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
        &[1, 0, 6, 0, 3, 0, 18446744065119617025, 0],
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
        &[1, 0, 20, 1, 3, 0, 18446744060824649729, 0, 5, 0, 6, 0, 7, 0, 8, 0],
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
            72,
            0,
            3,
            0,
            18446744039349813249,
            0,
            5,
            0,
            18444492273895342081,
            0,
            7,
            0,
            18444492273895342081,
            0,
            9,
            0,
            17870274525178886145,
            0,
            11,
            0,
            576451956076185600,
            0,
            13,
            0,
            18446744039349813249,
            0,
            15,
            0,
            18444492273895866369,
            0,
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_mfa_truncate_sqrt2_outer() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, n1: usize, trunc: usize, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_mfa_truncate_sqrt2_outer(&mut ii, n, w, &mut t1, &mut t2, &mut temp, n1, trunc);
        assert_eq!(ii.concat(), out);
    };
    // - (1usize << depth) >= n2
    // - (1usize << depth) < n2
    // - !w.odd()
    // - j >= trunc - two_n
    // - j < trunc - two_n
    // - j >= two_n
    // - j >= s
    // - j >= s
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0], &[5, 0], &[6, 0], &[7, 0], &[8, 0]],
        2,
        32,
        2,
        8,
        &[
            16,
            0,
            20,
            0,
            18446744073709551612,
            18446744073709551615,
            18446744056529682433,
            0,
            18446744056529682429,
            0,
            18445618173802446850,
            1,
            17179869179,
            18446744073709551615,
            18445618173802446849,
            0,
        ],
    );
    // - j < two_n
    // - j < s
    // - j < s
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
            36,
            0,
            42,
            0,
            30064771077,
            0,
            2251799814078464,
            0,
            18446744073709551610,
            18446744073709551615,
            18446744047939747841,
            0,
            18446744043644780549,
            18446744073709551615,
            1688849860788224,
            0,
            1970346311286777,
            1,
            576467349238970370,
            2,
            21474836480,
            0,
            6597069766656,
            0,
            18444773770347937783,
            18446744073709551614,
            576451956344617472,
            0,
            1970324836974592,
            0,
            576460752303423488,
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
            16,
            0,
            12,
            1,
            18446744073709551612,
            18446744073709551615,
            21474836480,
            0,
            18446744056529682429,
            0,
            844424929869825,
            1,
            17179869179,
            18446744073709551615,
            18445618173802905601,
            0,
        ],
    );
}

#[test]
fn fft_radix2_twiddle_properties() {
    large_type_gen_var_44().test_properties(|(mut ii, n1, n2, w, c)| {
        let limbs = ii[0].len() - 1;
        let xs = residues_mod(&column(&ii, n1, c), limbs);
        let wn1 = w * u64::try_from(n1).unwrap();
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        fft_radix2_twiddle(&mut ii[c..], n1, n2 >> 1, wn1, &mut t1, &mut t2, w, 0, c, 1);
        let ys = fermat_dft(&xs, wn1, false, limbs);
        let bits = u64::from(n2.trailing_zeros());
        let out = column(&ii, n1, c);
        for (j, o) in out.iter().enumerate() {
            let r = revbin(j, bits);
            let e = w * u64::try_from(r * c).unwrap();
            assert_eq!(
                residue_mod(o, limbs),
                fermat_mul_power_of_2(&ys[r], e, limbs)
            );
        }
        // The other columns are unchanged.
        let mut t = t1.clone();
        t.fill(0);
    });
}

#[test]
fn fft_truncate1_twiddle_properties() {
    large_type_gen_var_45().test_properties(|(ii, n1, n2, w, c, trunc)| {
        let limbs = ii[0].len() - 1;
        let wn1 = w * u64::try_from(n1).unwrap();
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut full = ii.clone();
        fft_radix2_twiddle(
            &mut full[c..],
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
        let mut out = ii;
        fft_truncate1_twiddle(
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
            residues_mod(&column(&full, n1, c)[..trunc], limbs)
        );
    });
}

#[test]
fn fft_mfa_truncate_sqrt2_outer_properties() {
    // Without the row transforms in between, the column transforms and their inverses multiply by
    // `n1` less than the normalization divides by.
    large_type_gen_var_46().test_properties(|(mut ii, n, w, n1, trunc)| {
        let limbs = ii[0].len() - 1;
        for x in &mut ii[trunc..] {
            x.fill(0);
        }
        let xs = residues_mod(&ii, limbs);
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_mfa_truncate_sqrt2_outer(&mut ii, n, w, &mut t1, &mut t2, &mut temp, n1, trunc);
        ifft_mfa_truncate_sqrt2_outer(&mut ii, n, w, &mut t1, &mut t2, &mut temp, n1, trunc);
        let e = u64::from(n1.trailing_zeros());
        for (x, y) in xs[..trunc].iter().zip(ii.iter()) {
            assert!(residue_is_normalized(y, limbs));
            assert_eq!(residue_mod(y, limbs), fermat_div_power_of_2(x, e, limbs));
        }
    });
}

// With `n >= Limb::WIDTH`, `w` may be odd, which takes the square-root-of-2 paths. The inputs are
// too large to write out, so the transform is checked against its inverse, which, without the row
// transforms in between, divides by `n1`.
#[test]
fn test_fft_mfa_truncate_sqrt2_outer_odd_w() {
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
    // - w.odd()
    // - j >= trunc - two_n
    // - j < trunc - two_n
    // - !j.odd()
    // - j.odd()
    // - j >= two_n
    // - j < two_n
    // - !i.odd()
    // - i.odd()
    // - j >= s
    // - j < s
    // - j >= s
    // - j < s
    test(n, 1, 8, (n << 1) + 48);
    test(n, 1, 8, n << 2);
}
