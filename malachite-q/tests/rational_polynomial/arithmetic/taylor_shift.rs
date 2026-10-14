// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Reciprocal;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Evaluate, Polynomial, TaylorShift, TaylorShiftAssign};
use malachite_base::test_util::generators::common::SMALL_LIMIT;
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_rational_pair_gen, rational_polynomial_rational_pair_gen_var_1,
    rational_polynomial_rational_pair_gen_var_2, rational_polynomial_rational_rational_triple_gen,
};
use malachite_q::test_util::rational_polynomial::arithmetic::taylor_shift::taylor_shift_naive;

fn check(p: &RationalPolynomial, c: &Rational) -> RationalPolynomial {
    let q = p.taylor_shift(c);
    assert!(q.is_valid());
    assert_eq!(p.clone().taylor_shift(c.clone()), q);
    assert_eq!(p.clone().taylor_shift(c), q);
    assert_eq!(p.taylor_shift(c.clone()), q);
    let mut r = p.clone();
    r.taylor_shift_assign(c);
    assert_eq!(r, q);
    let mut r = p.clone();
    r.taylor_shift_assign(c.clone());
    assert_eq!(r, q);
    q
}

#[test]
fn test_taylor_shift() {
    let test = |s, c, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let c = Rational::from_str(c).unwrap();
        let q = check(&p, &c);
        assert_eq!(q.to_string(), out);
        assert_eq!(taylor_shift_naive(&p, &c), q);
    };
    test("0", "0", "0");
    test("0", "1/2", "0");
    test("7/3", "0", "7/3");
    test("7/3", "5/2", "7/3");
    test("x", "1/2", "x+1/2");
    test("x^2-3*x+2", "1", "x^2-x");
    test("x^2-3*x+2", "1/2", "x^2-2*x+3/4");
    test("1/2*x^2", "-1/3", "1/2*x^2-1/3*x+1/18");
    test("1/5*x^2+1/3*x+1/2", "2/7", "1/5*x^2+47/105*x+899/1470");
    test(
        "11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "3/4",
        "11*x^10+185/2*x^9+5679/16*x^8+6571/8*x^7+163331/128*x^6+358683/256*x^5+2281795/2048*x^4+13\
        22411/2048*x^3+17677551/65536*x^2+10222777/131072*x+14120011/1048576",
    );
    test(
        "11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "-5",
        "11*x^10-540*x^9+11934*x^8-156352*x^7+1344777*x^6-7934004*x^5+32517480*x^4-91416096*x^3+168\
        705693*x^2-184552228*x+90874566",
    );
    test("5/6*x^4+1/6", "6", "5/6*x^4+20*x^3+180*x^2+720*x+6481/6");
    test("4*x^3+3", "1/2", "4*x^3+6*x^2+3*x+7/2");
    test("2*x^3+4", "1/2", "2*x^3+3*x^2+3/2*x+17/4");
    test(
        "x+1000000000000000000000000000001/7",
        "-1/100000000000000000000",
        "x+100000000000000000000000000000099999999999999999993/700000000000000000000",
    );
}

#[test]
fn test_taylor_shift_long() {
    // A polynomial long enough for the numerator's shift to be split, shifted by 1/3 and back.
    let p = RationalPolynomial::from_coefficients_asc(
        (1..=300u32)
            .map(|i| Rational::from(i).reciprocal())
            .collect::<Vec<_>>(),
    );
    let c = Rational::from_signeds(1, 3);
    let q = check(&p, &c);
    assert_eq!(check(&q, &-&c), p);
}

// With `compare_naive`, the result is also checked against the binomial expansion in `Rational`
// arithmetic, which is slow for long polynomials.
fn taylor_shift_properties_helper(p: &RationalPolynomial, c: &Rational, compare_naive: bool) {
    let q = check(p, c);
    // It is the binomial expansion.
    if compare_naive {
        assert_eq!(taylor_shift_naive(p, c), q);
    }
    // The degree and the leading coefficient are unchanged.
    assert_eq!(q.len(), p.len());
    assert_eq!(q.leading_coefficient(), p.leading_coefficient());
    // Shifting back by -c recovers p.
    assert_eq!(check(&q, &-c), *p);
    // q(0) is p(c).
    assert_eq!(q.coefficient(0), p.evaluate(c));
}

#[test]
fn taylor_shift_properties() {
    rational_polynomial_rational_pair_gen().test_properties(|(p, c)| {
        taylor_shift_properties_helper(&p, &c, true);
        // Shifting by 0 changes nothing.
        assert_eq!(check(&p, &Rational::ZERO), p);
    });

    // Long rational polynomials have coefficients of thousands of bits, since their common
    // denominator has many factors, so fewer of them are tested.
    rational_polynomial_rational_pair_gen_var_1().test_properties_with_limit(
        SMALL_LIMIT,
        |(p, c)| {
            taylor_shift_properties_helper(&p, &c, false);
        },
    );

    rational_polynomial_rational_pair_gen_var_2().test_properties(|(p, c)| {
        taylor_shift_properties_helper(&p, &c, true);
        // Shifting by an integer shifts only the numerator, and keeps the denominator.
        let q = (&p).taylor_shift(&c);
        assert_eq!(
            *q.numerator_ref(),
            p.numerator_ref()
                .taylor_shift(Integer::try_from(&c).unwrap())
        );
        assert_eq!(q.denominator_ref(), p.denominator_ref());
    });

    rational_polynomial_rational_rational_triple_gen().test_properties(|(p, a, b)| {
        // Shifting by a and then by b is shifting by a + b, and q(x) evaluates as p(x + a).
        let q = (&p).taylor_shift(&a);
        assert_eq!((&q).taylor_shift(&b), (&p).taylor_shift(&a + &b));
        assert_eq!(q.evaluate(&b), p.evaluate(&a + &b));
    });
}
