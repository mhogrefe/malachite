// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1,
    natural_polynomial_natural_polynomial_natural_triple_gen_var_2,
};

// Checks the rem of `p` by `q` in every way of calling it against the corresponding part of the
// quotient and remainder, and returns it.
fn verify(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> Option<NaturalPolynomial> {
    let r = p.checked_mod_rem(q, m);
    assert_eq!(p.clone().checked_mod_rem(q.clone(), m), r);
    assert_eq!(p.clone().checked_mod_rem(q, m), r);
    assert_eq!(p.checked_mod_rem(q.clone(), m), r);
    assert_eq!(p.checked_mod_div_rem(q, m).map(|x| x.1), r);
    if let Some(s) = &r {
        assert!(s.is_valid());
        assert_eq!(p.mod_rem(q, m), *s);
        assert_eq!(p.clone().mod_rem(q.clone(), m), *s);
        assert_eq!(p.clone().mod_rem(q, m), *s);
        assert_eq!(p.mod_rem(q.clone(), m), *s);
        assert_eq!(p.mod_rem(q, m.clone()), *s);
        let mut u = p.clone();
        u.mod_rem_assign(q, m);
        assert_eq!(u, *s);
        let mut u = p.clone();
        u.mod_rem_assign(q.clone(), m);
        assert_eq!(u, *s);
    }
    r
}

#[test]
fn test_mod_rem() {
    let test = |p, q, m: &str, out: Option<&str>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(
            verify(&p, &q, &m).map(|s| s.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - modulo 1
    test("0", "0", "1", Some("0"));
    // - the divisor is 0
    test("3*x+5", "0", "7", None);
    // - the divisor's leading coefficient is not invertible
    test("x^3+x+1", "2*x+1", "4", None);
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", "7", Some("5"));
    // - the dividend is 0
    test("0", "x+1", "7", Some("0"));
    // - a constant divisor
    test("6*x^2+5", "3", "7", Some("0"));
    // - a non-monic divisor
    test("x^5+3*x^4+2*x+1", "3*x^2+x+1", "7", Some("3*x+6"));
    // - a modulus of two limbs
    test(
        "x^4+3*x+1",
        "5*x^2+7",
        "1267650600228229401496703205653",
        Some("3*x+862002408155195993017758179847"),
    );
    // - a modulus of more than two limbs, the divisor is 0
    test(
        "3*x+5",
        "0",
        "1361129467683753853853498429727072845827",
        None,
    );
    // - a modulus of more than two limbs, the dividend is shorter
    test(
        "5",
        "3*x+2",
        "1361129467683753853853498429727072845827",
        Some("5"),
    );
    // - a modulus of more than two limbs, a constant divisor
    test(
        "6*x^2+5",
        "3",
        "1361129467683753853853498429727072845827",
        Some("0"),
    );
    // - a modulus of more than two limbs
    test(
        "x^5+3*x^4+2*x+1",
        "3*x^2+x+1",
        "1361129467683753853853498429727072845827",
        Some("1344325400181485287756541658989701576128*x+84020337511342830484783853686856348509"),
    );
    // - a modulus of more than two limbs, the divisor's low coefficients are 0
    test(
        "x^3+2*x+1",
        "x^2",
        "1361129467683753853853498429727072845827",
        Some("2*x+1"),
    );
    // - a modulus of more than two limbs, not invertible
    test(
        "x^3+x+1",
        "3*x+1",
        "4083388403051261561560495289181218537481",
        None,
    );
}

#[test]
#[should_panic]
fn mod_rem_fail_1() {
    // The divisor is 0.
    NaturalPolynomial::from_str("x+1").unwrap().mod_rem(
        NaturalPolynomial::from_str("0").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_rem_fail_2() {
    // The divisor's leading coefficient is not invertible.
    NaturalPolynomial::from_str("x^2+1").unwrap().mod_rem(
        NaturalPolynomial::from_str("2*x+1").unwrap(),
        Natural::from(6u32),
    );
}

#[test]
#[should_panic]
fn mod_rem_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x^2+1").unwrap();
    p.mod_rem_assign(
        NaturalPolynomial::from_str("2*x+1").unwrap(),
        Natural::from(6u32),
    );
}

#[test]
#[should_panic]
fn checked_mod_rem_fail() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("7*x+1")
        .unwrap()
        .checked_mod_rem(
            NaturalPolynomial::from_str("x+1").unwrap(),
            Natural::from(7u32),
        );
}

#[test]
fn mod_rem_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            verify(&p, &q, &m);
        },
    );

    natural_polynomial_natural_polynomial_natural_triple_gen_var_2().test_properties(
        |(p, q, m)| {
            verify(&p, &q, &m).unwrap();
        },
    );
}
