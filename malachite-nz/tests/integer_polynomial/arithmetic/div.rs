// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CheckedDiv, DivRem};
use malachite_base::polynomial::CheckedDivRem;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_polynomial_pair_gen_var_4,
};

#[test]
fn test_div() {
    let test = |p, q, out: Option<&str>| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        let s = (&p).checked_div(&q);
        assert_eq!(
            s.as_ref().map(ToString::to_string),
            out.map(ToString::to_string)
        );
        if let Some(s) = s {
            assert!(s.is_valid());
            assert_eq!(&p / &q, s);
            let mut t = p.clone();
            t /= q;
            assert_eq!(t, s);
        }
    };
    test("x^3+2*x+1", "x+1", Some("x^2-x+3"));
    test("6*x^2+3*x+4", "3*x", Some("2*x+1"));
    test("x^3+2*x+1", "2*x+1", None);
    test("x+1", "0", None);
}

#[test]
#[should_panic]
fn div_fail() {
    let _ = IntegerPolynomial::from_str("x+1").unwrap() / IntegerPolynomial::from_str("0").unwrap();
}

#[test]
fn div_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        let s = (&p).checked_div(&q);
        assert_eq!(s, (&p).checked_div_rem(&q).map(|x| x.0));
        assert_eq!(p.clone().checked_div(q.clone()), s);
    });

    integer_polynomial_pair_gen_var_4().test_properties(|(p, q)| {
        let s = &p / &q;
        assert_eq!(p.clone() / q.clone(), s);
        assert_eq!(s, (&p).div_rem(&q).0);
    });
}
