// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivAssignRem, DivRem};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{CheckedDivRem, Polynomial};
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_pair_gen, rational_polynomial_pair_gen_var_3,
};
use malachite_q::test_util::rational_polynomial::arithmetic::div_rem::div_rem_naive;

#[test]
fn test_div_rem() {
    let test = |s, t, q_out, r_out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        let (quotient, remainder) = (&p).div_rem(&q);
        assert!(quotient.is_valid());
        assert!(remainder.is_valid());
        assert_eq!(quotient.to_string(), q_out);
        assert_eq!(remainder.to_string(), r_out);
        assert_eq!(
            (&p).div_rem(q.clone()),
            (quotient.clone(), remainder.clone())
        );
        assert_eq!(p.clone().div_rem(&q), (quotient.clone(), remainder.clone()));
        assert_eq!(
            p.clone().div_rem(q.clone()),
            (quotient.clone(), remainder.clone())
        );
        let mut s = p.clone();
        assert_eq!(s.div_assign_rem(&q), remainder);
        assert_eq!(s, quotient);
        let mut s = p.clone();
        assert_eq!(s.div_assign_rem(q.clone()), remainder);
        assert_eq!(s, quotient);
        assert_eq!(
            (&p).checked_div_rem(&q),
            Some((quotient.clone(), remainder.clone()))
        );
        assert_eq!(div_rem_naive(&p, &q), (quotient, remainder));
    };
    test("0", "x+1", "0", "0");
    test("x", "x^2+1", "0", "x");
    // Constant divisors
    test("3/2*x^2+x", "3", "1/2*x^2+1/3*x", "0");
    test("x^2+x", "-2/3", "-3/2*x^2-3/2*x", "0");
    // The leading coefficient of the divisor's numerator is 1 or -1, or a power of it is needed
    test("x^3-1", "x-1", "x^2+x+1", "0");
    test("x^2+1", "2*x+1", "1/2*x-1/4", "5/4");
    // - the pseudo-denominator is negative
    test("x+1", "-2*x+1", "-1/2", "3/2");
    test("1/2*x^2+1/3", "1/5*x", "5/2*x", "1/3");
    test("2*x^3+x+5", "4*x^2-1", "1/2*x", "3/2*x+5");
    test(
        "x^4+x+1",
        "3*x^2+2*x+1",
        "1/3*x^2-2/9*x+1/27",
        "31/27*x+26/27",
    );
    test(
        "18446744073709551616*x^2+1",
        "3*x+1",
        "18446744073709551616/3*x-18446744073709551616/9",
        "18446744073709551625/9",
    );
}

#[test]
fn test_checked_div_rem() {
    // - the divisor is 0
    assert_eq!(
        RationalPolynomial::from_str("x")
            .unwrap()
            .checked_div_rem(RationalPolynomial::ZERO),
        None
    );
}

#[test]
#[should_panic]
fn div_rem_fail() {
    let _ = RationalPolynomial::from_str("x")
        .unwrap()
        .div_rem(RationalPolynomial::ZERO);
}

#[test]
#[should_panic]
fn div_rem_ref_ref_fail() {
    let _ = (&RationalPolynomial::from_str("x").unwrap()).div_rem(&RationalPolynomial::ZERO);
}

#[test]
#[should_panic]
fn div_assign_rem_fail() {
    let mut p = RationalPolynomial::from_str("x").unwrap();
    let _ = p.div_assign_rem(RationalPolynomial::ZERO);
}

#[test]
fn div_rem_properties() {
    rational_polynomial_pair_gen_var_3().test_properties(|(p, q)| {
        let (quotient, remainder) = (&p).div_rem(&q);
        assert!(quotient.is_valid());
        assert!(remainder.is_valid());
        assert_eq!(
            (&p).div_rem(q.clone()),
            (quotient.clone(), remainder.clone())
        );
        assert_eq!(p.clone().div_rem(&q), (quotient.clone(), remainder.clone()));
        assert_eq!(
            p.clone().div_rem(q.clone()),
            (quotient.clone(), remainder.clone())
        );
        let mut s = p.clone();
        assert_eq!(s.div_assign_rem(&q), remainder);
        assert_eq!(s, quotient);
        assert_eq!(div_rem_naive(&p, &q), (quotient.clone(), remainder.clone()));
        assert_eq!(&quotient * &q + &remainder, p);
        assert!(remainder.len() < q.len());
        assert_eq!((-&p).div_rem(&q), (-&quotient, -&remainder));
        assert_eq!((&p).div_rem(-&q), (-quotient, remainder));
    });

    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        assert_eq!(
            (&p).checked_div_rem(&q).is_none(),
            q == RationalPolynomial::ZERO
        );
    });
}
