// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModAdd;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModComposeTruncated, ModPowerOf2Compose, ModPowerOf2ComposeTruncated, Polynomial,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::{
    mod_generated_coefficients, test_moduli,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_mul::{
    mod_mul_to_out, mod_mul_to_out_classical,
};
use malachite_base::unsigned_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_to_out;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_square::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_square_truncated::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_square::mod_square_to_out;
use malachite_base::unsigned_polynomial::arithmetic::mod_square_truncated::*;
use malachite_base::unsigned_polynomial::arithmetic::ntt::{
    mod_mul_truncated_many_by_fixed_ntt, mod_mul_truncated_ntt, mod_square_truncated_ntt,
    mul_truncated_many_by_fixed_ntt, mul_truncated_ntt, square_truncated_ntt,
};

// Checks the number-theoretic transform products against schoolbook multiplication, for every
// truncation of the product, with inputs that are reduced and with inputs that are not.
fn check<T: PrimitiveUnsigned>(len1: usize, len2: usize, pow: u64, seed: u64) {
    let xs = mod_power_of_2_generated_coefficients::<T>(len1, pow, seed);
    let ys = mod_power_of_2_generated_coefficients::<T>(len2, pow, seed + 1);
    let mut expected = vec![T::ZERO; len1 + len2 - 1];
    mod_power_of_2_mul_to_out_classical(&mut expected, &xs, &ys, pow);
    let mut square = vec![T::ZERO; len1 << 1 | 1];
    square.truncate((len1 << 1) - 1);
    mod_power_of_2_mul_to_out_classical(&mut square, &xs, &xs, pow);
    // The same inputs with their high bits set, which the products must ignore.
    let unreduced = |zs: &[T]| -> Vec<T> {
        zs.iter()
            .map(|&z| {
                if pow < T::WIDTH {
                    z | (T::MAX << pow)
                } else {
                    z
                }
            })
            .collect()
    };
    let (xs_high, ys_high) = (unreduced(&xs), unreduced(&ys));
    for out_len in [1, (len1 + len2) >> 1, len1 + len2 - 1] {
        for (a, b) in [(&xs, &ys), (&xs_high, &ys_high)] {
            let mut out = vec![T::ZERO; out_len];
            mul_truncated_ntt(&mut out, a, b, pow);
            for o in &mut out {
                o.mod_power_of_2_assign(pow);
            }
            assert_eq!(out, expected[..out_len], "{len1} {len2} {pow} {out_len}");
        }
    }
    for out_len in [1, len1, (len1 << 1) - 1] {
        let mut out = vec![T::ZERO; out_len];
        square_truncated_ntt(&mut out, &xs_high, pow);
        for o in &mut out {
            o.mod_power_of_2_assign(pow);
        }
        assert_eq!(out, square[..out_len], "{len1} {pow} {out_len}");
    }
}

fn test_ntt_helper<T: PrimitiveUnsigned>() {
    let mut seed = 0;
    for pow in [0, 1, 7, T::WIDTH >> 1, T::WIDTH - 1, T::WIDTH] {
        // - (1, 5), (60, 70), (1000, 1100): the product barely exceeds half the transform length,
        //   so the excess is wrapped around
        for &(len1, len2) in &[
            (1, 1),
            (1, 5),
            (2, 2),
            (3, 7),
            (16, 16),
            (17, 30),
            (100, 3),
            (129, 200),
            (60, 70),
            (1000, 1100),
        ] {
            seed += 2;
            check::<T>(len1, len2, pow, seed);
        }
    }
}

#[test]
fn test_ntt() {
    apply_fn_to_unsigneds!(test_ntt_helper);
    // - the wrapped excess is long enough to be multiplied by transforms itself
    check::<u64>(4800, 4800, 64, 1);
    check::<u64>(4800, 4800, 8, 3);
}

// The multiplication dispatchers, at lengths on both sides of the thresholds at which they switch
// to number-theoretic transforms, against schoolbook multiplication.
fn check_dispatchers<T: PrimitiveUnsigned>(len1: usize, len2: usize, pow: u64) {
    let xs = mod_power_of_2_generated_coefficients::<T>(len1, pow, 5);
    let ys = mod_power_of_2_generated_coefficients::<T>(len2, pow, 6);
    let mut expected = vec![T::ZERO; len1 + len2 - 1];
    mod_power_of_2_mul_to_out_classical(&mut expected, &xs, &ys, pow);
    let mut out = vec![T::ZERO; len1 + len2 - 1];
    mod_power_of_2_mul_to_out(&mut out, &xs, &ys, pow);
    assert_eq!(out, expected);
    let n = (len1 + len2) >> 1;
    let mut out = vec![T::ZERO; n];
    mod_power_of_2_mul_truncated_to_out(&mut out, &xs, &ys, pow);
    assert_eq!(out, expected[..n]);
    let mut square = vec![T::ZERO; (len1 << 1) - 1];
    mod_power_of_2_square_to_out_classical(&mut square, &xs, pow);
    let mut out = vec![T::ZERO; (len1 << 1) - 1];
    mod_power_of_2_square_to_out(&mut out, &xs, pow);
    assert_eq!(out, square);
    let mut out = vec![T::ZERO; len1];
    mod_power_of_2_square_truncated_to_out(&mut out, &xs, pow);
    assert_eq!(out, square[..len1]);
}

#[test]
fn test_mod_power_of_2_mul_dispatchers_ntt() {
    // - one prime, below and above the threshold, and with the product just above a power of 2
    check_dispatchers::<u64>(500, 500, 8);
    check_dispatchers::<u64>(900, 1000, 8);
    check_dispatchers::<u64>(1030, 1030, 8);
    // - two and three primes
    check_dispatchers::<u64>(2100, 2200, 32);
    check_dispatchers::<u64>(6200, 6300, 64);
    // - u128, with four and five primes
    check_dispatchers::<u128>(3600, 3600, 100);
    check_dispatchers::<u128>(6200, 6200, 128);
    // - narrower words
    check_dispatchers::<u8>(1100, 1200, 8);
    check_dispatchers::<u32>(2100, 2100, 32);
}

// A long composition, whose products use number-theoretic transforms, against Horner's rule with
// schoolbook products.
#[test]
fn test_mod_power_of_2_compose_truncated_ntt() {
    // - (1500, 1500, 1500): the Brent–Kung algorithm, multiplying by fixed factors by transforms
    // - (6, 1000, 3000): Horner's rule with a long inner polynomial, a fixed factor
    // - (2200, 2200, 2200) modulo 2^32: two primes
    // - (8, 3000, 3000): divide and conquer, reusing each level's power
    for &(len1, len2, n, pow) in &[
        (1500usize, 1500usize, 1500usize, 8u64),
        (40, 3, 3000, 8),
        (6, 1000, 3000, 8),
        (2200, 2200, 2200, 32),
        (8, 3000, 3000, 8),
        (8, 3000, 3000, 32),
    ] {
        let xs = mod_power_of_2_generated_coefficients::<u64>(len1, pow, 7);
        let mut ys = mod_power_of_2_generated_coefficients::<u64>(len2, pow, 8);
        ys[0] = 0;
        let mut acc: Vec<u64> = Vec::new();
        for &x in xs.iter().rev() {
            if !acc.is_empty() {
                let mut product = vec![0; acc.len() + ys.len() - 1];
                mod_power_of_2_mul_to_out_classical(&mut product, &acc, &ys, pow);
                product.truncate(n);
                acc = product;
            }
            if acc.is_empty() {
                acc.push(0);
            }
            acc[0] = (acc[0] + x) & ((1u64 << pow) - 1);
        }
        let expected = UnsignedPolynomial::from_coefficients_asc(acc);
        let p = UnsignedPolynomial::from_coefficients_asc(xs);
        let q = UnsignedPolynomial::from_coefficients_asc(ys);
        assert_eq!(
            p.mod_power_of_2_compose_truncated(q, n as u64, pow),
            expected
        );
    }
}

// Products by a fixed factor, with its transforms computed once, against schoolbook multiplication,
// for other factors of several lengths.
fn check_fixed<T: PrimitiveUnsigned>(n: usize, fixed_len: usize, lens: &[usize], pow: u64) {
    let ys = mod_power_of_2_generated_coefficients::<T>(fixed_len, pow, 11);
    let xss: Vec<Vec<T>> = lens
        .iter()
        .zip(20..)
        .map(|(&len, seed)| mod_power_of_2_generated_coefficients::<T>(len, pow, seed))
        .collect();
    let mut outs = vec![vec![T::ZERO; n]; lens.len()];
    mul_truncated_many_by_fixed_ntt(&mut outs, &xss, &ys, pow);
    for (out, xs) in outs.iter_mut().zip(&xss) {
        for o in out.iter_mut() {
            o.mod_power_of_2_assign(pow);
        }
        let mut expected = vec![T::ZERO; xs.len() + fixed_len - 1];
        mod_power_of_2_mul_to_out_classical(&mut expected, xs, &ys, pow);
        expected.resize(n, T::ZERO);
        assert_eq!(*out, expected[..n], "{n} {fixed_len} {} {pow}", xs.len());
    }
}

#[test]
fn test_mul_truncated_by_fixed_ntt() {
    // - nothing wraps around
    check_fixed::<u64>(300, 300, &[300, 7, 1], 8);
    // - the products slightly exceed half the transform length, so the excess wraps around, and is
    //   corrected for the longer other factors but not the shorter
    check_fixed::<u64>(514, 514, &[514, 513, 300, 1], 8);
    check_fixed::<u64>(514, 514, &[514, 400], 64);
    check_fixed::<u128>(514, 514, &[514, 2], 128);
    // - the products are longer than the half-length transform
    check_fixed::<u64>(1030, 3, &[1030, 1029, 5], 8);
    check_fixed::<u8>(1030, 3, &[1030], 8);
}

// Long whole compositions, whose products use number-theoretic transforms with reused factors,
// against Horner's rule with schoolbook products.
#[test]
fn test_mod_power_of_2_compose_ntt() {
    // - (16, 300): divide and conquer, reusing each level's power
    // - (6, 700): Horner's rule, reusing the inner polynomial
    for &(len1, len2, pow) in &[(16usize, 300usize, 8u64), (16, 300, 64), (6, 700, 8)] {
        let xs = mod_power_of_2_generated_coefficients::<u64>(len1, pow, 9);
        let ys = mod_power_of_2_generated_coefficients::<u64>(len2, pow, 10);
        let mask = if pow == 64 {
            u64::MAX
        } else {
            (1u64 << pow) - 1
        };
        let mut acc: Vec<u64> = vec![xs[len1 - 1]];
        for &x in xs[..len1 - 1].iter().rev() {
            let mut product = vec![0; acc.len() + ys.len() - 1];
            mod_power_of_2_mul_to_out_classical(&mut product, &acc, &ys, pow);
            acc = product;
            acc[0] = acc[0].wrapping_add(x) & mask;
        }
        let expected = UnsignedPolynomial::from_coefficients_asc(acc);
        let p = UnsignedPolynomial::from_coefficients_asc(xs);
        let q = UnsignedPolynomial::from_coefficients_asc(ys);
        assert_eq!(p.mod_power_of_2_compose(q, pow), expected);
    }
}

// Checks the number-theoretic transform products modulo `m` against schoolbook multiplication, for
// every truncation of the product.
fn check_mod<T: PrimitiveUnsigned>(len1: usize, len2: usize, m: T, seed: u64) {
    let xs = mod_generated_coefficients::<T>(len1, m, seed);
    let ys = mod_generated_coefficients::<T>(len2, m, seed + 1);
    let mut expected = vec![T::ZERO; len1 + len2 - 1];
    mod_mul_to_out_classical(&mut expected, &xs, &ys, m);
    let mut square = vec![T::ZERO; (len1 << 1) - 1];
    mod_mul_to_out_classical(&mut square, &xs, &xs, m);
    for out_len in [1, (len1 + len2) >> 1, len1 + len2 - 1] {
        let mut out = vec![T::ZERO; out_len];
        mod_mul_truncated_ntt(&mut out, &xs, &ys, m);
        assert_eq!(out, expected[..out_len], "{len1} {len2} {m} {out_len}");
    }
    for out_len in [1, len1, (len1 << 1) - 1] {
        let mut out = vec![T::ZERO; out_len];
        mod_square_truncated_ntt(&mut out, &xs, m);
        assert_eq!(out, square[..out_len], "{len1} {m} {out_len}");
    }
}

fn test_mod_ntt_helper<T: PrimitiveUnsigned>() {
    let mut seed = 0;
    for m in test_moduli::<T>() {
        // - (1, 5), (60, 70), (1000, 1100): the product barely exceeds half the transform length,
        //   so the excess is wrapped around, and corrected modulo m
        for &(len1, len2) in &[
            (1, 1),
            (1, 5),
            (2, 2),
            (3, 7),
            (16, 16),
            (17, 30),
            (100, 3),
            (129, 200),
            (60, 70),
            (1000, 1100),
        ] {
            seed += 2;
            check_mod::<T>(len1, len2, m, seed);
        }
    }
}

#[test]
fn test_mod_ntt() {
    apply_fn_to_unsigneds!(test_mod_ntt_helper);
    // - the wrapped excess is long enough to be multiplied by transforms itself
    check_mod::<u64>(4800, 4800, u64::MAX - 58, 1);
}

// The multiplication dispatchers modulo `m`, at lengths on both sides of the thresholds at which
// they switch to number-theoretic transforms, against schoolbook multiplication.
fn check_mod_dispatchers<T: PrimitiveUnsigned>(len1: usize, len2: usize, m: T) {
    let xs = mod_generated_coefficients::<T>(len1, m, 5);
    let ys = mod_generated_coefficients::<T>(len2, m, 6);
    let mut expected = vec![T::ZERO; len1 + len2 - 1];
    mod_mul_to_out_classical(&mut expected, &xs, &ys, m);
    let mut out = vec![T::ZERO; len1 + len2 - 1];
    mod_mul_to_out(&mut out, &xs, &ys, m);
    assert_eq!(out, expected);
    let n = (len1 + len2) >> 1;
    let mut out = vec![T::ZERO; n];
    mod_mul_truncated_to_out(&mut out, &xs, &ys, m);
    assert_eq!(out, expected[..n]);
    let mut square = vec![T::ZERO; (len1 << 1) - 1];
    mod_mul_to_out_classical(&mut square, &xs, &xs, m);
    let mut out = vec![T::ZERO; (len1 << 1) - 1];
    mod_square_to_out(&mut out, &xs, m);
    assert_eq!(out, square);
    let mut out = vec![T::ZERO; len1];
    mod_square_truncated_to_out(&mut out, &xs, m);
    assert_eq!(out, square[..len1]);
}

#[test]
fn test_mod_mul_dispatchers_ntt() {
    // - one prime, below and above the thresholds, and with the product just above a power of 2
    check_mod_dispatchers::<u64>(200, 200, 251);
    check_mod_dispatchers::<u64>(400, 420, 251);
    check_mod_dispatchers::<u64>(520, 520, 251);
    // - two and three primes
    check_mod_dispatchers::<u64>(800, 820, 4294967291);
    check_mod_dispatchers::<u64>(800, 850, u64::MAX - 58);
    // - u128, with four and five primes
    check_mod_dispatchers::<u128>(500, 500, (1 << 100) - 15);
    check_mod_dispatchers::<u128>(500, 500, u128::MAX - 158);
    // - narrower words
    check_mod_dispatchers::<u8>(500, 520, 251);
    check_mod_dispatchers::<u32>(800, 800, u32::MAX - 4);
}

// Products modulo `m` by a fixed factor, with its transforms computed once, against schoolbook
// multiplication, for other factors of several lengths.
fn check_mod_fixed<T: PrimitiveUnsigned>(n: usize, fixed_len: usize, lens: &[usize], m: T) {
    let ys = mod_generated_coefficients::<T>(fixed_len, m, 11);
    let xss: Vec<Vec<T>> = lens
        .iter()
        .zip(20..)
        .map(|(&len, seed)| mod_generated_coefficients::<T>(len, m, seed))
        .collect();
    let mut outs = vec![vec![T::ZERO; n]; lens.len()];
    mod_mul_truncated_many_by_fixed_ntt(&mut outs, &xss, &ys, m);
    for (out, xs) in outs.iter().zip(&xss) {
        let mut expected = vec![T::ZERO; xs.len() + fixed_len - 1];
        mod_mul_to_out_classical(&mut expected, xs, &ys, m);
        expected.resize(n, T::ZERO);
        assert_eq!(*out, expected[..n], "{n} {fixed_len} {} {m}", xs.len());
    }
}

#[test]
fn test_mod_mul_truncated_by_fixed_ntt() {
    // - nothing wraps around
    check_mod_fixed::<u64>(300, 300, &[300, 7, 1], 251);
    // - the products slightly exceed half the transform length, so the excess wraps around, and is
    //   corrected for the longer other factors but not the shorter
    check_mod_fixed::<u64>(514, 514, &[514, 513, 300, 1], 251);
    check_mod_fixed::<u64>(514, 514, &[514, 400], u64::MAX);
    check_mod_fixed::<u128>(514, 514, &[514, 2], u128::MAX - 158);
    // - the products are longer than the half-length transform
    check_mod_fixed::<u64>(1030, 3, &[1030, 1029, 5], 7);
    check_mod_fixed::<u8>(1030, 3, &[1030], 255);
    // - modulo 1
    check_mod_fixed::<u64>(514, 514, &[514, 400], 1);
}

// Long compositions modulo m, whose products use number-theoretic transforms with reused factors,
// against Horner's rule with schoolbook products.
#[test]
fn test_mod_compose_truncated_ntt() {
    // - (600, 600, 600): the Brent–Kung algorithm, multiplying by fixed factors by transforms
    // - (6, 1000, 3000): Horner's rule with a long inner polynomial, a fixed factor
    // - (8, 3000, 3000): divide and conquer, with one, two, and three primes
    for &(len1, len2, n, m) in &[
        (600usize, 600usize, 600usize, 251u64),
        (40, 3, 3000, 251),
        (6, 1000, 3000, 251),
        (8, 3000, 3000, 251),
        (8, 3000, 3000, 4294967291),
        (8, 3000, 3000, u64::MAX - 58),
    ] {
        let xs = mod_generated_coefficients::<u64>(len1, m, 7);
        let mut ys = mod_generated_coefficients::<u64>(len2, m, 8);
        ys[0] = 0;
        let mut acc: Vec<u64> = Vec::new();
        for &x in xs.iter().rev() {
            if !acc.is_empty() {
                let mut product = vec![0; acc.len() + ys.len() - 1];
                mod_mul_to_out_classical(&mut product, &acc, &ys, m);
                product.truncate(n);
                acc = product;
            }
            if acc.is_empty() {
                acc.push(0);
            }
            acc[0] = acc[0].mod_add(x, m);
        }
        let expected = UnsignedPolynomial::from_coefficients_asc(acc);
        let p = UnsignedPolynomial::from_coefficients_asc(xs);
        let q = UnsignedPolynomial::from_coefficients_asc(ys);
        assert_eq!(p.mod_compose_truncated(q, n as u64, m), expected);
    }
}
