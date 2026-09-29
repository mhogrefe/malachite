// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModNthDerivative, ModPowerOf2Derivative, ModPowerOf2NthDerivative,
    ModPowerOf2NthDerivativeAssign,
};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_6;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_nth_derivative::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

fn test_mod_power_of_2_nth_derivative_helper<T: PrimitiveUnsigned>() {
    let test = |s, n, pow, out| {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = (&p).mod_power_of_2_nth_derivative(n, pow);
        assert!(q.is_valid());
        assert!(q.mod_power_of_2_is_reduced(pow));
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().mod_power_of_2_nth_derivative(n, pow), q);
        let mut r = p.clone();
        r.mod_power_of_2_nth_derivative_assign(n, pow);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mod_power_of_2_nth_derivative_naive(&p, n, pow), q);
    };
    test("0", 3, 0, "0");
    test("x^4+3*x^3+2*x+5", 0, 3, "x^4+3*x^3+2*x+5");
    test("x^4+3*x^3+2*x+5", 1, 3, "4*x^3+x^2+2");
    test("x^4+3*x^3+2*x+5", 2, 3, "4*x^2+2*x");
    test("x^4+3*x^3+2*x+5", 3, 3, "2");
    // 2^3 divides 4!.
    test("x^5+x^4", 4, 3, "0");
    test("x^4+3*x^3+2*x+5", 2, 8, "12*x^2+18*x");
    test("255*x^3", 2, 8, "250*x");
}

#[test]
fn test_mod_power_of_2_nth_derivative() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_nth_derivative_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_nth_derivative_fail_1() {
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_nth_derivative(1, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_nth_derivative_fail_2() {
    (&UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap()).mod_power_of_2_nth_derivative(1, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_nth_derivative_fail_3() {
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_nth_derivative_assign(1, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_nth_derivative_fail_4() {
    UnsignedPolynomial::<u8>::ZERO.mod_power_of_2_nth_derivative(1, 9);
}

fn mod_power_of_2_nth_derivative_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_6::<T>().test_properties(|(p, n, pow)| {
        let q = (&p).mod_power_of_2_nth_derivative(n, pow);
        assert!(q.is_valid());
        assert!(q.mod_power_of_2_is_reduced(pow));
        // The forms agree.
        assert_eq!(p.clone().mod_power_of_2_nth_derivative(n, pow), q);
        let mut r = p.clone();
        r.mod_power_of_2_nth_derivative_assign(n, pow);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mod_power_of_2_nth_derivative_naive(&p, n, pow), q);

        // Differentiating once more is the (n + 1)th derivative.
        assert_eq!(
            (&q).mod_power_of_2_derivative(pow),
            (&p).mod_power_of_2_nth_derivative(n + 1, pow)
        );
        // It is the nth derivative modulo the power of 2, when that power fits in a `T`.
        if pow < T::WIDTH {
            assert_eq!((&p).mod_nth_derivative(n, T::power_of_2(pow)), q);
        }
        // If 2^pow divides n!, the result is zero.
        if n - u64::from(n.count_ones()) >= pow {
            assert_eq!(q, UnsignedPolynomial::ZERO);
        }
    });
}

#[test]
fn mod_power_of_2_nth_derivative_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_nth_derivative_properties_helper);
}
