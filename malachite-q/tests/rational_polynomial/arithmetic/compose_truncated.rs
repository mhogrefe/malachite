// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Compose, ComposeTruncated, ComposeTruncatedAssign, Polynomial};
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2,
    rational_polynomial_unsigned_pair_gen_var_1,
};
use malachite_q::test_util::rational_polynomial::arithmetic::compose_truncated::*;
use std::panic::catch_unwind;

// Checks the composition as power series of `p` and `q`, truncated to `len` coefficients, in every
// way of calling it, against the naive version and the truncation of the full composition, and
// returns it.
fn verify(p: &RationalPolynomial, q: &RationalPolynomial, len: u64) -> RationalPolynomial {
    let r = p.compose_truncated(q, len);
    assert!(r.is_valid());
    assert_eq!(p.clone().compose_truncated(q.clone(), len), r);
    assert_eq!(p.clone().compose_truncated(q, len), r);
    assert_eq!(p.compose_truncated(q.clone(), len), r);
    let mut s = p.clone();
    s.compose_truncated_assign(q, len);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.compose_truncated_assign(q.clone(), len);
    assert_eq!(s, r);
    assert_eq!(rational_polynomial_compose_truncated_naive(p, q, len), r);
    assert_eq!(p.compose(q).truncate(len), r);
    r
}

#[test]
fn test_compose_truncated() {
    let test = |p, q, len, out| {
        let p = RationalPolynomial::from_str(p).unwrap();
        let q = RationalPolynomial::from_str(q).unwrap();
        assert_eq!(verify(&p, &q, len).to_string(), out);
    };
    // - the length is 0
    test("1/2*x^2+x+1/3", "x^2+1/2*x", 0, "0");
    // - the outer polynomial is 0
    test("0", "1/2*x", 5, "0");
    // - the inner polynomial is 0: the constant coefficient
    test("1/2*x^2+x+1/3", "0", 5, "1/3");
    // - a constant outer polynomial
    test("7/3", "1/3*x+x^2", 5, "7/3");
    // - length 1: the constant coefficient, reduced
    test("1/2*x+1/3", "x", 1, "1/3");
    // - an inner polynomial with denominator 1
    test("1/2*x^2+x+1/3", "x^2+x", 3, "3/2*x^2+x+1/3");
    // - an inner polynomial with a denominator
    test("1/2*x^2+x+1/3", "x^2+1/2*x", 3, "9/8*x^2+1/2*x+1/3");
    // - a monomial inner polynomial with a denominator
    test(
        "-64*x^13-32/5*x^12-29/15*x^11-8*x^10-18/13*x^9+40/9*x^8-61/7*x^7-19/4*x^6-1/13*x^5+31/16*\
        x^4-6*x^3-1/2*x^2-19/5*x+1/12",
        "2/3*x^2",
        11,
        "-32/3159*x^10+31/81*x^8-16/9*x^6-2/9*x^4-38/15*x^2+1/12",
    );
    // - the inner polynomial's valuation truncates the outer polynomial
    test(
        "7/8*x^13+5/4*x^12+22/3*x^11+13*x^10-48/11*x^9-3*x^8+7*x^7-59/12*x^6+17/5*x^5-18/7*x^4-2*x\
        ^3+10/11*x^2-11/3*x-11/7",
        "1/2*x^4+x^5",
        12,
        "10/11*x^10+10/11*x^9+5/22*x^8-11/3*x^5-11/6*x^4-11/7",
    );
    // - many outer coefficients: the Brent-Kung algorithm
    test(
        "-37/12*x^13+2*x^11-2/15*x^10-7/2*x^9-53/6*x^8-62/15*x^7+29/13*x^6-30/7*x^5+27/13*x^4+x^3-\
        19/2*x^2+59/16*x+8/3",
        "-11/3*x^8+x^7+1/2*x^6+1/2*x^5-7/6*x^4+8/3*x^3-1/2*x^2+11/2*x",
        9,
        "-1090742331629/161280*x^8-26936233729/37440*x^7+3934051633/52416*x^6-63671485/2912*x^5+65\
        3035/416*x^4+5483/24*x^3-9255/32*x^2+649/32*x+8/3",
    );
}

#[test]
fn compose_truncated_fail() {
    // - the inner polynomial has a nonzero constant coefficient
    let p = RationalPolynomial::from_str("1/2*x+1").unwrap();
    let q = RationalPolynomial::from_str("x+1/3").unwrap();
    assert_panic!((&p).compose_truncated(&q, 3));
    let q = RationalPolynomial::from_str("x+1/3").unwrap();
    assert_panic!(p.clone().compose_truncated(q, 3));
    // - even when the length is 0
    let q = RationalPolynomial::from_str("x+1/3").unwrap();
    assert_panic!((&p).compose_truncated(&q, 0));
}

#[test]
#[should_panic]
fn compose_truncated_assign_fail() {
    let mut p = RationalPolynomial::from_str("1/2*x+1").unwrap();
    p.compose_truncated_assign(&RationalPolynomial::from_str("x+1/3").unwrap(), 3);
}

#[test]
fn compose_truncated_properties() {
    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2().test_properties(
        |(p, q, len)| {
            let r = verify(&p, &q, len);
            assert!(r.len() <= len);
            // Composition is associative: (p(q))(q) = p(q(q)).
            assert_eq!(
                (&r).compose_truncated(&q, len),
                (&p).compose_truncated((&q).compose_truncated(&q, len), len)
            );
            // Only the first `len` coefficients of each polynomial matter.
            assert_eq!(p.truncate(len).compose_truncated(q.truncate(len), len), r);
        },
    );

    rational_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, len)| {
        // Composing with x truncates.
        assert_eq!(verify(&p, &RationalPolynomial::x(), len), p.truncate(len));
        // Composing with 0 gives the constant coefficient.
        assert_eq!(
            verify(&p, &RationalPolynomial::ZERO, len),
            RationalPolynomial::from(p.coefficient(0)).truncate(len)
        );
        // Composing x with a polynomial truncates it.
        if p != RationalPolynomial::ZERO && p.coefficient(0) == 0u32 {
            assert_eq!(verify(&RationalPolynomial::x(), &p, len), p.truncate(len));
        }
    });

    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_2().test_properties(
        |(p, q, len)| {
            // For integer polynomials, the composition is that of the IntegerPolynomials.
            let p = p.into_numerator_and_denominator().0;
            let q = q.into_numerator_and_denominator().0;
            assert_eq!(
                RationalPolynomial::from(&p).compose_truncated(RationalPolynomial::from(&q), len),
                RationalPolynomial::from(p.compose_truncated(q, len))
            );
        },
    );
}
