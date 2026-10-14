// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, OneHalf, Zero};
use malachite_base::polynomial::{Compose, ComposeAssign, Evaluate, Polynomial, TaylorShift};
use malachite_base::test_util::generators::common::SMALL_LIMIT;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_pair_gen, rational_polynomial_pair_gen_var_1,
    rational_polynomial_pair_gen_var_2, rational_polynomial_rational_pair_gen,
    rational_polynomial_triple_gen,
};
use malachite_q::test_util::rational_polynomial::arithmetic::compose::*;

fn check(p: &RationalPolynomial, q: &RationalPolynomial) -> RationalPolynomial {
    let r = p.compose(q);
    assert!(r.is_valid());
    assert_eq!(p.compose(q.clone()), r);
    assert_eq!(p.clone().compose(q), r);
    assert_eq!(p.clone().compose(q.clone()), r);
    let mut s = p.clone();
    s.compose_assign(q);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.compose_assign(q.clone());
    assert_eq!(s, r);
    r
}

#[test]
fn test_compose() {
    let test = |s, t, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        let r = check(&p, &q);
        assert_eq!(r.to_string(), out);
        assert_eq!(compose_naive(&p, &q), r);
    };
    test("0", "0", "0");
    test("0", "1/2*x+1", "0");
    test("5/3", "0", "5/3");
    test("5/3", "3*x^2+2*x+1", "5/3");
    test("3*x^2+2*x+1", "0", "1");
    test("3*x^2+2*x+1/2", "0", "1/2");
    test("3*x+1/2", "0", "1/2");
    test("3*x^2+2*x+1", "4/7", "153/49");
    test("x^2-1/3*x+1/2", "5/2", "71/12");
    test("x", "5*x^2+1/2*x+3", "5*x^2+1/2*x+3");
    test("3*x^2+2*x+1", "x", "3*x^2+2*x+1");
    test("3*x^2+2*x+1", "x+1", "3*x^2+8*x+6");
    test("3*x^2+2*x+1", "x+1/2", "3*x^2+5*x+11/4");
    test("1/2*x^2+1/3", "2*x+1/2", "2*x^2+x+11/24");
    test("x^3", "x^2+1/2*x", "x^6+3/2*x^5+3/4*x^4+1/8*x^3");
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x+3",
        "5*x^4+64*x^3+309*x^2+668*x+547",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "-x+3",
        "5*x^4-64*x^3+309*x^2-668*x+547",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "2/5*x+3/4",
        "16/125*x^4+152/125*x^3+231/50*x^2+347/40*x+1909/256",
    );
    test(
        "1/2*x^4+1/2*x^3+1/2*x^2+1/2*x+1/2",
        "1/2*x",
        "1/32*x^4+1/16*x^3+1/8*x^2+1/4*x+1/2",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2",
        "5*x^8+4*x^6+3*x^4+2*x^2+1",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2+1/3*x+1/2",
        "5*x^8+20/3*x^7+52/3*x^6+398/27*x^5+3439/162*x^4+311/27*x^3+37/3*x^2+7/2*x+57/16",
    );
    test(
        "1/6*x^4+1",
        "3*x^2-2/3*x+1",
        "27/2*x^8-12*x^7+22*x^6-340/27*x^5+2843/243*x^4-340/81*x^3+22/9*x^2-4/9*x+7/6",
    );
    test(
        "1/64*x^5+1/32*x^4-1/16*x^3+1/8*x^2-1/4*x+1/2",
        "2/3*x^2+1/3",
        "1/486*x^10+11/972*x^8-1/972*x^6+77/1944*x^4-943/7776*x^2+6667/15552",
    );
    test("1/2*x+1/2", "2*x+1", "x+1");
    test("1/2*x^2+1/2", "2*x+1", "2*x^2+2*x+1");
    test("1/4*x^5+2", "2*x", "8*x^5+2");
    test(
        "1/6*x^6+1/6*x^5+1/6*x^4+1/6*x^3+1/6*x^2+1/6*x+1/6",
        "1/2*x^2+1/2*x-1/2",
        "1/384*x^12+1/64*x^11+11/384*x^10-1/24*x^8+1/192*x^7+9/128*x^6+1/48*x^5-1/96*x^4+7/192*x^3\
        +23/384*x^2+1/32*x+43/384",
    );
    test(
        "-7/9*x^4+5*x^3+x^2+3/1180591620717411303424*x+18446744073709551616/3",
        "x^2+7/11*x-36893488147419103232/5",
        "-7/9*x^8-196/99*x^7+124995137843455921766951/5445*x^6+2624897894712574357274041/59895*x^5\
        -836988454527029287592035888066986072023945143/3294225*x^4-968416393667637192343829399995\
        49087087348687/299475*x^3+182599493043678147639437475327871411177372848504055175023631660\
        28639559129828987141/14609821306377964879872000*x^2+1161996773914315485073897774671557798\
        0689918537135832790936623974269305589566823497/14609821306377964879872000*x-4149988478265\
        41244759639470229410273733260963651094339336059661347031546985450799/180000",
    );
}

fn compose_properties_helper(p: &RationalPolynomial, q: &RationalPolynomial) {
    let r = check(p, q);
    assert_eq!(compose_naive(p, q), r);
    // The degree of p(q) is the product of the degrees, unless q is constant.
    if p.len() > 1 && q.len() > 1 {
        assert_eq!(r.len() - 1, (p.len() - 1) * (q.len() - 1));
    }
    // Evaluating p(q) at a point is evaluating p at q's value there.
    for x in [Rational::ZERO, Rational::ONE, -Rational::ONE_HALF] {
        assert_eq!((&r).evaluate(&x), p.evaluate(&q.evaluate(&x)));
    }
}

#[test]
fn compose_properties() {
    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        compose_properties_helper(&p, &q);
    });
}

#[test]
fn compose_properties_long() {
    rational_polynomial_pair_gen_var_1().test_properties(|(p, q)| {
        compose_properties_helper(&p, &q);
    });
}

#[test]
fn compose_properties_linear() {
    rational_polynomial_pair_gen_var_2().test_properties(|(p, q)| {
        compose_properties_helper(&p, &q);
        // Composing with a linear polynomial b x + a is a Taylor shift by a followed by composing
        // with b x.
        let b_x = RationalPolynomial::from_coefficients_asc(vec![
            Rational::ZERO,
            q.coefficient(1).clone(),
        ]);
        assert_eq!(
            (&p).compose(&q),
            p.taylor_shift(q.coefficient(0)).compose(b_x)
        );
    });
}

#[test]
fn compose_properties_identities() {
    rational_polynomial_gen().test_properties(|p| {
        let x = RationalPolynomial::from_coefficients_asc(vec![Rational::ZERO, Rational::ONE]);
        // x is an identity on both sides.
        assert_eq!((&p).compose(&x), p);
        assert_eq!((&x).compose(&p), p);
        // Composing with 0 gives the constant term.
        assert_eq!(
            (&p).compose(RationalPolynomial::ZERO),
            RationalPolynomial::from(p.coefficient(0).clone())
        );
        assert_eq!(
            RationalPolynomial::ZERO.compose(&p),
            RationalPolynomial::ZERO
        );
        // Composing integer polynomials agrees with IntegerPolynomial composition.
        let numerator = p.clone().into_numerator_and_denominator().0;
        let q = RationalPolynomial::from(numerator.clone());
        assert_eq!(
            (&q).compose(&q),
            RationalPolynomial::from((&numerator).compose(&numerator))
        );
    });
}

#[test]
fn compose_properties_constants() {
    rational_polynomial_rational_pair_gen().test_properties(|(p, c)| {
        // Composing with a constant evaluates.
        assert_eq!(
            (&p).compose(RationalPolynomial::from(c.clone())),
            RationalPolynomial::from((&p).evaluate(&c))
        );
        // Composing a constant with anything gives the constant.
        assert_eq!(
            RationalPolynomial::from(c.clone()).compose(&p),
            RationalPolynomial::from(c)
        );
    });
}

#[test]
fn compose_properties_algebra() {
    // Composing three polynomials makes the degrees multiply and the denominators grow fast, so
    // fewer cases are tried.
    rational_polynomial_triple_gen().test_properties_with_limit(SMALL_LIMIT, |(p, q, s)| {
        // Composition is associative.
        assert_eq!((&p).compose(&q).compose(&s), (&p).compose((&q).compose(&s)));
        // It distributes over addition and multiplication on the left.
        assert_eq!((&p + &q).compose(&s), (&p).compose(&s) + (&q).compose(&s));
        assert_eq!((&p * &q).compose(&s), (&p).compose(&s) * (&q).compose(&s));
    });
}
