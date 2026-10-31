// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivisibleBy, Pow};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::factorization::traits::{RemovePower, RemovePowerAssign};
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_pair_gen_var_5;
use malachite_q::test_util::rational_polynomial::arithmetic::div_rem::remove_power_naive;

#[test]
fn test_remove_power() {
    let test = |s, t, out, e_out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        let (r, e) = (&p).remove_power(&q);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(e, e_out);
        assert_eq!((&p).remove_power(q.clone()), (r.clone(), e));
        assert_eq!(p.clone().remove_power(&q), (r.clone(), e));
        assert_eq!(p.clone().remove_power(q.clone()), (r.clone(), e));
        let mut s = p.clone();
        assert_eq!(s.remove_power_assign(&q), e);
        assert_eq!(s, r);
        let mut s = p.clone();
        assert_eq!(s.remove_power_assign(q.clone()), e);
        assert_eq!(s, r);
        assert_eq!(remove_power_naive(&p, &q), (r, e));
    };
    test("0", "x+1", "0", 0);
    test("x^2+1", "x^3+1", "x^2+1", 0);
    test("x^2+1", "x+1", "x^2+1", 0);
    test("x^3-3*x^2+3*x-1", "2*x-2", "1/8", 3);
    test("1/4*x^4-1/2*x^2+1/4", "x^2-1", "1/4", 2);
    test("x^2-2*x+1", "1/3*x-1/3", "9", 2);
}

#[test]
#[should_panic]
fn remove_power_fail_1() {
    let _ = RationalPolynomial::from_str("x")
        .unwrap()
        .remove_power(RationalPolynomial::ZERO);
}

#[test]
#[should_panic]
fn remove_power_fail_2() {
    let _ = RationalPolynomial::from_str("x")
        .unwrap()
        .remove_power(RationalPolynomial::from_str("2/3").unwrap());
}

#[test]
fn remove_power_properties() {
    rational_polynomial_pair_gen_var_5().test_properties(|(p, q)| {
        let (r, e) = (&p).remove_power(&q);
        assert!(r.is_valid());
        assert_eq!((&p).remove_power(q.clone()), (r.clone(), e));
        assert_eq!(p.clone().remove_power(&q), (r.clone(), e));
        let mut s = p.clone();
        assert_eq!(s.remove_power_assign(&q), e);
        assert_eq!(s, r);
        assert_eq!(remove_power_naive(&p, &q), (r.clone(), e));
        assert_eq!(&r * (&q).pow(e), p);
        if p != RationalPolynomial::ZERO {
            assert!(e >= 1);
            assert!(!r.divisible_by(&q));
        }
    });
}
