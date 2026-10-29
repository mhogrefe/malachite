// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    ModInverseTruncated, ModMulTruncated, ModSquareTruncated, Polynomial,
};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_8,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_14,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_inverse_truncated::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the inverse modulo m as a power series of `p`, truncated to `len` coefficients, in every
// way of calling it, against the naive version and each algorithm, checks that it exists exactly
// when the constant coefficient is invertible and that its product with `p` is 1, and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    let r = p.mod_inverse_truncated(len, m);
    assert_eq!(p.clone().mod_inverse_truncated(len, m), r);
    assert_eq!(mod_inverse_truncated_naive(p, len, m), r);
    assert_eq!(mod_inverse_truncated_basecase_wrapper(p, len, m), r);
    assert_eq!(mod_inverse_truncated_newton_wrapper(p, len, m), r);
    let invertible = len == 0 || m == T::ONE || p.coefficient(0).coprime_with(m);
    assert_eq!(r.is_some(), invertible);
    if let Some(q) = &r {
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        assert!(q.len() <= len);
        assert_eq!(
            p.mod_mul_truncated(q, len, m),
            UnsignedPolynomial::one().truncate(len) % m
        );
    }
    r
}

fn test_mod_inverse_truncated_helper<T: PrimitiveUnsigned>() {
    let test = |p, len, m: u8, out: Option<&str>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        assert_eq!(
            verify(&p, len, T::from(m)).map(|q| q.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - the length is 0
    test("3*x+5", 0, 7, Some("0"));
    // - modulo 1
    test("0", 5, 1, Some("0"));
    // - the polynomial is 0
    test("0", 3, 7, None);
    // - the constant coefficient is not invertible
    test("2*x+4", 3, 6, None);
    // - the constant coefficient is 0
    test("3*x", 3, 7, None);
    // - length 1: only the constant coefficient
    test("3*x+5", 1, 7, Some("3"));
    // - a constant polynomial
    test("3", 5, 7, Some("5"));
    // - the constant coefficient is 1
    test("x+1", 5, 7, Some("x^4+6*x^3+x^2+6*x+1"));
    // - the constant coefficient is not 1
    test("x^2+3*x+2", 6, 7, Some("6*x^4+3*x^3+x+4"));
    // - only the first coefficients are read
    test("5*x^4+x+3", 2, 7, Some("3*x+5"));
    // - the inverse has zeros at the end
    test("x^2+1", 4, 7, Some("6*x^2+1"));
    // - a composite modulus
    test("2*x+3", 5, 10, Some("2*x^4+2*x^3+2*x^2+2*x+7"));
    // - a modulus near the top of the word
    test(
        "250*x^2+249*x+248",
        4,
        251,
        Some("127*x^3+158*x^2+56*x+167"),
    );
}

#[test]
fn test_mod_inverse_truncated() {
    apply_fn_to_unsigneds!(test_mod_inverse_truncated_helper);
}

// Every algorithm, at lengths that reach Newton's iteration from the dispatcher, against the naive
// version, for a small, an odd, and the largest modulus.
fn mod_inverse_truncated_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(p_len, len) in &[(11, 11), (12, 40), (40, 40), (50, 120), (300, 300), (200, 700)] {
        for m in [T::from(7u8), T::MAX >> 1 | T::ONE, T::MAX] {
            let mut xs = mod_generated_coefficients(p_len, m, 1);
            xs[0] = T::ONE;
            let p = UnsignedPolynomial::<T>::from_coefficients_asc(xs);
            assert!(verify(&p, u64::exact_from(len), m).is_some());
        }
    }
}

#[test]
fn test_mod_inverse_truncated_algorithms() {
    apply_fn_to_unsigneds!(mod_inverse_truncated_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_inverse_truncated_fail_1() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .mod_inverse_truncated(3, 7);
}

#[test]
#[should_panic]
fn mod_inverse_truncated_fail_2() {
    // The modulus is zero.
    UnsignedPolynomial::<u8>::ZERO.mod_inverse_truncated(3, 0);
}

fn mod_inverse_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_8::<T>().test_properties(|(p, len, m)| {
        let Some(q) = verify(&p, len, m) else {
            return;
        };
        // The inverse of the inverse is the truncated polynomial.
        assert_eq!(verify(&q, len, m), Some(p.truncate(len)));
        // Inverting to fewer coefficients truncates the inverse.
        let half = len >> 1;
        assert_eq!(verify(&p, half, m), Some(q.truncate(half)));
        // The inverse of the square is the square of the inverse.
        assert_eq!(
            (&p).mod_square_truncated(len, m)
                .mod_inverse_truncated(len, m),
            Some(q.mod_square_truncated(len, m))
        );
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_14::<T>().test_properties(
        |(p, len, m)| {
            verify(&p, len, m);
        },
    );
}

#[test]
fn mod_inverse_truncated_properties() {
    apply_fn_to_unsigneds!(mod_inverse_truncated_properties_helper);
}
