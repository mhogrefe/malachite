// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Square, ModPowerOf2SquareAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_square::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
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

#[test]
fn test_mod_power_of_2_square() {
    fn test<T: PrimitiveUnsigned>(s: &str, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let r = (&p).mod_power_of_2_square(pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().mod_power_of_2_square(pow), r);
        let mut s = p.clone();
        s.mod_power_of_2_square_assign(pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_power_of_2_square_polynomial_naive(&p, pow), r);
    }
    test::<u8>("0", 0, "0");
    test::<u8>("0", 3, "0");
    test::<u8>("x+1", 8, "x^2+2*x+1");
    // The square is x^4+6*x^3+13*x^2+12*x+4; its coefficients wrap around modulo 8.
    test::<u8>("x^2+3*x+2", 3, "x^4+6*x^3+5*x^2+4*x+4");
    // The square is 16*x^2+8*x+1, which is 1 modulo 8.
    test::<u8>("4*x+1", 3, "1");
    // (2x+1)^2 = 4x^2+4x+1, which is 1 modulo 4.
    test::<u8>("2*x+1", 2, "1");
    // The full width of the type: (x - 1)^2 = x^2 - 2x + 1.
    test::<u64>("x+18446744073709551615", 64, "x^2+18446744073709551614*x+1");
}

#[test]
#[should_panic]
fn mod_power_of_2_square_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_square(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_val_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_square(9);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_square(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_ref_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_square(9);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_assign_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_square_assign(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_assign_pow_fail() {
    // pow is greater than the width of the type.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_square_assign(9);
}

fn mod_power_of_2_square_public_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, _, pow)| {
            let r = (&p).mod_power_of_2_square(pow);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!(p.clone().mod_power_of_2_square(pow), r);
            let mut s = p.clone();
            s.mod_power_of_2_square_assign(pow);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, is the schoolbook square, and is the product of the polynomial
            // with itself.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(mod_power_of_2_square_polynomial_naive(&p, pow), r);
            assert_eq!((&p).mod_power_of_2_mul(&p, pow), r);
        },
    );
}

#[test]
fn mod_power_of_2_square_public_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_square_public_properties_helper);
}
