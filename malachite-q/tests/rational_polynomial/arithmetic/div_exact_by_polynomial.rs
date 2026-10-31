// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, DivRem};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::CheckedDivExact;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_pair_gen, rational_polynomial_pair_gen_var_4,
};

#[test]
fn test_checked_div_exact() {
    let test = |s, t, out: Option<&str>| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        let r = (&p).checked_div_exact(&q);
        assert!(r.as_ref().is_none_or(RationalPolynomial::is_valid));
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!((&p).checked_div_exact(q.clone()), r);
        assert_eq!(p.clone().checked_div_exact(&q), r);
        assert_eq!(p.clone().checked_div_exact(q.clone()), r);
        if let Some(r) = r {
            assert_eq!((&p).div_exact(&q), r);
            assert_eq!((&p).div_exact(q.clone()), r);
            assert_eq!(p.clone().div_exact(&q), r);
            assert_eq!(p.clone().div_exact(q.clone()), r);
            let mut s = p.clone();
            s.div_exact_assign(&q);
            assert_eq!(s, r);
            let mut s = p;
            s.div_exact_assign(q);
            assert_eq!(s, r);
        }
    };
    test("0", "x", Some("0"));
    test("x", "0", None);
    test("x", "3/5", Some("5/3*x"));
    test("x^2-1/4", "x-1/2", Some("x+1/2"));
    test("2/3*x^2+2/3*x", "4*x", Some("1/6*x+1/6"));
    test("x^3-1", "x-1", Some("x^2+x+1"));
    test("x^2+1", "x+1", None);
}

#[test]
#[should_panic]
fn div_exact_fail() {
    let _ = RationalPolynomial::from_str("x")
        .unwrap()
        .div_exact(RationalPolynomial::ZERO);
}

#[test]
fn div_exact_properties() {
    rational_polynomial_pair_gen_var_4().test_properties(|(p, q)| {
        let r = (&p).div_exact(&q);
        assert!(r.is_valid());
        assert_eq!((&p).div_exact(q.clone()), r);
        assert_eq!(p.clone().div_exact(&q), r);
        assert_eq!(p.clone().div_exact(q.clone()), r);
        let mut s = p.clone();
        s.div_exact_assign(&q);
        assert_eq!(s, r);
        assert_eq!((&p).checked_div_exact(&q), Some(r.clone()));
        assert_eq!(&r * &q, p);
        assert_eq!((&p).div_rem(&q).0, r);
    });

    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        let r = (&p).checked_div_exact(&q);
        let (quotient, remainder) = if q == RationalPolynomial::ZERO {
            (
                RationalPolynomial::ZERO,
                RationalPolynomial::from_str("1").unwrap(),
            )
        } else {
            (&p).div_rem(&q)
        };
        assert_eq!(
            r,
            if remainder == RationalPolynomial::ZERO {
                Some(quotient)
            } else {
                None
            }
        );
    });
}
