// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    CheckedDivTruncated, DivTruncated, DivTruncatedAssign, MulTruncated, Polynomial,
};
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::rational_polynomial::arithmetic::div_truncated::*;
use malachite_q::test_util::generators::{
    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_1,
    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_4,
};
use malachite_q::test_util::rational_polynomial::arithmetic::div_truncated::div_truncated_naive;

#[test]
fn test_div_truncated() {
    let test = |s, t, len, out: Option<&str>| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        let r = (&p).checked_div_truncated(&q, len);
        assert!(r.as_ref().is_none_or(RationalPolynomial::is_valid));
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!((&p).checked_div_truncated(q.clone(), len), r);
        assert_eq!(p.clone().checked_div_truncated(&q, len), r);
        assert_eq!(p.clone().checked_div_truncated(q.clone(), len), r);
        assert_eq!(div_truncated_naive(&p, &q, len), r);
        if let Some(r) = r {
            assert_eq!((&p).div_truncated(&q, len), r);
            assert_eq!((&p).div_truncated(q.clone(), len), r);
            assert_eq!(p.clone().div_truncated(&q, len), r);
            assert_eq!(p.clone().div_truncated(q.clone(), len), r);
            let mut s = p.clone();
            s.div_truncated_assign(&q, len);
            assert_eq!(s, r);
            let mut s = p;
            s.div_truncated_assign(q, len);
            assert_eq!(s, r);
        }
    };
    test("x", "0", 0, Some("0"));
    test("x", "0", 3, None);
    test("x+1", "x", 3, None);
    test("0", "x+3", 3, Some("0"));
    // - the series is short, so it is divided by reversed polynomial division
    test("1", "x+2", 3, Some("1/8*x^2-1/4*x+1/2"));
    // - the series is short, but the divisor is an integer polynomial with constant coefficient 1,
    //   so it is inverted
    test("x^2+3*x+2", "x+1", 3, Some("x+2"));
    test("x^2+3*x+2", "x+1", 5, Some("x+2"));
    test("1/2*x+1", "2*x+3", 3, Some("1/27*x^2-1/18*x+1/3"));
    // - the series is long, so the divisor is inverted
    test(
        "x^3+1/2",
        "1/3*x^2-x+2",
        5,
        Some("139/576*x^4+47/96*x^3+1/48*x^2+1/8*x+1/4"),
    );
}

#[test]
#[should_panic]
fn div_truncated_fail() {
    let _ = RationalPolynomial::from_str("x+1")
        .unwrap()
        .div_truncated(RationalPolynomial::from_str("x").unwrap(), 3);
}

#[test]
#[should_panic]
fn div_truncated_assign_fail() {
    let mut p = RationalPolynomial::from_str("x+1").unwrap();
    p.div_truncated_assign(RationalPolynomial::ZERO, 3);
}

#[test]
fn div_truncated_properties() {
    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, len)| {
            let r = (&p).checked_div_truncated(&q, len);
            assert!(r.as_ref().is_none_or(RationalPolynomial::is_valid));
            assert_eq!(div_truncated_naive(&p, &q, len), r);
            assert_eq!(r.is_some(), len == 0 || q.coefficient(0) != 0u32);
        },
    );

    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_4().test_properties(
        |(p, q, len)| {
            let r = (&p).div_truncated(&q, len);
            assert!(r.is_valid());
            assert_eq!((&p).div_truncated(q.clone(), len), r);
            assert_eq!(p.clone().div_truncated(&q, len), r);
            assert_eq!(p.clone().div_truncated(q.clone(), len), r);
            let mut s = p.clone();
            s.div_truncated_assign(&q, len);
            assert_eq!(s, r);
            assert_eq!((&q).mul_truncated(&r, len), p.truncate(len));
            if len != 0 && p != RationalPolynomial::ZERO {
                assert_eq!(div_truncated_by_inverse(&p, &q, len), r);
                assert_eq!(div_truncated_by_division(&p, &q, len), r);
            }
            assert_eq!((-&p).div_truncated(&q, len), -&r);
        },
    );
}
