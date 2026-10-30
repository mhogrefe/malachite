// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, DivisibleBy};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::*;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::pseudo_div_rem::*;

// Checks the checked exact quotient of `p` by `q`, and divisibility, in every way of calling them,
// against the quotient and remainder; when `q` divides `p`, checks the exact quotient too; and
// returns the checked quotient.
fn verify(p: &IntegerPolynomial, q: &IntegerPolynomial) -> Option<IntegerPolynomial> {
    let x = p.checked_div_exact(q);
    assert_eq!(p.clone().checked_div_exact(q.clone()), x);
    assert_eq!(p.clone().checked_div_exact(q), x);
    assert_eq!(p.checked_div_exact(q.clone()), x);
    let naive = match div_rem_naive(p, q) {
        Some((s, r)) if r == IntegerPolynomial::ZERO => Some(s),
        _ => None,
    };
    assert_eq!(x, naive);
    let divisible = p.divisible_by(q);
    assert_eq!(p.clone().divisible_by(q.clone()), divisible);
    assert_eq!(p.clone().divisible_by(q), divisible);
    assert_eq!(p.divisible_by(q.clone()), divisible);
    assert_eq!(divisible, x.is_some() || *p == IntegerPolynomial::ZERO);
    if let Some(s) = &x {
        assert!(s.is_valid());
        assert_eq!(p.div_exact(q), *s);
        assert_eq!(p.clone().div_exact(q.clone()), *s);
        assert_eq!(p.clone().div_exact(q), *s);
        assert_eq!(p.div_exact(q.clone()), *s);
        let mut t = p.clone();
        t.div_exact_assign(q);
        assert_eq!(t, *s);
        let mut t = p.clone();
        t.div_exact_assign(q.clone());
        assert_eq!(t, *s);
        assert_eq!(q * s, *p);
    }
    x
}

#[test]
fn test_div_exact_by_polynomial() {
    let test = |p, q, out: Option<&str>| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q).map(|s| s.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - the dividend is 0
    test("0", "x+1", Some("0"));
    // - a constant divisor
    test("6*x^2+3", "3", Some("2*x^2+1"));
    // - a monic divisor
    test("x^3+3*x^2+3*x+1", "x+1", Some("x^2+2*x+1"));
    // - a non-monic divisor
    test("6*x^3+13*x^2+8*x+5", "3*x+5", Some("2*x^2+x+1"));
    // - not divisible
    test("x^2+1", "x+1", None);
    // - the constant coefficients do not divide
    test("x^2+3", "2*x+2", None);
    // - the values at 1 do not divide
    test("x^2+x+2", "x^2+1", None);
    // - the dividend is shorter
    test("x+1", "x^2+1", None);
    // - the divisor is 0
    test("x+1", "0", None);
}

// Exact division from both ends, from the top, and by divide-and-conquer, at lengths that reach
// each, against multiplication.
#[test]
fn test_div_exact_by_polynomial_algorithms() {
    let coefficients = |n: usize, seed: i64| -> Vec<Integer> {
        (0..n)
            .map(|i| {
                let i = i64::try_from(i).unwrap();
                Integer::from((i * i * 7 + i * seed + 3) % 101 - 50)
            })
            .collect()
    };
    for &(s_len, q_len) in &[(5, 3), (50, 10), (150, 20), (300, 100), (300, 300), (1, 40)] {
        for (lead, low) in [(1, 1), (-3, 2), (6, 0)] {
            let s = IntegerPolynomial::from_coefficients_asc(coefficients(s_len, 1));
            let mut ys = coefficients(q_len, 2);
            *ys.last_mut().unwrap() = Integer::from(lead);
            ys[0] = Integer::from(low);
            let q = IntegerPolynomial::from_coefficients_asc(ys);
            let p = &q * &s;
            assert_eq!(verify(&p, &q), Some(s));
        }
    }
}

#[test]
#[should_panic]
fn div_exact_by_polynomial_fail() {
    IntegerPolynomial::from_str("x+1")
        .unwrap()
        .div_exact(IntegerPolynomial::ZERO);
}

#[test]
#[should_panic]
fn div_exact_assign_by_polynomial_fail() {
    let mut p = IntegerPolynomial::from_str("x+1").unwrap();
    p.div_exact_assign(IntegerPolynomial::ZERO);
}

#[test]
fn div_exact_by_polynomial_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        verify(&p, &q);
    });

    integer_polynomial_pair_gen_var_4().test_properties(|(p, q)| {
        let s = verify(&p, &q).unwrap();
        assert_eq!(s * q, p);
    });
}
