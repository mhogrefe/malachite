// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModDerivative, ModNthDerivative, ModNthDerivativeAssign, ModPowerOf2NthDerivative,
};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_5;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_nth_derivative::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

fn test_mod_nth_derivative_helper<T: PrimitiveUnsigned>() {
    let test = |s, n, m: u8, out| {
        let m = T::from(m);
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = (&p).mod_nth_derivative(n, m);
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().mod_nth_derivative(n, m), q);
        let mut r = p.clone();
        r.mod_nth_derivative_assign(n, m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mod_nth_derivative_naive(&p, n, m), q);
    };
    test("0", 3, 1, "0");
    test("x^4+3*x^3+2*x+4", 0, 5, "x^4+3*x^3+2*x+4");
    test("x^4+3*x^3+2*x+4", 1, 5, "4*x^3+4*x^2+2");
    test("x^4+3*x^3+2*x+4", 2, 5, "2*x^2+3*x");
    test("x^4+3*x^3+2*x+4", 4, 5, "4");
    test("x^4+3*x^3+2*x+4", 5, 5, "0");
    // 5 divides 5!.
    test("x^9+x^5", 5, 5, "0");
    // 6 divides 3!.
    test("x^4+x^3", 3, 6, "0");
    // 8 does not divide 3!, but it divides 4 * 3 * 2.
    test("x^4+x^3", 3, 8, "6");
    test("x^4+3*x^3+2*x+4", 2, 255, "12*x^2+18*x");
    test("254*x^3", 2, 255, "249*x");
}

#[test]
fn test_mod_nth_derivative() {
    apply_fn_to_unsigneds!(test_mod_nth_derivative_helper);
}

#[test]
#[should_panic]
fn mod_nth_derivative_fail_1() {
    UnsignedPolynomial::<u8>::from_str("10*x+1")
        .unwrap()
        .mod_nth_derivative(1, 10);
}

#[test]
#[should_panic]
fn mod_nth_derivative_fail_2() {
    (&UnsignedPolynomial::<u8>::from_str("10*x+1").unwrap()).mod_nth_derivative(1, 10);
}

#[test]
#[should_panic]
fn mod_nth_derivative_fail_3() {
    let mut p = UnsignedPolynomial::<u8>::from_str("10*x+1").unwrap();
    p.mod_nth_derivative_assign(1, 10);
}

#[test]
#[should_panic]
fn mod_nth_derivative_fail_4() {
    UnsignedPolynomial::<u8>::ZERO.mod_nth_derivative(1, 0);
}

fn mod_nth_derivative_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_5::<T>().test_properties(|(p, n, m)| {
        let q = (&p).mod_nth_derivative(n, m);
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        // The forms agree.
        assert_eq!(p.clone().mod_nth_derivative(n, m), q);
        let mut r = p.clone();
        r.mod_nth_derivative_assign(n, m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mod_nth_derivative_naive(&p, n, m), q);

        // The zeroth derivative is the polynomial itself, and the first is the derivative.
        assert_eq!((&p).mod_nth_derivative(0, m), p);
        assert_eq!((&p).mod_nth_derivative(1, m), (&p).mod_derivative(m));
        // Differentiating once more is the (n + 1)th derivative.
        assert_eq!((&q).mod_derivative(m), (&p).mod_nth_derivative(n + 1, m));
        // A power-of-2 modulus gives the same result as the nth derivative modulo that power of 2.
        if m.is_power_of_2() {
            assert_eq!((&p).mod_power_of_2_nth_derivative(n, m.trailing_zeros()), q);
        }
    });
}

#[test]
fn mod_nth_derivative_properties() {
    apply_fn_to_unsigneds!(mod_nth_derivative_properties_helper);
}
