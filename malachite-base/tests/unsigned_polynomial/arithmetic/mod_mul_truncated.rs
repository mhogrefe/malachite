// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul_truncated::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_mul_truncated::*;

#[test]
fn test_mod_mul_truncated_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], len: usize, m: T, out: &[T]) {
        let mut result = vec![T::ZERO; len];
        mod_mul_truncated_to_out(&mut result, xs, ys, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_mul_truncated_to_out_classical(&mut result, xs, ys, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_mul_truncated_to_out_karatsuba(&mut result, xs, ys, m);
        assert_eq!(result, out);
        assert_eq!(mod_mul_truncated_naive(xs, ys, len, m), out);
    }
    // (x^2+3x+2)(2x+5) = 2x^3+11x^2+19x+10, truncated and reduced modulo 7.
    test::<u8>(&[2, 3, 1], &[5, 2], 1, 7, &[3]);
    test::<u8>(&[2, 3, 1], &[5, 2], 2, 7, &[3, 5]);
    // A length past the end of the product pads with zeros.
    test::<u8>(&[2, 3, 1], &[5, 2], 6, 7, &[3, 5, 4, 2, 0, 0]);
    // (x - 1)(-x - 1) = 1 - x^2 modulo 2^64 - 1.
    test::<u64>(
        &[u64::MAX - 1, 1],
        &[u64::MAX - 1, u64::MAX - 1],
        2,
        u64::MAX,
        &[1, 0],
    );
}

fn mod_mul_truncated_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(n, m, len) in &[
        (1, 1, 1),
        (3, 2, 2),
        (31, 31, 40),
        (32, 32, 32),
        (33, 32, 64),
        (65, 33, 80),
        (80, 80, 80),
        (80, 80, 159),
        (100, 37, 70),
        (129, 128, 129),
        (129, 128, 300),
        (300, 260, 400),
    ] {
        for modulus in test_moduli::<T>() {
            let xs = mod_generated_coefficients::<T>(n, modulus, 1);
            let ys = mod_generated_coefficients::<T>(m, modulus, 2);
            let expected = mod_mul_truncated_naive(&xs, &ys, len, modulus);
            let mut out = vec![T::ZERO; len];
            mod_mul_truncated_to_out(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out(&mut out, &ys, &xs, modulus);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out_classical(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out_karatsuba(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_mul_truncated_algorithms() {
    apply_fn_to_unsigneds!(mod_mul_truncated_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_mul_truncated_to_out_fail_1() {
    mod_mul_truncated_to_out::<u8>(&mut [], &[1, 2], &[3], 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_to_out_fail_2() {
    mod_mul_truncated_to_out::<u8>(&mut [0; 2], &[1, 2], &[3], 0);
}

fn mod_mul_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
        |(p, q, len, m)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            let len = usize::try_from(len).unwrap();
            if xs.is_empty() || ys.is_empty() || len == 0 {
                return;
            }
            let expected = mod_mul_truncated_naive(xs, ys, len, m);
            let mut out = vec![T::ZERO; len];
            mod_mul_truncated_to_out(&mut out, xs, ys, m);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out_classical(&mut out, xs, ys, m);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out_karatsuba(&mut out, xs, ys, m);
            assert_eq!(out, expected);
            // It is the full product, truncated.
            let mut full = mod_mul_naive(xs, ys, m);
            full.resize(len, T::ZERO);
            assert_eq!(out, full);
        },
    );
}

#[test]
fn mod_mul_truncated_properties() {
    apply_fn_to_unsigneds!(mod_mul_truncated_properties_helper);
}
