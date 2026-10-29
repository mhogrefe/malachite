// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::*;
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the quotient of `p` by `q` in every way of calling it against the corresponding part of
// the quotient and remainder, and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    let r = p.checked_mod_quotient(q, m);
    assert_eq!(p.clone().checked_mod_quotient(q.clone(), m), r);
    assert_eq!(p.clone().checked_mod_quotient(q, m), r);
    assert_eq!(p.checked_mod_quotient(q.clone(), m), r);
    assert_eq!(p.checked_mod_div_rem(q, m).map(|x| x.0), r);
    if let Some(s) = &r {
        assert!(s.is_valid());
        assert_eq!(p.mod_quotient(q, m), *s);
        assert_eq!(p.clone().mod_quotient(q.clone(), m), *s);
        assert_eq!(p.clone().mod_quotient(q, m), *s);
        assert_eq!(p.mod_quotient(q.clone(), m), *s);
        let mut u = p.clone();
        u.mod_quotient_assign(q, m);
        assert_eq!(u, *s);
        let mut u = p.clone();
        u.mod_quotient_assign(q.clone(), m);
        assert_eq!(u, *s);
    }
    r
}

fn test_mod_quotient_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, m: u8, out: Option<&str>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, T::from(m)).map(|s| s.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - modulo 1
    test("0", "0", 1, Some("0"));
    // - the divisor is 0
    test("3*x+5", "0", 7, None);
    // - the divisor's leading coefficient is not invertible
    test("x^3+x+1", "2*x+1", 4, None);
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", 7, Some("0"));
    // - the dividend is 0
    test("0", "x+1", 7, Some("0"));
    // - a constant divisor
    test("6*x^2+5", "3", 7, Some("2*x^2+4"));
    // - a monic divisor
    test("x^2+3*x+2", "x+1", 7, Some("x+2"));
    // - a quotient of one coefficient
    test("3*x+5", "x+1", 7, Some("3"));
    // - a quotient of two coefficients
    test("x^3+2*x+1", "x^2+3", 7, Some("x"));
    // - a non-monic divisor
    test("x^5+3*x^4+2*x+1", "3*x^2+x+1", 7, Some("5*x^3+4*x^2+4*x+2"));
    // - a composite modulus
    test("4*x^3+2*x+1", "5*x+3", 6, Some("2*x^2+4"));
    // - a modulus near the top of the word
    test("250*x^4+3*x+1", "249*x^2+7", 251, Some("126*x^2+190"));
}

#[test]
fn test_mod_quotient() {
    apply_fn_to_unsigneds!(test_mod_quotient_helper);
}

#[test]
#[should_panic]
fn mod_quotient_fail_1() {
    // The divisor is 0.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_quotient(UnsignedPolynomial::from_str("0").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_quotient_fail_2() {
    // The divisor's leading coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("x^2+1")
        .unwrap()
        .mod_quotient(UnsignedPolynomial::from_str("2*x+1").unwrap(), 6);
}

#[test]
#[should_panic]
fn mod_quotient_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    p.mod_quotient_assign(UnsignedPolynomial::from_str("2*x+1").unwrap(), 6);
}

#[test]
#[should_panic]
fn checked_mod_quotient_fail() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .checked_mod_quotient(UnsignedPolynomial::from_str("x+1").unwrap(), 7);
}

fn mod_quotient_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            verify(&p, &q, m);
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>().test_properties(
        |(p, q, m)| {
            verify(&p, &q, m).unwrap();
        },
    );
}

#[test]
fn mod_quotient_properties() {
    apply_fn_to_unsigneds!(mod_quotient_properties_helper);
}
