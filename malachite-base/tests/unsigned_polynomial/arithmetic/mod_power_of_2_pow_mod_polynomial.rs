// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::*;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_pow_mod_polynomial;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks `mod_power_of_2_pow_mod_polynomial` in every way of calling it against the naive version,
// and returns its result.
fn verify<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    exp: u64,
    f: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    let r = a.mod_power_of_2_pow_mod_polynomial(exp, f, pow);
    assert!(r.is_valid());
    assert_eq!(
        a.clone()
            .mod_power_of_2_pow_mod_polynomial(exp, f.clone(), pow),
        r
    );
    assert_eq!(a.clone().mod_power_of_2_pow_mod_polynomial(exp, f, pow), r);
    assert_eq!(a.mod_power_of_2_pow_mod_polynomial(exp, f.clone(), pow), r);
    let mut c = a.clone();
    c.mod_power_of_2_pow_mod_polynomial_assign(exp, f, pow);
    assert_eq!(c, r);
    let mut c = a.clone();
    c.mod_power_of_2_pow_mod_polynomial_assign(exp, f.clone(), pow);
    assert_eq!(c, r);
    assert_eq!(
        mod_power_of_2_pow_mod_polynomial::mod_power_of_2_pow_mod_polynomial_naive(a, exp, f, pow),
        r
    );
    assert!(r.len() < f.len() || r == UnsignedPolynomial::ZERO);
    r
}

fn test_mod_power_of_2_pow_mod_polynomial_helper<T: PrimitiveUnsigned>() {
    let test = |a, exp, f, pow: u64, out| {
        let a = UnsignedPolynomial::<T>::from_str(a).unwrap();
        let f = UnsignedPolynomial::<T>::from_str(f).unwrap();
        assert_eq!(verify(&a, exp, &f, pow).to_string(), out);
    };
    // - modulo 1
    test("0", 5, "0", 0, "0");
    // - a constant modulus
    test("0", 5, "3", 3, "0");
    // - a modulus of degree 1
    test("5", 5, "3*x+1", 3, "5");
    // - exponent 0
    test("3*x^2+x+5", 0, "x^3+2*x+1", 3, "1");
    // - exponent 0, base 0
    test("0", 0, "x^3+2*x+1", 3, "1");
    // - exponent 1
    test("3*x^2+x+5", 1, "x^3+2*x+1", 3, "3*x^2+x+5");
    // - exponent 2
    test("3*x^2+x+5", 2, "x^3+2*x+1", 3, "5*x^2+5*x+3");
    // - base 0
    test("0", 3, "x^3+2*x+1", 3, "0");
    // - binary exponentiation
    test("3*x^2+x+5", 10, "x^3+2*x+1", 3, "5*x^2+x+7");
    // - a non-monic modulus
    test("2*x+3", 5, "3*x^2+x+5", 3, "2*x+3");
    // - the full width of a byte
    test("255*x+1", 7, "x^2+1", 8, "8*x+8");
    // - a base of x, exponent 2
    test("x", 2, "x^3+2*x+1", 3, "x^2");
    // - a base of x by the sliding window, the smallest exponent
    test("x", 3, "x^3+2*x+1", 3, "6*x+7");
    // - a base of x by the sliding window
    test("x", 100, "x^3+2*x+1", 3, "6*x^2+7*x");
    // - a base of x by the sliding window, with a window of more than 1
    test("x", 1000, "x^9+3*x^2+1", 3, "4*x^8+2*x^6+3*x^5+4*x^2+4*x+1");
}

#[test]
fn test_mod_power_of_2_pow_mod_polynomial() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_pow_mod_polynomial_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_mod_polynomial_fail_1() {
    let _ = UnsignedPolynomial::<u8>::from_str("x^2")
        .unwrap()
        .mod_power_of_2_pow_mod_polynomial(
            3,
            UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap(),
            3,
        );
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_mod_polynomial_fail_2() {
    let _ = UnsignedPolynomial::<u8>::ZERO.mod_power_of_2_pow_mod_polynomial(
        3,
        UnsignedPolynomial::<u8>::ZERO,
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_mod_polynomial_fail_3() {
    let _ = UnsignedPolynomial::<u8>::from_str("1")
        .unwrap()
        .mod_power_of_2_pow_mod_polynomial(
            3,
            UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap(),
            3,
        );
}

fn mod_power_of_2_pow_mod_polynomial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_polynomial_unsigned_quadruple_gen_var_2::<T>()
        .test_properties(|(a, exp, f, pow)| {
            verify(&a, exp, &f, pow);
        });
}

#[test]
fn mod_power_of_2_pow_mod_polynomial_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_pow_mod_polynomial_properties_helper);
}
