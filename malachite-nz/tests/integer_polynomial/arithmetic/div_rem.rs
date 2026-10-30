// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CheckedDiv, DivAssignRem, DivRem};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::*;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::pseudo_div_rem::*;

// Checks the quotient and remainder of `p` by `q` in every way of calling them, against the naive
// version, the basecase, the quotient and remainder alone, and pseudo-division; checks that $p = qs
// + r$ with $\deg r < \deg q$; and returns them.
#[allow(clippy::type_complexity)]
fn verify(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> Option<(IntegerPolynomial, IntegerPolynomial)> {
    let x = p.checked_div_rem(q);
    assert_eq!(p.clone().checked_div_rem(q.clone()), x);
    assert_eq!(p.clone().checked_div_rem(q), x);
    assert_eq!(p.checked_div_rem(q.clone()), x);
    assert_eq!(div_rem_naive(p, q), x);
    if *q != IntegerPolynomial::ZERO {
        assert_eq!(div_rem_basecase_wrapper(p, q), x);
        assert_eq!(x.is_some(), p.pseudo_div_rem(q).2 == 0);
    }
    assert_eq!(p.checked_div(q), x.as_ref().map(|(s, _)| s.clone()));
    assert_eq!(
        p.clone().checked_div(q.clone()),
        x.as_ref().map(|(s, _)| s.clone())
    );
    assert_eq!(p.checked_rem(q), x.as_ref().map(|(_, r)| r.clone()));
    assert_eq!(
        p.clone().checked_rem(q.clone()),
        x.as_ref().map(|(_, r)| r.clone())
    );
    if let Some((s, r)) = &x {
        assert!(s.is_valid());
        assert!(r.is_valid());
        assert!(r.len() < q.len());
        assert_eq!(q * s + r, *p);
        assert_eq!(p.div_rem(q), (s.clone(), r.clone()));
        assert_eq!(p.clone().div_rem(q.clone()), (s.clone(), r.clone()));
        assert_eq!(p.clone().div_rem(q), (s.clone(), r.clone()));
        assert_eq!(p.div_rem(q.clone()), (s.clone(), r.clone()));
        let mut t = p.clone();
        assert_eq!(t.div_assign_rem(q), *r);
        assert_eq!(t, *s);
        let mut t = p.clone();
        assert_eq!(t.div_assign_rem(q.clone()), *r);
        assert_eq!(t, *s);
        assert_eq!(p / q, *s);
        assert_eq!(p.clone() / q.clone(), *s);
        assert_eq!(p.clone() / q, *s);
        assert_eq!(p / q.clone(), *s);
        let mut t = p.clone();
        t /= q;
        assert_eq!(t, *s);
        let mut t = p.clone();
        t /= q.clone();
        assert_eq!(t, *s);
        assert_eq!(p % q, *r);
        assert_eq!(p.clone() % q.clone(), *r);
        assert_eq!(p.clone() % q, *r);
        assert_eq!(p % q.clone(), *r);
        let mut t = p.clone();
        t %= q;
        assert_eq!(t, *r);
        let mut t = p.clone();
        t %= q.clone();
        assert_eq!(t, *r);
    }
    x
}

#[test]
fn test_div_rem() {
    let test = |p, q, out: Option<(&str, &str)>| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q).map(|(s, r)| (s.to_string(), r.to_string())),
            out.map(|(s, r)| (s.to_string(), r.to_string()))
        );
    };
    // - the dividend is 0
    test("0", "x+1", Some(("0", "0")));
    // - the dividend is shorter than the divisor
    test("3*x+5", "2*x^2+1", Some(("0", "3*x+5")));
    // - a monic divisor
    test("x^3+2*x+1", "x+1", Some(("x^2-x+3", "-2")));
    // - a leading coefficient that divides every step
    test("6*x^2+3*x+4", "3*x", Some(("2*x+1", "4")));
    // - a leading coefficient that does not divide
    test("x^3+2*x+1", "2*x+1", None);
    // - a constant divisor
    test("6*x^2+3", "3", Some(("2*x^2+1", "0")));
    // - a constant divisor that does not divide
    test("5*x^2+3", "2", None);
    // - a negative leading coefficient
    test("3*x^2+6*x+1", "-3*x", Some(("-x-2", "1")));
    // - an exact division
    test("6*x^2+5*x+1", "2*x+1", Some(("3*x+1", "0")));
    // - the divisor is 0
    test("x+1", "0", None);
}

// The basecase and divide-and-conquer, at lengths that reach each case of divide-and-conquer,
// against the naive version.
#[test]
fn test_div_rem_algorithms() {
    let coefficients = |n: usize, seed: i64| -> Vec<Integer> {
        (0..n)
            .map(|i| {
                let i = i64::try_from(i).unwrap();
                Integer::from((i * i * 7 + i * seed + 3) % 101 - 50)
            })
            .collect()
    };
    for &(p_len, q_len) in &[(20, 17), (40, 30), (55, 30), (200, 30), (130, 20), (300, 100)] {
        for lead in [1, -1, 3] {
            let s = IntegerPolynomial::from_coefficients_asc(coefficients(p_len - q_len + 1, 1));
            let mut ys = coefficients(q_len, 2);
            *ys.last_mut().unwrap() = Integer::from(lead);
            let q = IntegerPolynomial::from_coefficients_asc(ys);
            let r = IntegerPolynomial::from_coefficients_asc(coefficients(q_len - 1, 3));
            // - the quotient is integral
            let p = &q * &s + &r;
            assert_eq!(verify(&p, &q), Some((s.clone(), r)));
            // - the quotient is not integral
            if lead == 3 {
                let p = &p + IntegerPolynomial::from_str(&format!("x^{}", p_len - 1)).unwrap();
                assert_eq!(verify(&p, &q), None);
            }
        }
    }
}

#[test]
#[should_panic]
fn div_rem_fail_1() {
    // The divisor is 0.
    IntegerPolynomial::from_str("x+1")
        .unwrap()
        .div_rem(IntegerPolynomial::ZERO);
}

#[test]
#[should_panic]
fn div_rem_fail_2() {
    // The quotient is not integral.
    IntegerPolynomial::from_str("x+1")
        .unwrap()
        .div_rem(IntegerPolynomial::from_str("2*x").unwrap());
}

#[test]
#[should_panic]
fn div_assign_rem_fail() {
    let mut p = IntegerPolynomial::from_str("x+1").unwrap();
    p.div_assign_rem(IntegerPolynomial::from_str("2*x").unwrap());
}

#[test]
#[should_panic]
fn div_fail() {
    let _ =
        IntegerPolynomial::from_str("x+1").unwrap() / IntegerPolynomial::from_str("2*x").unwrap();
}

#[test]
#[should_panic]
fn div_assign_fail() {
    let mut p = IntegerPolynomial::from_str("x+1").unwrap();
    p /= IntegerPolynomial::from_str("2*x").unwrap();
}

#[test]
#[should_panic]
fn rem_fail() {
    let _ =
        IntegerPolynomial::from_str("x+1").unwrap() % IntegerPolynomial::from_str("2*x").unwrap();
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut p = IntegerPolynomial::from_str("x+1").unwrap();
    p %= IntegerPolynomial::from_str("2*x").unwrap();
}

#[test]
fn div_rem_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        verify(&p, &q);
    });

    integer_polynomial_pair_gen_var_4().test_properties(|(p, q)| {
        // A multiple of the divisor divides with no remainder.
        let (s, r) = verify(&p, &q).unwrap();
        assert_eq!(r, IntegerPolynomial::ZERO);
        assert_eq!(&s * &q, p);
    });
}
