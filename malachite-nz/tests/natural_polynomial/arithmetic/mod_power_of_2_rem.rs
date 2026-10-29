// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::*;

use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1,
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5,
};

// Checks the rem of `p` by `q` in every way of calling it against the corresponding part of the
// quotient and remainder, and returns it.
fn verify(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) -> Option<NaturalPolynomial> {
    let r = p.checked_mod_power_of_2_rem(q, pow);
    assert_eq!(p.clone().checked_mod_power_of_2_rem(q.clone(), pow), r);
    assert_eq!(p.clone().checked_mod_power_of_2_rem(q, pow), r);
    assert_eq!(p.checked_mod_power_of_2_rem(q.clone(), pow), r);
    assert_eq!(p.checked_mod_power_of_2_div_rem(q, pow).map(|x| x.1), r);
    if let Some(s) = &r {
        assert!(s.is_valid());
        assert_eq!(p.mod_power_of_2_rem(q, pow), *s);
        assert_eq!(p.clone().mod_power_of_2_rem(q.clone(), pow), *s);
        assert_eq!(p.clone().mod_power_of_2_rem(q, pow), *s);
        assert_eq!(p.mod_power_of_2_rem(q.clone(), pow), *s);
        let mut u = p.clone();
        u.mod_power_of_2_rem_assign(q, pow);
        assert_eq!(u, *s);
        let mut u = p.clone();
        u.mod_power_of_2_rem_assign(q.clone(), pow);
        assert_eq!(u, *s);
    }
    r
}

#[test]
fn test_mod_power_of_2_rem() {
    let test = |p, q, pow: u64, out: Option<&str>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, pow).map(|s| s.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - modulo 1
    test("0", "0", 0, Some("0"));
    // - the divisor is 0
    test("3*x+1", "0", 3, None);
    // - the divisor's leading coefficient is even
    test("x^3+2*x^2+3", "2*x+1", 2, None);
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", 3, Some("5"));
    // - a non-monic divisor
    test("x^4+7*x+5", "5*x^2+2", 3, Some("7*x+1"));
    // - an exponent of two limbs
    test(
        "x^4+7*x+5",
        "5*x^2+3",
        100,
        Some("7*x+811296384146066816957890051446"),
    );
    // - an exponent of more than two limbs, the divisor is 0
    test("3*x+1", "0", 200, None);
    // - an exponent of more than two limbs, the dividend is shorter
    test("5", "3*x+2", 200, Some("5"));
    // - an exponent of more than two limbs, a constant divisor
    test("6*x^2+5", "3", 200, Some("0"));
    // - an exponent of more than two limbs
    test(
        "x^4+7*x+5",
        "5*x^2+3",
        200,
        Some("7*x+1028440348325753776346855739098344065614209916020987414592886"),
    );
    // - an exponent of more than two limbs, the divisor's low coefficients are 0
    test("x^3+2*x+1", "x^2", 200, Some("2*x+1"));
    // - an exponent of more than two limbs, the leading coefficient is even
    test("x^4+7*x+5", "6*x^2+3", 200, None);
}

#[test]
#[should_panic]
fn mod_power_of_2_rem_fail_1() {
    // The divisor is 0.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_rem(NaturalPolynomial::from_str("0").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_rem_fail_2() {
    // The divisor's leading coefficient is not invertible.
    NaturalPolynomial::from_str("x^2+1")
        .unwrap()
        .mod_power_of_2_rem(NaturalPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_rem_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x^2+1").unwrap();
    p.mod_power_of_2_rem_assign(NaturalPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_rem_fail() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .checked_mod_power_of_2_rem(NaturalPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
fn mod_power_of_2_rem_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );

    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow).unwrap();
        },
    );
}
