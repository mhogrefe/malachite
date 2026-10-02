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
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_square::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_square::*;

#[test]
fn test_mod_power_of_2_square_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], pow: u64, out: &[T]) {
        let len = (xs.len() << 1) - 1;
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_square_to_out(&mut result, xs, pow);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_square_to_out_classical(&mut result, xs, pow);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_square_to_out_karatsuba(&mut result, xs, pow);
        assert_eq!(result, out);
        assert_eq!(mod_power_of_2_square_naive(xs, pow), out);
    }
    test::<u8>(&[0], 0, &[0]);
    // (x^2+3x+2)^2 = x^4+6x^3+13x^2+12x+4, reduced modulo 8.
    test::<u8>(&[2, 3, 1], 3, &[4, 4, 5, 6, 1]);
    test::<u64>(&[2, 3, 1], 64, &[4, 12, 13, 6, 1]);
    test::<u64>(&[u64::MAX, 1], 64, &[1, u64::MAX - 1, 1]);
}

fn mod_power_of_2_square_algorithms_helper<T: PrimitiveUnsigned>() {
    for n in [1, 2, 15, 16, 17, 33, 40, 100, 129] {
        for pow in [0, 1, T::WIDTH >> 1, T::WIDTH - 1, T::WIDTH] {
            let xs = mod_power_of_2_generated_coefficients::<T>(n, pow, 3);
            let expected = mod_power_of_2_square_naive(&xs, pow);
            let mut out = vec![T::ZERO; (n << 1) - 1];
            mod_power_of_2_square_to_out(&mut out, &xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_square_to_out_classical(&mut out, &xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_square_to_out_karatsuba(&mut out, &xs, pow);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_power_of_2_square_algorithms() {
    apply_fn_to_unsigneds!(mod_power_of_2_square_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_to_out_fail() {
    mod_power_of_2_square_to_out::<u8>(&mut [0; 3], &[1, 2], 9);
}

fn mod_power_of_2_square_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, _, pow)| {
            let xs = p.coefficients_asc();
            if xs.is_empty() {
                return;
            }
            let expected = mod_power_of_2_square_naive(xs, pow);
            let mut out = vec![T::ZERO; (xs.len() << 1) - 1];
            mod_power_of_2_square_to_out(&mut out, xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_square_to_out_classical(&mut out, xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_square_to_out_karatsuba(&mut out, xs, pow);
            assert_eq!(out, expected);
            assert_eq!(out, mod_power_of_2_mul_naive(xs, xs, pow));
        },
    );
}

#[test]
fn mod_power_of_2_square_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_square_properties_helper);
}
