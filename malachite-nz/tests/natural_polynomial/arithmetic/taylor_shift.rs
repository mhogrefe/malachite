// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::polynomial::{Evaluate, Polynomial, TaylorShift, TaylorShiftAssign};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_natural_triple_gen, natural_polynomial_natural_pair_gen,
    natural_polynomial_natural_pair_gen_var_2, natural_polynomial_natural_pair_gen_var_3,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::taylor_shift::taylor_shift_naive;

fn check(p: &NaturalPolynomial, c: &Natural) -> NaturalPolynomial {
    let q = p.taylor_shift(c);
    assert_eq!(p.clone().taylor_shift(c.clone()), q);
    assert_eq!(p.clone().taylor_shift(c), q);
    assert_eq!(p.taylor_shift(c.clone()), q);
    let mut r = p.clone();
    r.taylor_shift_assign(c);
    assert_eq!(r, q);
    let mut r = p.clone();
    r.taylor_shift_assign(c.clone());
    assert_eq!(r, q);
    // It agrees with the IntegerPolynomial shift.
    let p_integer = IntegerPolynomial::from(p.clone());
    let c_integer = Integer::from(c);
    assert_eq!(
        IntegerPolynomial::from(q.clone()),
        (&p_integer).taylor_shift(&c_integer)
    );
    assert_eq!(
        IntegerPolynomial::from(q.clone()),
        taylor_shift_naive(&p_integer, &c_integer)
    );
    q
}

#[test]
fn test_taylor_shift() {
    let test = |s, c, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let c = Natural::from_str(c).unwrap();
        assert_eq!(check(&p, &c).to_string(), out);
    };
    test("0", "0", "0");
    test("0", "5", "0");
    test("7", "0", "7");
    test("7", "4", "7");
    test("x", "3", "x+3");
    test("x^2", "1", "x^2+2*x+1");
    test("x^2+3*x+2", "0", "x^2+3*x+2");
    test("x^2+3*x+2", "1", "x^2+5*x+6");
    test("x^2+3*x+2", "10", "x^2+23*x+132");
    test("3*x^3+5", "2", "3*x^3+18*x^2+36*x+29");
    test(
        "11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "1",
        "11*x^10+120*x^9+594*x^8+1760*x^7+3465*x^6+4752*x^5+4620*x^4+3168*x^3+1485*x^2+440*x+66",
    );
    test(
        "11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "3",
        "11*x^10+340*x^9+4734*x^8+39104*x^7+212233*x^6+790908*x^5+2049800*x^4+3648736*x^3+4269981*x\
        ^2+2967212*x+930022",
    );
    test(
        "18446744073709551615*x^2+x+18446744073709551616",
        "1",
        "18446744073709551615*x^2+36893488147419103231*x+36893488147419103232",
    );
    test(
        "x+3",
        "1000000000000000000000000000000",
        "x+1000000000000000000000000000003",
    );
}

#[test]
fn test_taylor_shift_long() {
    // A polynomial long enough to be split, with every coefficient 1, shifted by 1 and by 2.
    let p = NaturalPolynomial::from_coefficients_asc(vec![Natural::ONE; 300]);
    check(&p, &Natural::ONE);
    check(&p, &Natural::TWO);
}

fn taylor_shift_properties_helper(p: &NaturalPolynomial, c: &Natural) {
    let q = check(p, c);
    // The degree and the leading coefficient are unchanged.
    assert_eq!(q.len(), p.len());
    assert_eq!(q.leading_coefficient(), p.leading_coefficient());
    // Since c is non-negative, no coefficient decreases.
    for i in 0..p.len() {
        assert!(q.coefficient(i) >= p.coefficient(i));
    }
    // q(0) is p(c).
    assert_eq!(*q.coefficient(0), p.evaluate(c));
}

#[test]
fn taylor_shift_properties() {
    natural_polynomial_natural_pair_gen().test_properties(|(p, c)| {
        taylor_shift_properties_helper(&p, &c);
        // Shifting by 0 changes nothing.
        assert_eq!(check(&p, &Natural::ZERO), p);
    });

    natural_polynomial_natural_pair_gen_var_2().test_properties(|(p, c)| {
        taylor_shift_properties_helper(&p, &c);
    });

    natural_polynomial_natural_pair_gen_var_3().test_properties(|(p, c)| {
        taylor_shift_properties_helper(&p, &c);
    });

    natural_polynomial_natural_natural_triple_gen().test_properties(|(p, a, b)| {
        // Shifting by a and then by b is shifting by a + b, and q(x) evaluates as p(x + a).
        let q = (&p).taylor_shift(&a);
        assert_eq!((&q).taylor_shift(&b), (&p).taylor_shift(&a + &b));
        assert_eq!(q.evaluate(&b), p.evaluate(&a + &b));
    });
}
