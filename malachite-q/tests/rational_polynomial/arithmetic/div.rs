// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CheckedDiv, DivRem};
use malachite_base::num::basic::traits::Zero;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_pair_gen, rational_polynomial_pair_gen_var_3,
};

#[test]
fn test_div() {
    let test = |s, t, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        let quotient = &p / &q;
        assert!(quotient.is_valid());
        assert_eq!(quotient.to_string(), out);
        assert_eq!(&p / q.clone(), quotient);
        assert_eq!(p.clone() / &q, quotient);
        assert_eq!(p.clone() / q.clone(), quotient);
        let mut s = p.clone();
        s /= &q;
        assert_eq!(s, quotient);
        let mut s = p.clone();
        s /= q.clone();
        assert_eq!(s, quotient);
        assert_eq!((&p).checked_div(&q), Some(quotient));
    };
    test("0", "x+1", "0");
    test("x", "x^2+1", "0");
    test("3/2*x^2+x", "3", "1/2*x^2+1/3*x");
    test("x^2+1", "2*x+1", "1/2*x-1/4");
    // - the pseudo-denominator is negative
    test("x+1", "-2*x+1", "-1/2");
    test("x^4+x+1", "3*x^2+2*x+1", "1/3*x^2-2/9*x+1/27");
}

#[test]
fn test_checked_div() {
    // - the divisor is 0
    assert_eq!(
        RationalPolynomial::from_str("x")
            .unwrap()
            .checked_div(RationalPolynomial::ZERO),
        None
    );
}

#[test]
#[should_panic]
fn div_fail() {
    let _ = RationalPolynomial::from_str("x").unwrap() / RationalPolynomial::ZERO;
}

#[test]
fn div_properties() {
    rational_polynomial_pair_gen_var_3().test_properties(|(p, q)| {
        let quotient = &p / &q;
        assert!(quotient.is_valid());
        assert_eq!(&p / q.clone(), quotient);
        assert_eq!(p.clone() / &q, quotient);
        assert_eq!(p.clone() / q.clone(), quotient);
        let mut s = p.clone();
        s /= &q;
        assert_eq!(s, quotient);
        assert_eq!((&p).div_rem(&q).0, quotient);
    });

    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        assert_eq!(
            (&p).checked_div(&q).is_none(),
            q == RationalPolynomial::ZERO
        );
    });
}
