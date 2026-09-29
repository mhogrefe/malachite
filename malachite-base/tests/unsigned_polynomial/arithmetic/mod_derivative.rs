// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAdd, ModIsReduced};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModDerivative, ModDerivativeAssign, ModPowerOf2Derivative, MulPowerOfX, Polynomial,
};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_derivative::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

fn test_mod_derivative_helper<T: PrimitiveUnsigned>() {
    let test = |s, m: u8, out| {
        let m = T::from(m);
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = (&p).mod_derivative(m);
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().mod_derivative(m), q);
        let mut r = p.clone();
        r.mod_derivative_assign(m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mod_derivative_naive(&p, m), q);
    };
    // A constant polynomial, including zero, has derivative zero.
    test("0", 1, "0");
    test("5", 7, "0");
    test("x^3+3*x^2+2*x+5", 255, "3*x^2+6*x+2");
    test("254*x^2", 255, "253*x");
    // Coefficients can vanish, and the derivative can lose more than one degree.
    test("x^3+3*x^2+2*x+5", 6, "3*x^2+2");
    test("x^3+2*x+1", 3, "2");
    test("2*x^2+x", 4, "1");
    test("x^2", 2, "0");
}

#[test]
fn test_mod_derivative() {
    apply_fn_to_unsigneds!(test_mod_derivative_helper);
}

#[test]
#[should_panic]
fn mod_derivative_fail_1() {
    UnsignedPolynomial::<u8>::from_str("10*x+1")
        .unwrap()
        .mod_derivative(10);
}

#[test]
#[should_panic]
fn mod_derivative_fail_2() {
    (&UnsignedPolynomial::<u8>::from_str("10*x+1").unwrap()).mod_derivative(10);
}

#[test]
#[should_panic]
fn mod_derivative_fail_3() {
    let mut p = UnsignedPolynomial::<u8>::from_str("10*x+1").unwrap();
    p.mod_derivative_assign(10);
}

#[test]
#[should_panic]
fn mod_derivative_fail_4() {
    UnsignedPolynomial::<u8>::ZERO.mod_derivative(0);
}

fn mod_derivative_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(p, _, m)| {
        let q = (&p).mod_derivative(m);
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        // The forms agree.
        assert_eq!(p.clone().mod_derivative(m), q);
        let mut r = p.clone();
        r.mod_derivative_assign(m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mod_derivative_naive(&p, m), q);

        // The degree drops by at least one.
        assert!(q.len() < p.len() || p.len() == 0);
        // The product rule for x * p: (xp)' = p + xp'.
        assert_eq!(
            (&p).mul_power_of_x(1).mod_derivative(m),
            p.clone().mod_add(q.mul_power_of_x(1), m)
        );
        // A power-of-2 modulus gives the same result as the derivative modulo that power of 2.
        if m.is_power_of_2() {
            assert_eq!(
                (&p).mod_power_of_2_derivative(m.trailing_zeros()),
                (&p).mod_derivative(m)
            );
        }
    });
}

#[test]
fn mod_derivative_properties() {
    apply_fn_to_unsigneds!(mod_derivative_properties_helper);
}
