// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2IsReduced};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    ModInverseTruncated, ModPowerOf2InverseTruncated, ModPowerOf2MulTruncated,
    ModPowerOf2SquareTruncated, Polynomial,
};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_7,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_15,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_inverse_truncated::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the inverse modulo 2^k as a power series of `p`, truncated to `len` coefficients, in every
// way of calling it, against the naive version, each algorithm, and the inverse modulo 2^k as a
// general modulus, checks that it exists exactly when the constant coefficient is odd and that its
// product with `p` is 1, and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    let r = p.mod_power_of_2_inverse_truncated(len, pow);
    assert_eq!(p.clone().mod_power_of_2_inverse_truncated(len, pow), r);
    assert_eq!(mod_power_of_2_inverse_truncated_naive(p, len, pow), r);
    assert_eq!(
        mod_power_of_2_inverse_truncated_basecase_wrapper(p, len, pow),
        r
    );
    assert_eq!(
        mod_power_of_2_inverse_truncated_newton_wrapper(p, len, pow),
        r
    );
    if pow < T::WIDTH {
        assert_eq!(p.mod_inverse_truncated(len, T::power_of_2(pow)), r);
    }
    let invertible = len == 0 || pow == 0 || p.coefficient(0).odd();
    assert_eq!(r.is_some(), invertible);
    if let Some(q) = &r {
        assert!(q.is_valid());
        assert!(q.mod_power_of_2_is_reduced(pow));
        assert!(q.len() <= len);
        assert_eq!(
            p.mod_power_of_2_mul_truncated(q, len, pow),
            UnsignedPolynomial::one().truncate(len).mod_power_of_2(pow)
        );
    }
    r
}

fn test_mod_power_of_2_inverse_truncated_helper<T: PrimitiveUnsigned>() {
    let test = |p, len, pow, out: Option<&str>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        assert_eq!(
            verify(&p, len, pow).map(|q| q.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - the length is 0
    test("3*x+5", 0, 3, Some("0"));
    // - modulo 1
    test("0", 5, 0, Some("0"));
    // - the polynomial is 0
    test("0", 3, 3, None);
    // - the constant coefficient is even
    test("2*x+4", 3, 3, None);
    // - the constant coefficient is 0
    test("3*x", 3, 3, None);
    // - length 1: only the constant coefficient
    test("3*x+5", 1, 3, Some("5"));
    // - a constant polynomial
    test("3", 5, 3, Some("3"));
    // - the constant coefficient is 1
    test("x+1", 5, 8, Some("x^4+255*x^3+x^2+255*x+1"));
    // - a binomial
    test("9*x+1", 7, 4, Some("x^6+7*x^5+x^4+7*x^3+x^2+7*x+1"));
    // - the constant coefficient is not 1
    test("3*x^2+x+5", 6, 3, Some("x^4+5*x^3+2*x^2+7*x+5"));
    // - only the first coefficients are read
    test("5*x^4+x+3", 2, 3, Some("7*x+3"));
    // - the inverse has zeros at the end
    test("x^2+1", 4, 8, Some("255*x^2+1"));
    // - the full width of a byte
    test("255*x^2+254*x+253", 4, 8, Some("60*x^3+237*x^2+114*x+85"));
}

#[test]
fn test_mod_power_of_2_inverse_truncated() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_inverse_truncated_helper);
}

// Every algorithm, at lengths that reach Newton's iteration from the dispatcher and
// number-theoretic transforms within it, against the naive version, for several exponents.
fn mod_power_of_2_inverse_truncated_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(p_len, len) in &[(11, 11), (12, 40), (40, 40), (50, 120), (300, 300), (600, 1300)] {
        for pow in [1, 3, T::WIDTH >> 1, T::WIDTH] {
            let mut xs = mod_power_of_2_generated_coefficients(p_len, pow, 1);
            xs[0] = T::ONE;
            let p = UnsignedPolynomial::<T>::from_coefficients_asc(xs);
            assert!(verify(&p, u64::exact_from(len), pow).is_some());
        }
    }
}

#[test]
fn test_mod_power_of_2_inverse_truncated_algorithms() {
    apply_fn_to_unsigneds!(mod_power_of_2_inverse_truncated_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_inverse_truncated_fail_1() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_inverse_truncated(3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_inverse_truncated_fail_2() {
    // The exponent is greater than the width of the type.
    UnsignedPolynomial::<u8>::ZERO.mod_power_of_2_inverse_truncated(3, 9);
}

fn mod_power_of_2_inverse_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_7::<T>().test_properties(
        |(p, len, pow)| {
            let Some(q) = verify(&p, len, pow) else {
                return;
            };
            // The inverse of the inverse is the truncated polynomial.
            assert_eq!(verify(&q, len, pow), Some(p.truncate(len)));
            // Inverting to fewer coefficients truncates the inverse.
            let half = len >> 1;
            assert_eq!(verify(&p, half, pow), Some(q.truncate(half)));
            // The inverse of the square is the square of the inverse.
            assert_eq!(
                (&p).mod_power_of_2_square_truncated(len, pow)
                    .mod_power_of_2_inverse_truncated(len, pow),
                Some(q.mod_power_of_2_square_truncated(len, pow))
            );
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_15::<T>().test_properties(
        |(p, len, pow)| {
            verify(&p, len, pow);
        },
    );
}

#[test]
fn mod_power_of_2_inverse_truncated_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_inverse_truncated_properties_helper);
}
