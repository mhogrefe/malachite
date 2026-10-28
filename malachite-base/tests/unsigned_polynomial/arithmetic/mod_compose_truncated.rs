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
    ModCompose, ModComposeTruncated, ModComposeTruncatedAssign, Polynomial,
};
use malachite_base::test_util::generators::{
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_5,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_8,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_compose_truncated::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the composition modulo m as power series of `p` and `q`, truncated to `len` coefficients,
// in every way of calling it, against the naive version, each algorithm, and the truncation of the
// full composition, and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> UnsignedPolynomial<T> {
    let r = p.mod_compose_truncated(q, len, m);
    assert!(r.is_valid());
    assert!(r.mod_is_reduced(&m));
    assert_eq!(p.clone().mod_compose_truncated(q.clone(), len, m), r);
    assert_eq!(p.clone().mod_compose_truncated(q, len, m), r);
    assert_eq!(p.mod_compose_truncated(q.clone(), len, m), r);
    let mut s = p.clone();
    s.mod_compose_truncated_assign(q, len, m);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_compose_truncated_assign(q.clone(), len, m);
    assert_eq!(s, r);
    assert_eq!(mod_compose_truncated_naive(p, q, len, m), r);
    assert_eq!(mod_compose_truncated_horner_wrapper(p, q, len, m), r);
    assert_eq!(mod_compose_truncated_brent_kung_wrapper(p, q, len, m), r);
    if p.len() > 2 {
        assert_eq!(
            mod_compose_truncated_divide_and_conquer_wrapper(p, q, len, m),
            r
        );
    }
    assert_eq!(p.mod_compose(q, m).truncate(len), r);
    r
}

fn test_mod_compose_truncated_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, len, m: u8, out| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(verify(&p, &q, len, T::from(m)).to_string(), out);
    };
    // - the length is 0
    test("3*x^2+2*x+1", "x", 0, 7, "0");
    // - the outer polynomial is 0
    test("0", "2*x", 5, 7, "0");
    // - the inner polynomial is 0: the constant coefficient
    test("3*x^2+2*x+1", "0", 5, 7, "1");
    // - a constant outer polynomial
    test("5", "3*x^2+2*x", 4, 7, "5");
    // - length 1: the constant coefficient
    test("3*x+5", "x", 1, 7, "5");
    // - modulo 1
    test("0", "0", 3, 1, "0");
    // - a monomial inner polynomial of degree 1
    test("x^3+3*x^2+x+2", "3*x", 4, 10, "7*x^3+7*x^2+3*x+2");
    // - a monomial inner polynomial of higher degree
    test("x^3+3*x^2+x+2", "2*x^2", 7, 10, "8*x^6+2*x^4+2*x^2+2");
    // - the inner polynomial's valuation truncates the outer polynomial
    test(
        "6*x^8+6*x^7+5*x^6+4*x^5+3*x^4+2*x^3+x^2+3*x+1",
        "x^3+x^2",
        7,
        7,
        "3*x^6+2*x^5+x^4+3*x^3+3*x^2+1",
    );
    // - Horner's rule
    test("x^3+3*x^2+x+2", "x^2+5*x", 4, 7, "x^3+6*x^2+5*x+2");
    // - a coefficient vanishes modulo m
    test("x^2+2*x+1", "x^2+2*x", 3, 6, "4*x+1");
    // - a modulus near the top of the word
    test(
        "250*x^3+249*x^2+248*x+247",
        "250*x^2+250*x",
        5,
        251,
        "x^4+248*x^3+x^2+3*x+247",
    );
    // - divide and conquer
    test(
        "9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "x^4+x^3+x^2+3*x",
        32,
        11,
        "6*x^31+5*x^30+3*x^29+9*x^28+3*x^27+x^25+2*x^24+7*x^23+5*x^22+8*x^21+3*x^20+9*x^18+x^17+10\
        *x^16+5*x^15+8*x^14+10*x^13+4*x^12+6*x^11+5*x^10+4*x^8+7*x^7+10*x^5+8*x^4+7*x^3+7*x^2+6*x+\
        1",
    );
    // - the Brent-Kung algorithm
    test(
        "17*x^16+16*x^15+15*x^14+14*x^13+13*x^12+12*x^11+11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*\
        x^4+4*x^3+3*x^2+2*x+1",
        "5*x^4+4*x^3+3*x^2+2*x",
        17,
        23,
        "20*x^16+17*x^14+9*x^13+18*x^12+8*x^11+7*x^10+13*x^9+16*x^8+2*x^7+x^6+16*x^5+10*x^4+7*x^3+\
        18*x^2+4*x+1",
    );
}

#[test]
fn test_mod_compose_truncated() {
    apply_fn_to_unsigneds!(test_mod_compose_truncated_helper);
}

// Every algorithm, at lengths that reach each of them, against the naive version, for a small, an
// odd, and the largest modulus.
fn mod_compose_truncated_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(len1, len2, n) in &[
        (8, 2, 8),
        (9, 3, 10),
        (16, 16, 16),
        (40, 3, 100),
        (50, 50, 50),
        (64, 9, 64),
        (100, 100, 100),
    ] {
        for m in [T::from(7u8), T::MAX >> 1 | T::ONE, T::MAX] {
            let p = UnsignedPolynomial::<T>::from_coefficients_asc(mod_generated_coefficients(
                len1, m, 1,
            ));
            let mut ys = mod_generated_coefficients(len2, m, 2);
            ys[0] = T::ZERO;
            let q = UnsignedPolynomial::from_coefficients_asc(ys);
            verify(&p, &q, u64::exact_from(n), m);
        }
    }
}

#[test]
fn test_mod_compose_truncated_algorithms() {
    apply_fn_to_unsigneds!(mod_compose_truncated_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_compose_truncated_fail_1() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .mod_compose_truncated(UnsignedPolynomial::from_str("x").unwrap(), 3, 7);
}

#[test]
#[should_panic]
fn mod_compose_truncated_fail_2() {
    // A coefficient of the inner polynomial is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_compose_truncated(UnsignedPolynomial::from_str("7*x").unwrap(), 3, 7);
}

#[test]
#[should_panic]
fn mod_compose_truncated_fail_3() {
    // The modulus is zero.
    UnsignedPolynomial::<u8>::ZERO.mod_compose_truncated(UnsignedPolynomial::ZERO, 3, 0);
}

#[test]
#[should_panic]
fn mod_compose_truncated_fail_4() {
    // The inner polynomial has a nonzero constant coefficient.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_compose_truncated(UnsignedPolynomial::from_str("x+1").unwrap(), 3, 7);
}

#[test]
#[should_panic]
fn mod_compose_truncated_fail_5() {
    // The inner polynomial has a nonzero constant coefficient, even when the length is 0.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_compose_truncated(UnsignedPolynomial::from_str("x+1").unwrap(), 0, 7);
}

#[test]
#[should_panic]
fn mod_compose_truncated_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    p.mod_compose_truncated_assign(UnsignedPolynomial::from_str("x+1").unwrap(), 3, 7);
}

fn mod_compose_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_5::<T>().test_properties(
        |(p, q, len, m)| {
            let r = verify(&p, &q, len, m);
            assert!(r.len() <= len);
            // Composition is associative: (p(q))(q) = p(q(q)).
            assert_eq!(
                (&r).mod_compose_truncated(&q, len, m),
                (&p).mod_compose_truncated((&q).mod_compose_truncated(&q, len, m), len, m)
            );
            // Only the first `len` coefficients of each polynomial matter.
            assert_eq!(
                p.truncate(len)
                    .mod_compose_truncated(q.truncate(len), len, m),
                r
            );
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_8::<T>().test_properties(|(p, len, m)| {
        let x = UnsignedPolynomial::<T>::x() % m;
        // Composing with x truncates.
        assert_eq!(verify(&p, &x, len, m), p.truncate(len));
        // Composing with 0 gives the constant coefficient.
        assert_eq!(
            verify(&p, &UnsignedPolynomial::ZERO, len, m),
            UnsignedPolynomial::from(p.coefficient(0)).truncate(len)
        );
        // Composing x with a polynomial truncates it.
        if p.coefficient(0) == T::ZERO {
            assert_eq!(verify(&x, &p, len, m), p.truncate(len));
        }
    });
}

#[test]
fn mod_compose_truncated_properties() {
    apply_fn_to_unsigneds!(mod_compose_truncated_properties_helper);
}
