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
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_pow_mod_polynomial::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks `mod_pow_mod_polynomial` in every way of calling it against the naive version, and returns
// its result.
fn verify<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    exp: u64,
    f: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    let r = a.mod_pow_mod_polynomial(exp, f, m);
    assert!(r.is_valid());
    assert_eq!(a.clone().mod_pow_mod_polynomial(exp, f.clone(), m), r);
    assert_eq!(a.clone().mod_pow_mod_polynomial(exp, f, m), r);
    assert_eq!(a.mod_pow_mod_polynomial(exp, f.clone(), m), r);
    let mut c = a.clone();
    c.mod_pow_mod_polynomial_assign(exp, f, m);
    assert_eq!(c, r);
    let mut c = a.clone();
    c.mod_pow_mod_polynomial_assign(exp, f.clone(), m);
    assert_eq!(c, r);
    assert_eq!(mod_pow_mod_polynomial_naive(a, exp, f, m), r);
    assert!(r.len() < f.len() || r == UnsignedPolynomial::ZERO);
    r
}

fn test_mod_pow_mod_polynomial_helper<T: PrimitiveUnsigned>() {
    let test = |a, exp, f, m: u8, out| {
        let a = UnsignedPolynomial::<T>::from_str(a).unwrap();
        let f = UnsignedPolynomial::<T>::from_str(f).unwrap();
        assert_eq!(verify(&a, exp, &f, T::from(m)).to_string(), out);
    };
    // - modulo 1
    test("0", 5, "0", 1, "0");
    // - a constant modulus
    test("0", 5, "3", 7, "0");
    // - a modulus of degree 1
    test("4", 5, "2*x+3", 7, "2");
    // - exponent 0
    test("3*x^2+x+5", 0, "x^3+2*x+1", 7, "1");
    // - exponent 0, base 0
    test("0", 0, "x^3+2*x+1", 7, "1");
    // - exponent 1
    test("3*x^2+x+5", 1, "x^3+2*x+1", 7, "3*x^2+x+5");
    // - exponent 2
    test("3*x^2+x+5", 2, "x^3+2*x+1", 7, "6*x^2+3*x+5");
    // - base 0
    test("0", 3, "x^3+2*x+1", 7, "0");
    // - binary exponentiation
    test("3*x^2+x+5", 10, "x^3+2*x+1", 7, "2*x+2");
    // - a non-monic modulus
    test("2*x+3", 5, "3*x^2+x+5", 7, "6*x");
    // - a composite modulus
    test("3*x+1", 7, "x^2+x+1", 6, "3*x+1");
    // - a base of x, exponent 2
    test("x", 2, "x^3+2*x+1", 7, "x^2");
    // - a base of x by the sliding window, the smallest exponent
    test("x", 3, "x^3+2*x+1", 7, "5*x+6");
    // - a base of x by the sliding window
    test("x", 100, "x^3+2*x+1", 7, "x^2+3*x+5");
    // - a base of x by the sliding window, with a window of more than 1
    test(
        "x",
        1000,
        "x^9+3*x^2+1",
        7,
        "2*x^8+5*x^7+4*x^6+x^5+6*x^4+4*x^2+6*x+6",
    );
}

#[test]
fn test_mod_pow_mod_polynomial() {
    apply_fn_to_unsigneds!(test_mod_pow_mod_polynomial_helper);
}

#[test]
#[should_panic]
fn mod_pow_mod_polynomial_fail_1() {
    let _ = UnsignedPolynomial::<u8>::from_str("x^2")
        .unwrap()
        .mod_pow_mod_polynomial(3, UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_pow_mod_polynomial_fail_2() {
    let _ =
        UnsignedPolynomial::<u8>::ZERO.mod_pow_mod_polynomial(3, UnsignedPolynomial::<u8>::ZERO, 7);
}

#[test]
#[should_panic]
fn mod_pow_mod_polynomial_fail_3() {
    let _ = UnsignedPolynomial::<u8>::from_str("1")
        .unwrap()
        .mod_pow_mod_polynomial(3, UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap(), 4);
}

fn mod_pow_mod_polynomial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_polynomial_unsigned_quadruple_gen_var_1::<T>()
        .test_properties(|(a, exp, f, m)| {
            verify(&a, exp, &f, m);
        });
}

#[test]
fn mod_pow_mod_polynomial_properties() {
    apply_fn_to_unsigneds!(mod_pow_mod_polynomial_properties_helper);
}
