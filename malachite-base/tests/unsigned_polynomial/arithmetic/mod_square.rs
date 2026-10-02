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
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_square::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_square::*;

#[test]
fn test_mod_square_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], m: T, out: &[T]) {
        let len = (xs.len() << 1) - 1;
        let mut result = vec![T::ZERO; len];
        mod_square_to_out(&mut result, xs, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_square_to_out_classical(&mut result, xs, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_square_to_out_karatsuba(&mut result, xs, m);
        assert_eq!(result, out);
        assert_eq!(mod_square_naive(xs, m), out);
    }
    test::<u8>(&[0], 1, &[0]);
    // (x^2+3x+2)^2 = x^4+6x^3+13x^2+12x+4, reduced modulo 7.
    test::<u8>(&[2, 3, 1], 7, &[4, 5, 6, 6, 1]);
    test::<u64>(&[2, 3, 1], 1000, &[4, 12, 13, 6, 1]);
    // (x - 1)^2 = x^2 - 2x + 1 modulo 255.
    test::<u8>(&[254, 1], 255, &[1, 253, 1]);
    test::<u128>(&[u128::MAX - 1], u128::MAX, &[1]);
}

fn mod_square_algorithms_helper<T: PrimitiveUnsigned>() {
    for n in [1, 2, 3, 63, 64, 65, 100, 129, 200, 300] {
        for modulus in test_moduli::<T>() {
            let xs = mod_generated_coefficients::<T>(n, modulus, 3);
            let expected = mod_square_naive(&xs, modulus);
            let mut out = vec![T::ZERO; (n << 1) - 1];
            mod_square_to_out(&mut out, &xs, modulus);
            assert_eq!(out, expected);
            mod_square_to_out_classical(&mut out, &xs, modulus);
            assert_eq!(out, expected);
            mod_square_to_out_karatsuba(&mut out, &xs, modulus);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_square_algorithms() {
    apply_fn_to_unsigneds!(mod_square_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_square_to_out_fail_1() {
    mod_square_to_out::<u8>(&mut [0; 2], &[1, 2], 7);
}

#[test]
#[should_panic]
fn mod_square_to_out_fail_2() {
    mod_square_to_out::<u8>(&mut [0; 3], &[1, 2], 0);
}

fn mod_square_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(p, _, m)| {
        let xs = p.coefficients_asc();
        if xs.is_empty() {
            return;
        }
        let expected = mod_square_naive(xs, m);
        let mut out = vec![T::ZERO; (xs.len() << 1) - 1];
        mod_square_to_out(&mut out, xs, m);
        assert_eq!(out, expected);
        mod_square_to_out_classical(&mut out, xs, m);
        assert_eq!(out, expected);
        mod_square_to_out_karatsuba(&mut out, xs, m);
        assert_eq!(out, expected);
        // It is the product of the polynomial with itself.
        assert_eq!(out, mod_mul_naive(xs, xs, m));
    });
}

#[test]
fn mod_square_properties() {
    apply_fn_to_unsigneds!(mod_square_properties_helper);
}
