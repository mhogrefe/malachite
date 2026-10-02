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
use malachite_base::unsigned_polynomial::arithmetic::mod_mul::*;

#[test]
fn test_mod_mul_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T, out: &[T]) {
        let len = xs.len() + ys.len() - 1;
        let mut result = vec![T::ZERO; len];
        mod_mul_to_out(&mut result, xs, ys, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_mul_to_out_classical(&mut result, xs, ys, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_mul_to_out_karatsuba(&mut result, xs, ys, m);
        assert_eq!(result, out);
        assert_eq!(mod_mul_naive(xs, ys, m), out);
    }
    test::<u8>(&[0], &[0], 1, &[0]);
    // (x^2+3x+2)(2x+5) = 2x^3+11x^2+19x+10, reduced modulo 7.
    test::<u8>(&[2, 3, 1], &[5, 2], 7, &[3, 5, 4, 2]);
    test::<u64>(&[2, 3, 1], &[5, 2], 1000, &[10, 19, 11, 2]);
    // (x - 1)(-x - 1) = 1 - x^2 modulo 255.
    test::<u8>(&[254, 1], &[254, 254], 255, &[1, 0, 254]);
    test::<u128>(&[u128::MAX - 1], &[2], u128::MAX, &[u128::MAX - 2]);
}

fn mod_mul_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(n, m) in &[
        (1, 1),
        (3, 2),
        (31, 31),
        (32, 32),
        (33, 32),
        (65, 33),
        (80, 80),
        (100, 37),
        (129, 128),
        (300, 260),
    ] {
        for modulus in test_moduli::<T>() {
            let xs = mod_generated_coefficients::<T>(n, modulus, 1);
            let ys = mod_generated_coefficients::<T>(m, modulus, 2);
            let expected = mod_mul_naive(&xs, &ys, modulus);
            let mut out = vec![T::ZERO; n + m - 1];
            mod_mul_to_out(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
            mod_mul_to_out(&mut out, &ys, &xs, modulus);
            assert_eq!(out, expected);
            mod_mul_to_out_classical(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
            mod_mul_to_out_karatsuba(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_mul_algorithms() {
    apply_fn_to_unsigneds!(mod_mul_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_mul_to_out_fail_1() {
    mod_mul_to_out::<u8>(&mut [0; 3], &[1, 2], &[3], 7);
}

#[test]
#[should_panic]
fn mod_mul_to_out_fail_2() {
    mod_mul_to_out::<u8>(&mut [0; 2], &[1, 2], &[3], 0);
}

fn mod_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            if xs.is_empty() || ys.is_empty() {
                return;
            }
            let expected = mod_mul_naive(xs, ys, m);
            let mut out = vec![T::ZERO; xs.len() + ys.len() - 1];
            mod_mul_to_out(&mut out, xs, ys, m);
            assert_eq!(out, expected);
            mod_mul_to_out(&mut out, ys, xs, m);
            assert_eq!(out, expected);
            mod_mul_to_out_classical(&mut out, xs, ys, m);
            assert_eq!(out, expected);
            mod_mul_to_out_karatsuba(&mut out, xs, ys, m);
            assert_eq!(out, expected);
        },
    );
}

#[test]
fn mod_mul_properties() {
    apply_fn_to_unsigneds!(mod_mul_properties_helper);
}
