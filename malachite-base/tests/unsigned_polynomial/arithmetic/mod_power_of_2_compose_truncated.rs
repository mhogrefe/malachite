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
    ModPowerOf2Compose, ModPowerOf2ComposeTruncated, ModPowerOf2ComposeTruncatedAssign, Polynomial,
};
use malachite_base::test_util::generators::{
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_3,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_7,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_compose_truncated::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the composition modulo $2^k$ as power series of `p` and `q`, truncated to `len`
// coefficients, in every way of calling it, against the naive version, each algorithm, and the
// truncation of the full composition, and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> UnsignedPolynomial<T> {
    let r = p.mod_power_of_2_compose_truncated(q, len, pow);
    assert!(r.is_valid());
    assert!(r.mod_power_of_2_is_reduced(pow));
    assert_eq!(
        p.clone()
            .mod_power_of_2_compose_truncated(q.clone(), len, pow),
        r
    );
    assert_eq!(p.clone().mod_power_of_2_compose_truncated(q, len, pow), r);
    assert_eq!(p.mod_power_of_2_compose_truncated(q.clone(), len, pow), r);
    let mut s = p.clone();
    s.mod_power_of_2_compose_truncated_assign(q, len, pow);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_power_of_2_compose_truncated_assign(q.clone(), len, pow);
    assert_eq!(s, r);
    assert_eq!(mod_power_of_2_compose_truncated_naive(p, q, len, pow), r);
    assert_eq!(
        mod_power_of_2_compose_truncated_horner_wrapper(p, q, len, pow),
        r
    );
    assert_eq!(
        mod_power_of_2_compose_truncated_brent_kung_wrapper(p, q, len, pow),
        r
    );
    if p.len() > 2 {
        assert_eq!(
            mod_power_of_2_compose_truncated_divide_and_conquer_wrapper(p, q, len, pow),
            r
        );
    }
    assert_eq!(p.mod_power_of_2_compose(q, pow).truncate(len), r);
    r
}

fn test_mod_power_of_2_compose_truncated_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, len, pow, out| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(verify(&p, &q, len, pow).to_string(), out);
    };
    // - the length is 0
    test("3*x^2+2*x+1", "x", 0, 3, "0");
    // - the outer polynomial is 0
    test("0", "2*x", 5, 3, "0");
    // - the inner polynomial is 0: the constant coefficient
    test("3*x^2+2*x+1", "0", 5, 3, "1");
    // - a constant outer polynomial
    test("5", "3*x^2+2*x", 4, 3, "5");
    // - length 1: the constant coefficient
    test("3*x+5", "x", 1, 3, "5");
    // - a monomial inner polynomial of degree 1
    test("x^3+3*x^2+x+2", "3*x", 4, 4, "11*x^3+11*x^2+3*x+2");
    // - a monomial inner polynomial of higher degree
    test("x^3+3*x^2+x+2", "2*x^2", 7, 4, "8*x^6+12*x^4+2*x^2+2");
    // - the inner polynomial's valuation truncates the outer polynomial
    test(
        "7*x^8+6*x^7+5*x^6+4*x^5+3*x^4+2*x^3+x^2+3*x+1",
        "x^3+x^2",
        7,
        4,
        "3*x^6+2*x^5+x^4+3*x^3+3*x^2+1",
    );
    // - Horner's rule
    test("x^3+3*x^2+x+2", "x^2+5*x", 4, 4, "11*x^3+12*x^2+5*x+2");
    // - a coefficient vanishes modulo 2^k
    test("x^2+2*x+1", "x^2+2*x", 3, 2, "2*x^2+1");
    // - the coefficients wrap around modulo 2^8
    test(
        "255*x^3+254*x^2+253*x+252",
        "255*x^2+255*x",
        5,
        8,
        "x^4+253*x^3+x^2+3*x+252",
    );
    // - divide and conquer
    test(
        "9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2+3*x",
        10,
        5,
        "14*x^9+27*x^8+6*x^7+31*x^6+18*x^5+4*x^4+30*x^3+29*x^2+6*x+1",
    );
    // - the Brent-Kung algorithm
    test(
        "12*x^11+11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "6*x^5+5*x^4+4*x^3+3*x^2+2*x",
        12,
        6,
        "4*x^11+10*x^10+56*x^9+28*x^8+60*x^7+6*x^6+8*x^5+53*x^4+12*x^3+18*x^2+4*x+1",
    );
}

#[test]
fn test_mod_power_of_2_compose_truncated() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_compose_truncated_helper);
}

// Every algorithm, at lengths that reach each of them, against the naive version.
fn mod_power_of_2_compose_truncated_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(len1, len2, n) in &[
        (8, 2, 8),
        (9, 3, 10),
        (16, 16, 16),
        (40, 3, 100),
        (50, 50, 50),
        (64, 9, 64),
        (100, 100, 100),
    ] {
        for pow in [T::WIDTH, T::WIDTH - 1] {
            let p = UnsignedPolynomial::<T>::from_coefficients_asc(
                mod_power_of_2_generated_coefficients(len1, pow, 1),
            );
            let mut ys = mod_power_of_2_generated_coefficients(len2, pow, 2);
            ys[0] = T::ZERO;
            let q = UnsignedPolynomial::from_coefficients_asc(ys);
            verify(&p, &q, u64::exact_from(n), pow);
        }
    }
}

#[test]
fn test_mod_power_of_2_compose_truncated_algorithms() {
    apply_fn_to_unsigneds!(mod_power_of_2_compose_truncated_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_truncated_fail_1() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_compose_truncated(UnsignedPolynomial::from_str("x").unwrap(), 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_truncated_fail_2() {
    // A coefficient of the inner polynomial is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_compose_truncated(UnsignedPolynomial::from_str("8*x").unwrap(), 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_truncated_fail_3() {
    // The modulus is too large for the coefficient type.
    UnsignedPolynomial::<u8>::ZERO.mod_power_of_2_compose_truncated(UnsignedPolynomial::ZERO, 3, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_truncated_fail_4() {
    // The inner polynomial has a nonzero constant coefficient.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_compose_truncated(UnsignedPolynomial::from_str("x+1").unwrap(), 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_truncated_fail_5() {
    // The inner polynomial has a nonzero constant coefficient, even when the length is 0.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_compose_truncated(UnsignedPolynomial::from_str("x+1").unwrap(), 0, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_truncated_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    p.mod_power_of_2_compose_truncated_assign(UnsignedPolynomial::from_str("x+1").unwrap(), 3, 3);
}

fn mod_power_of_2_compose_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_3::<T>().test_properties(
        |(p, q, len, pow)| {
            let r = verify(&p, &q, len, pow);
            assert!(r.len() <= len);
            // Composition is associative: (p(q))(q) = p(q(q)).
            assert_eq!(
                (&r).mod_power_of_2_compose_truncated(&q, len, pow),
                (&p).mod_power_of_2_compose_truncated(
                    (&q).mod_power_of_2_compose_truncated(&q, len, pow),
                    len,
                    pow
                )
            );
            // Only the first `len` coefficients of each polynomial matter.
            assert_eq!(
                p.truncate(len)
                    .mod_power_of_2_compose_truncated(q.truncate(len), len, pow),
                r
            );
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_7::<T>().test_properties(
        |(p, len, pow)| {
            let x = UnsignedPolynomial::<T>::x().mod_power_of_2(pow);
            // Composing with x truncates.
            assert_eq!(verify(&p, &x, len, pow), p.truncate(len));
            // Composing with 0 gives the constant coefficient.
            assert_eq!(
                verify(&p, &UnsignedPolynomial::ZERO, len, pow),
                UnsignedPolynomial::from(p.coefficient(0)).truncate(len)
            );
            // Composing x with a polynomial truncates it.
            if p.coefficient(0) == T::ZERO {
                assert_eq!(verify(&x, &p, len, pow), p.truncate(len));
            }
        },
    );
}

#[test]
fn mod_power_of_2_compose_truncated_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_compose_truncated_properties_helper);
}
