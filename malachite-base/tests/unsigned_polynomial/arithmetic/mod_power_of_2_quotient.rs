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
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the quotient of `p` by `q` in every way of calling it against the corresponding part of
// the quotient and remainder, and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    let r = p.checked_mod_power_of_2_quotient(q, pow);
    assert_eq!(p.clone().checked_mod_power_of_2_quotient(q.clone(), pow), r);
    assert_eq!(p.clone().checked_mod_power_of_2_quotient(q, pow), r);
    assert_eq!(p.checked_mod_power_of_2_quotient(q.clone(), pow), r);
    assert_eq!(p.checked_mod_power_of_2_div_rem(q, pow).map(|x| x.0), r);
    if let Some(s) = &r {
        assert!(s.is_valid());
        assert_eq!(p.mod_power_of_2_quotient(q, pow), *s);
        assert_eq!(p.clone().mod_power_of_2_quotient(q.clone(), pow), *s);
        assert_eq!(p.clone().mod_power_of_2_quotient(q, pow), *s);
        assert_eq!(p.mod_power_of_2_quotient(q.clone(), pow), *s);
        let mut u = p.clone();
        u.mod_power_of_2_quotient_assign(q, pow);
        assert_eq!(u, *s);
        let mut u = p.clone();
        u.mod_power_of_2_quotient_assign(q.clone(), pow);
        assert_eq!(u, *s);
    }
    r
}

fn test_mod_power_of_2_quotient_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, pow: u64, out: Option<&str>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
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
    test("5", "3*x+2", 3, Some("0"));
    // - the dividend is 0
    test("0", "x+1", 3, Some("0"));
    // - a constant divisor
    test("6*x^2+5", "3", 3, Some("2*x^2+7"));
    // - a quotient of two coefficients
    test("3*x^2+x+1", "3*x+1", 3, Some("x"));
    // - a non-monic divisor
    test("x^4+7*x+5", "5*x^2+2", 3, Some("5*x^2+6"));
    // - the full width
    test("255*x^3+1", "253*x^2+3", 8, Some("171*x"));
}

#[test]
fn test_mod_power_of_2_quotient() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_quotient_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_quotient_fail_1() {
    // The divisor is 0.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_quotient(UnsignedPolynomial::from_str("0").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_quotient_fail_2() {
    // The divisor's leading coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("x^2+1")
        .unwrap()
        .mod_power_of_2_quotient(UnsignedPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_quotient_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    p.mod_power_of_2_quotient_assign(UnsignedPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_quotient_fail() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .checked_mod_power_of_2_quotient(UnsignedPolynomial::from_str("x+1").unwrap(), 3);
}

fn mod_power_of_2_quotient_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow).unwrap();
        },
    );
}

#[test]
fn mod_power_of_2_quotient_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_quotient_properties_helper);
}
