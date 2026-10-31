// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{InverseTruncated, MulTruncated, Polynomial};
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::rational_polynomial::arithmetic::inverse_truncated::*;
use malachite_q::test_util::generators::{
    rational_polynomial_unsigned_pair_gen_var_1, rational_polynomial_unsigned_pair_gen_var_4,
};
use malachite_q::test_util::rational_polynomial::arithmetic::div_truncated::inverse_truncated_naive;

#[test]
fn test_inverse_truncated() {
    let test = |s, len, out: Option<&str>| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let r = (&p).inverse_truncated(len);
        assert!(r.as_ref().is_none_or(RationalPolynomial::is_valid));
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(p.clone().inverse_truncated(len), r);
        assert_eq!(inverse_truncated_naive(&p, len), r);
    };
    test("x+1", 0, Some("0"));
    test("0", 3, None);
    test("x", 3, None);
    test("1/3", 3, Some("3"));
    test("2*x+1", 4, Some("-8*x^3+4*x^2-2*x+1"));
    test("x^2+x+1", 6, Some("-x^4+x^3-x+1"));
    test("x+2", 3, Some("1/8*x^2-1/4*x+1/2"));
    test("1/2*x+1/3", 4, Some("-81/8*x^3+27/4*x^2-9/2*x+3"));
    test("-3/2*x^2+5", 5, Some("9/500*x^4+3/50*x^2+1/5"));
}

// The basecase and Newton iteration, on series of chosen lengths with rational coefficients.
#[test]
fn test_inverse_truncated_algorithms() {
    let test = |len: i64, n: u64| {
        let p = RationalPolynomial::from_coefficients_asc(
            (0..len)
                .map(|i| Rational::from_signeds(if i == 0 { 3 } else { 2 * i - 7 }, i + 2))
                .collect(),
        );
        let expected = inverse_truncated_naive(&p, n).unwrap();
        assert_eq!(inverse_truncated_basecase(&p, n), expected);
        assert_eq!(inverse_truncated_newton(&p, n), expected);
        for cutoff in [1, 4, 24, 100] {
            assert_eq!(
                inverse_truncated_newton_with_cutoff(&p, n, cutoff),
                expected
            );
        }
    };
    test(3, 10);
    test(30, 30);
    test(30, 60);
    test(100, 100);
}

#[test]
fn inverse_truncated_properties() {
    rational_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, len)| {
        let r = (&p).inverse_truncated(len);
        assert!(r.as_ref().is_none_or(RationalPolynomial::is_valid));
        assert_eq!(p.clone().inverse_truncated(len), r);
        assert_eq!(inverse_truncated_naive(&p, len), r);
        assert_eq!(r.is_some(), len == 0 || p.coefficient(0) != 0u32);
    });

    rational_polynomial_unsigned_pair_gen_var_4().test_properties(|(p, len)| {
        let r = (&p).inverse_truncated(len).unwrap();
        assert!(r.is_valid());
        assert!(r.len() <= len);
        let one = if len == 0 {
            RationalPolynomial::ZERO
        } else {
            RationalPolynomial::one()
        };
        assert_eq!((&p).mul_truncated(&r, len), one);
        assert_eq!((&r).inverse_truncated(len).unwrap(), p.truncate(len));
    });
}
