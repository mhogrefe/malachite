// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModDerivative, ModPowerOf2Derivative, ModPowerOf2DerivativeAssign, Polynomial,
};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_derivative::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

fn test_mod_power_of_2_derivative_helper<T: PrimitiveUnsigned>() {
    let test = |s, pow, out| {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = (&p).mod_power_of_2_derivative(pow);
        assert!(q.is_valid());
        assert!(q.mod_power_of_2_is_reduced(pow));
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().mod_power_of_2_derivative(pow), q);
        let mut r = p.clone();
        r.mod_power_of_2_derivative_assign(pow);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mod_power_of_2_derivative_naive(&p, pow), q);
    };
    // A constant polynomial, including zero, has derivative zero.
    test("0", 0, "0");
    test("5", 3, "0");
    test("x^3+3*x^2+2*x+5", 8, "3*x^2+6*x+2");
    test("255*x^2", 8, "254*x");
    // Coefficients can vanish, and the derivative can lose more than one degree.
    test("x^3+3*x^2+2*x+1", 2, "3*x^2+2*x+2");
    test("x^5+x", 1, "x^4+1");
    test("x^4+x^3+1", 1, "x^2");
    test("x^2", 1, "0");
}

#[test]
fn test_mod_power_of_2_derivative() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_derivative_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_derivative_fail_1() {
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_derivative(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_derivative_fail_2() {
    (&UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap()).mod_power_of_2_derivative(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_derivative_fail_3() {
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_derivative_assign(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_derivative_fail_4() {
    UnsignedPolynomial::<u8>::from_str("x")
        .unwrap()
        .mod_power_of_2_derivative(9);
}

fn mod_power_of_2_derivative_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, _, pow)| {
        let q = (&p).mod_power_of_2_derivative(pow);
        assert!(q.is_valid());
        assert!(q.mod_power_of_2_is_reduced(pow));
        // The forms agree.
        assert_eq!(p.clone().mod_power_of_2_derivative(pow), q);
        let mut r = p.clone();
        r.mod_power_of_2_derivative_assign(pow);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mod_power_of_2_derivative_naive(&p, pow), q);

        // The degree drops by at least one.
        assert!(q.len() < p.len() || p.len() == 0);
        // It is the derivative modulo the power of 2, when that power fits in a `T`.
        if pow < T::WIDTH {
            assert_eq!((&p).mod_derivative(T::power_of_2(pow)), q);
        }
    });
}

#[test]
fn mod_power_of_2_derivative_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_derivative_properties_helper);
}
