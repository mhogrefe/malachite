// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Square};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModPowerOf2MulTruncated, ModPowerOf2SquareTruncated, ModPowerOf2SquareTruncatedAssign,
    Polynomial,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_square_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_square_truncated::*;

#[test]
fn test_mod_power_of_2_square_truncated_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], len: usize, pow: u64, out: &[T]) {
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_square_truncated_to_out(&mut result, xs, pow);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_square_truncated_to_out_classical(&mut result, xs, pow);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_square_truncated_to_out_karatsuba(&mut result, xs, pow);
        assert_eq!(result, out);
        assert_eq!(mod_power_of_2_square_truncated_naive(xs, len, pow), out);
    }
    // (x^2+3x+2)^2 = x^4+6x^3+13x^2+12x+4, truncated and reduced modulo 8.
    test::<u8>(&[2, 3, 1], 1, 3, &[4]);
    test::<u8>(&[2, 3, 1], 3, 3, &[4, 4, 5]);
    // A length past the end of the square pads with zeros.
    test::<u8>(&[2, 3, 1], 6, 3, &[4, 4, 5, 6, 1, 0]);
    test::<u64>(&[u64::MAX, 1], 2, 64, &[1, u64::MAX - 1]);
}

fn mod_power_of_2_square_truncated_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(n, len) in &[
        (1, 1),
        (2, 2),
        (15, 20),
        (16, 16),
        (17, 31),
        (33, 40),
        (40, 40),
        (40, 79),
        (100, 70),
        (129, 129),
        (129, 300),
    ] {
        for pow in [0, 1, T::WIDTH >> 1, T::WIDTH - 1, T::WIDTH] {
            let xs = mod_power_of_2_generated_coefficients::<T>(n, pow, 4);
            let expected = mod_power_of_2_square_truncated_naive(&xs, len, pow);
            let mut out = vec![T::ZERO; len];
            mod_power_of_2_square_truncated_to_out(&mut out, &xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_square_truncated_to_out_classical(&mut out, &xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_square_truncated_to_out_karatsuba(&mut out, &xs, pow);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_power_of_2_square_truncated_algorithms() {
    apply_fn_to_unsigneds!(mod_power_of_2_square_truncated_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_truncated_to_out_fail() {
    mod_power_of_2_square_truncated_to_out::<u8>(&mut [0; 2], &[1, 2], 9);
}

fn mod_power_of_2_square_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(p, _, len, pow)| {
            let xs = p.coefficients_asc();
            let len = usize::try_from(len).unwrap();
            if xs.is_empty() || len == 0 {
                return;
            }
            let expected = mod_power_of_2_square_truncated_naive(xs, len, pow);
            let mut out = vec![T::ZERO; len];
            mod_power_of_2_square_truncated_to_out(&mut out, xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_square_truncated_to_out_classical(&mut out, xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_square_truncated_to_out_karatsuba(&mut out, xs, pow);
            assert_eq!(out, expected);
            let mut full = mod_power_of_2_mul_naive(xs, xs, pow);
            full.resize(len, T::ZERO);
            assert_eq!(out, full);
        },
    );
}

#[test]
fn mod_power_of_2_square_truncated_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_square_truncated_properties_helper);
}

#[test]
fn test_mod_power_of_2_square_truncated() {
    fn test<T: PrimitiveUnsigned>(s: &str, len: u64, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let r = (&p).mod_power_of_2_square_truncated(len, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().mod_power_of_2_square_truncated(len, pow), r);
        let mut s = p.clone();
        s.mod_power_of_2_square_truncated_assign(len, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(
            mod_power_of_2_square_truncated_polynomial_naive(&p, len, pow),
            r
        );
    }
    test::<u8>("0", 3, 3, "0");
    test::<u8>("x^2+3*x+2", 0, 3, "0");
    // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 8.
    test::<u8>("x^2+3*x+2", 3, 3, "5*x^2+4*x+4");
    // A length past the end of the square keeps all of it.
    test::<u8>("x^2+3*x+2", 10, 3, "x^4+6*x^3+5*x^2+4*x+4");
    // The linear coefficient of the square, 8, vanishes modulo 8.
    test::<u8>("4*x+1", 2, 3, "1");
    // The full width of the type: (x - 1)^2 = x^2 - 2x + 1.
    test::<u64>("x+18446744073709551615", 2, 64, "18446744073709551614*x+1");
}

#[test]
#[should_panic]
fn mod_power_of_2_square_truncated_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_square_truncated(2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_truncated_val_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_square_truncated(2, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_truncated_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_square_truncated(2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_truncated_ref_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_square_truncated(2, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_truncated_assign_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_square_truncated_assign(2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_truncated_assign_pow_fail() {
    // pow is greater than the width of the type.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_square_truncated_assign(2, 9);
}

fn mod_power_of_2_square_truncated_public_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(p, _, len, pow)| {
            let r = (&p).mod_power_of_2_square_truncated(len, pow);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!(p.clone().mod_power_of_2_square_truncated(len, pow), r);
            let mut s = p.clone();
            s.mod_power_of_2_square_truncated_assign(len, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, is the truncation of the whole square, and is the truncated
            // product of the polynomial with itself.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(
                mod_power_of_2_square_truncated_polynomial_naive(&p, len, pow),
                r
            );
            assert_eq!((&p).mod_power_of_2_square(pow).truncate(len), r);
            assert_eq!((&p).mod_power_of_2_mul_truncated(&p, len, pow), r);
        },
    );
}

#[test]
fn mod_power_of_2_square_truncated_public_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_square_truncated_public_properties_helper);
}
