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
    ComposePowerOfX, ModIntegral, ModPowerOf2Derivative, ModPowerOf2Integral,
    ModPowerOf2IntegralAssign, Polynomial,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_integral::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_power_of_2_integral() {
    fn test<T: PrimitiveUnsigned>(s: &str, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let r = (&p).mod_power_of_2_integral(pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().mod_power_of_2_integral(pow), r);
        let mut q = p.clone();
        q.mod_power_of_2_integral_assign(pow);
        assert_eq!(q, r);
        assert_eq!(mod_power_of_2_integral_naive(&p, pow).unwrap(), r);
    }
    // - p.coefficients.is_empty(), for every pow
    test::<u8>("0", 0, "0");
    test::<u8>("0", 3, "0");
    // - n < 2: a constant moves to x without a division
    test::<u8>("5", 3, "5*x");
    // - n >= 2
    // - *c != T::ZERO, with an odd index
    // Dividing by 3 is multiplying by 3 modulo 8.
    test::<u8>("x^2", 3, "3*x^3");
    // - *c == T::ZERO: the coefficient of x is zero, so 2 need not be inverted
    test::<u8>("3*x^2+5", 3, "x^3+5*x");
    // Dividing by 5 is multiplying by 13 modulo 16.
    test::<u8>("x^4+1", 4, "13*x^5+x");
    test::<u8>("x^6+x^4+x^2+1", 8, "183*x^7+205*x^5+171*x^3+x");
    // The full width of the type.
    test::<u64>("x^2", 64, "12297829382473034411*x^3");
}

#[test]
#[should_panic]
fn mod_power_of_2_integral_fail_1() {
    // The coefficient of x is nonzero, and 2 is not a unit modulo 8.
    let _ = (&UnsignedPolynomial::<u8>::from_str("x").unwrap()).mod_power_of_2_integral(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_integral_fail_2() {
    // The coefficient of x^3 is nonzero, and 4 is not a unit modulo 8.
    let _ = (&UnsignedPolynomial::<u8>::from_str("x^3+x^2").unwrap()).mod_power_of_2_integral(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_integral_fail_3() {
    // A coefficient is not reduced.
    let _ = (&UnsignedPolynomial::<u8>::from_str("8").unwrap()).mod_power_of_2_integral(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_integral_fail_4() {
    // pow is too large for the type.
    let _ = (&UnsignedPolynomial::<u8>::from_str("1").unwrap()).mod_power_of_2_integral(9);
}

#[test]
#[should_panic]
fn mod_power_of_2_integral_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_integral_assign(3);
}

fn mod_power_of_2_integral_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, _, pow)| {
        // Most polynomials have a nonzero coefficient of an odd power of x, so their integrals are
        // not defined; composing with x^2 gives one that is always defined.
        if !mod_power_of_2_integral_is_defined(&p, pow) {
            assert!(mod_power_of_2_integral_naive(&p, pow).is_none());
        }
        let p = p.compose_power_of_x(2);
        assert!(mod_power_of_2_integral_is_defined(&p, pow));
        let r = (&p).mod_power_of_2_integral(pow);
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(p.clone().mod_power_of_2_integral(pow), r);
        let mut q = p.clone();
        q.mod_power_of_2_integral_assign(pow);
        assert_eq!(q, r);

        // It is reduced, and is the coefficient-wise integral.
        assert!(r.mod_power_of_2_is_reduced(pow));
        assert_eq!(mod_power_of_2_integral_naive(&p, pow).unwrap(), r);
        // Its constant term is zero, and differentiating it gives back the polynomial.
        assert_eq!(r.coefficient(0), T::ZERO);
        assert_eq!((&r).mod_power_of_2_derivative(pow), p);
        // It is the integral modulo 2^pow, when 2^pow fits in the type.
        if pow < T::WIDTH {
            assert_eq!((&p).mod_integral(T::power_of_2(pow)), r);
        }
    });
}

#[test]
fn mod_power_of_2_integral_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_integral_properties_helper);
}
