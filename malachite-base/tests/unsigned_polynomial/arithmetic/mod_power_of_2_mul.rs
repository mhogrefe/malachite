// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;

#[test]
fn test_mod_power_of_2_mul_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], pow: u64, out: &[T]) {
        let len = xs.len() + ys.len() - 1;
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_mul_to_out(&mut result, xs, ys, pow);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_mul_to_out_classical(&mut result, xs, ys, pow);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_mul_to_out_karatsuba(&mut result, xs, ys, pow);
        assert_eq!(result, out);
        assert_eq!(mod_power_of_2_mul_naive(xs, ys, pow), out);
    }
    test::<u8>(&[0], &[0], 0, &[0]);
    // (x^2+3x+2)(2x+5) = 2x^3+11x^2+19x+10, reduced modulo 16.
    test::<u8>(&[2, 3, 1], &[5, 2], 4, &[10, 3, 11, 2]);
    test::<u64>(&[2, 3, 1], &[5, 2], 64, &[10, 19, 11, 2]);
    // The products wrap around modulo 2^64: (x - 1)(-x - 1) = 1 - x^2.
    test::<u64>(&[u64::MAX, 1], &[u64::MAX, u64::MAX], 64, &[1, 0, u64::MAX]);
    test::<u128>(&[u128::MAX], &[2], 128, &[u128::MAX - 1]);
}

fn mod_power_of_2_mul_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(n, m) in
        &[(1, 1), (3, 2), (15, 15), (16, 16), (17, 16), (33, 17), (40, 40), (100, 37), (129, 128)]
    {
        for pow in [0, 1, T::WIDTH >> 1, T::WIDTH - 1, T::WIDTH] {
            let xs = mod_power_of_2_generated_coefficients::<T>(n, pow, 1);
            let ys = mod_power_of_2_generated_coefficients::<T>(m, pow, 2);
            let expected = mod_power_of_2_mul_naive(&xs, &ys, pow);
            let mut out = vec![T::ZERO; n + m - 1];
            mod_power_of_2_mul_to_out(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out(&mut out, &ys, &xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out_classical(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out_karatsuba(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_power_of_2_mul_algorithms() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_to_out_fail_1() {
    mod_power_of_2_mul_to_out::<u8>(&mut [0; 2], &[1, 2], &[3], 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_to_out_fail_2() {
    mod_power_of_2_mul_to_out::<u8>(&mut [0; 3], &[1, 2], &[3], 4);
}

fn mod_power_of_2_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            if xs.is_empty() || ys.is_empty() {
                return;
            }
            let expected = mod_power_of_2_mul_naive(xs, ys, pow);
            let mut out = vec![T::ZERO; xs.len() + ys.len() - 1];
            mod_power_of_2_mul_to_out(&mut out, xs, ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out(&mut out, ys, xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out_classical(&mut out, xs, ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out_karatsuba(&mut out, xs, ys, pow);
            assert_eq!(out, expected);
        },
    );
}

#[test]
fn mod_power_of_2_mul_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_properties_helper);
}
