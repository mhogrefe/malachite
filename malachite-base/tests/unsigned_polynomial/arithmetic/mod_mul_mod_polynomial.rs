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
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul_mod_polynomial::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks `mod_mul_mod_polynomial` in every way of calling it against the naive version, and returns
// its result.
fn verify<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    b: &UnsignedPolynomial<T>,
    f: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    let r = a.mod_mul_mod_polynomial(b, f, m);
    assert!(r.is_valid());
    assert_eq!(a.clone().mod_mul_mod_polynomial(b.clone(), f.clone(), m), r);
    assert_eq!(a.clone().mod_mul_mod_polynomial(b, f.clone(), m), r);
    assert_eq!(a.clone().mod_mul_mod_polynomial(b.clone(), f, m), r);
    assert_eq!(a.mod_mul_mod_polynomial(b.clone(), f.clone(), m), r);
    let mut c = a.clone();
    c.mod_mul_mod_polynomial_assign(b, f, m);
    assert_eq!(c, r);
    let mut c = a.clone();
    c.mod_mul_mod_polynomial_assign(b.clone(), f.clone(), m);
    assert_eq!(c, r);
    assert_eq!(mod_mul_mod_polynomial_naive(a, b, f, m), r);
    assert!(r.len() < f.len() || r == UnsignedPolynomial::ZERO);
    r
}

fn test_mod_mul_mod_polynomial_helper<T: PrimitiveUnsigned>() {
    let test = |a, b, f, m: u8, out| {
        let a = UnsignedPolynomial::<T>::from_str(a).unwrap();
        let b = UnsignedPolynomial::<T>::from_str(b).unwrap();
        let f = UnsignedPolynomial::<T>::from_str(f).unwrap();
        assert_eq!(verify(&a, &b, &f, T::from(m)).to_string(), out);
    };
    // - modulo 1
    test("0", "0", "0", 1, "0");
    // - a constant modulus
    test("0", "0", "3", 7, "0");
    // - the first factor is 0
    test("0", "x+1", "x^2+1", 7, "0");
    // - the product is shorter than f
    test("x+2", "3", "x^2+1", 7, "3*x+6");
    // - a product reduced modulo f
    test("3*x^2+x+5", "x^2+4*x+2", "x^3+2*x+1", 7, "2*x^2+4");
    // - a non-monic f
    test("2*x+3", "4*x+1", "3*x^2+x+5", 7, "2*x+6");
    // - a composite modulus
    test("3*x+1", "5*x+2", "x^2+x+1", 6, "2*x+5");
}

#[test]
fn test_mod_mul_mod_polynomial() {
    apply_fn_to_unsigneds!(test_mod_mul_mod_polynomial_helper);
}

#[test]
#[should_panic]
fn mod_mul_mod_polynomial_fail_1() {
    let _ = UnsignedPolynomial::<u8>::from_str("x^2")
        .unwrap()
        .mod_mul_mod_polynomial(
            UnsignedPolynomial::<u8>::from_str("1").unwrap(),
            UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap(),
            7,
        );
}

#[test]
#[should_panic]
fn mod_mul_mod_polynomial_fail_2() {
    let _ = UnsignedPolynomial::<u8>::ZERO.mod_mul_mod_polynomial(
        UnsignedPolynomial::<u8>::ZERO,
        UnsignedPolynomial::<u8>::ZERO,
        7,
    );
}

#[test]
#[should_panic]
fn mod_mul_mod_polynomial_fail_3() {
    let _ = UnsignedPolynomial::<u8>::from_str("1")
        .unwrap()
        .mod_mul_mod_polynomial(
            UnsignedPolynomial::<u8>::from_str("1").unwrap(),
            UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap(),
            4,
        );
}

fn mod_mul_mod_polynomial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_triple_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(a, b, f, m)| {
            verify(&a, &b, &f, m);
        },
    );
}

#[test]
fn mod_mul_mod_polynomial_properties() {
    apply_fn_to_unsigneds!(mod_mul_mod_polynomial_properties_helper);
}
