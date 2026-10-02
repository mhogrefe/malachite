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
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul_middle::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_mul_middle::*;

#[test]
fn test_mod_mul_middle_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T, out: &[T]) {
        let mut result = vec![T::ZERO; out.len()];
        mod_mul_middle_to_out_classical(&mut result, xs, ys, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; out.len()];
        mod_mul_middle_to_out_karatsuba(&mut result, xs, ys, m);
        assert_eq!(result, out);
        assert_eq!(mod_mul_middle_naive(xs, ys, m), out);
    }
    // (x^2+3x+2)(2x^3+x^2+5x+1) = 2x^5+7x^4+12x^3+18x^2+13x+2; coefficients 2 and 3, modulo 7.
    test::<u8>(&[2, 3, 1], &[1, 5, 1, 2], 7, &[4, 5]);
    // A single coefficient is a scalar multiple of ys.
    test::<u8>(&[3], &[1, 2, 4], 7, &[3, 6, 5]);
    // A single output is a dot product.
    test::<u64>(&[1, 2, 3], &[4, 5, 6], 1000, &[28]);
}

// Classical and Karatsuba middle products against the schoolbook product, on both sides of the
// threshold, with lengths odd and even, and with more outputs than coefficients and fewer.
fn mod_mul_middle_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(n, k) in &[
        (1, 1),
        (3, 2),
        (2, 5),
        (31, 31),
        (32, 32),
        (33, 33),
        (64, 64),
        (65, 65),
        (100, 100),
        (200, 37),
        (37, 200),
        (129, 300),
        (300, 129),
        (600, 600),
    ] {
        for m in test_moduli::<T>() {
            let xs = mod_generated_coefficients::<T>(n, m, 11);
            let ys = mod_generated_coefficients::<T>(n + k - 1, m, 12);
            let expected = mod_mul_middle_naive(&xs, &ys, m);
            let mut out = vec![T::ZERO; k];
            mod_mul_middle_to_out_classical(&mut out, &xs, &ys, m);
            assert_eq!(out, expected);
            mod_mul_middle_to_out_karatsuba(&mut out, &xs, &ys, m);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_mul_middle_algorithms() {
    apply_fn_to_unsigneds!(mod_mul_middle_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_mul_middle_to_out_fail_1() {
    // ys has the wrong length.
    mod_mul_middle_to_out_karatsuba::<u8>(&mut [0; 2], &[1, 2], &[3, 4], 7);
}

#[test]
#[should_panic]
fn mod_mul_middle_to_out_fail_2() {
    // m is 0.
    mod_mul_middle_to_out_karatsuba::<u8>(&mut [0; 2], &[0, 0], &[0, 0, 0], 0);
}

fn mod_mul_middle_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            if xs.is_empty() || ys.len() < xs.len() {
                return;
            }
            let k = ys.len() - xs.len() + 1;
            let expected = mod_mul_middle_naive(xs, ys, m);
            let mut out = vec![T::ZERO; k];
            mod_mul_middle_to_out_classical(&mut out, xs, ys, m);
            assert_eq!(out, expected);
            mod_mul_middle_to_out_karatsuba(&mut out, xs, ys, m);
            assert_eq!(out, expected);
        },
    );
}

#[test]
fn mod_mul_middle_properties() {
    apply_fn_to_unsigneds!(mod_mul_middle_properties_helper);
}
