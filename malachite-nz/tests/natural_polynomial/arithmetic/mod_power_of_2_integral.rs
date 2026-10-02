// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2IsReduced, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    ComposePowerOfX, ModIntegral, ModPowerOf2Derivative, ModPowerOf2Integral,
    ModPowerOf2IntegralAssign, Polynomial,
};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_integral as unsigned;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_natural_unsigned_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_integral::*;

#[test]
fn test_mod_power_of_2_integral() {
    let test = |s, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let r = (&p).mod_power_of_2_integral(pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().mod_power_of_2_integral(pow), r);
        let mut q = p.clone();
        q.mod_power_of_2_integral_assign(pow);
        assert_eq!(q, r);
        assert_eq!(mod_power_of_2_integral_naive(&p, pow).unwrap(), r);
    };
    // - p.coefficients.is_empty(), for every pow
    test("0", 0, "0");
    test("0", 3, "0");
    // - n < 2: a constant moves to x without a division
    test("5", 3, "5*x");
    // - n >= 2
    // - *c != 0u32, with an odd index
    // Dividing by 3 is multiplying by 3 modulo 8.
    test("x^2", 3, "3*x^3");
    // - *c == 0u32: the coefficient of x is zero, so 2 need not be inverted
    test("3*x^2+5", 3, "x^3+5*x");
    // Dividing by 5 is multiplying by 13 modulo 16.
    test("x^4+1", 4, "13*x^5+x");
    test("x^6+x^4+x^2+1", 8, "183*x^7+205*x^5+171*x^3+x");
    // Moduli of a word and wider.
    test("x^2", 64, "12297829382473034411*x^3");
    test(
        "x^4+x^2",
        100,
        "1014120480182583521197362564301*x^5+845100400152152934331135470251*x^3",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_integral_fail_1() {
    // The coefficient of x is nonzero, and 2 is not a unit modulo 8.
    let _ = (&NaturalPolynomial::from_str("x").unwrap()).mod_power_of_2_integral(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_integral_fail_2() {
    // The coefficient of x^3 is nonzero, and 4 is not a unit modulo 8.
    let _ = (&NaturalPolynomial::from_str("x^3+x^2").unwrap()).mod_power_of_2_integral(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_integral_fail_3() {
    // A coefficient is not reduced.
    let _ = (&NaturalPolynomial::from_str("8").unwrap()).mod_power_of_2_integral(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_integral_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    p.mod_power_of_2_integral_assign(3);
}

#[test]
fn mod_power_of_2_integral_properties() {
    natural_polynomial_natural_unsigned_triple_gen_var_1().test_properties(|(p, _, pow)| {
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
        assert_eq!(*r.coefficient(0), Natural::ZERO);
        assert_eq!((&r).mod_power_of_2_derivative(pow), p);
        // It is the integral modulo 2^pow.
        assert_eq!((&p).mod_integral(Natural::power_of_2(pow)), r);
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().test_properties(
        |(p, _, pow)| {
            let p = p.compose_power_of_x(2);
            assert!(unsigned::mod_power_of_2_integral_is_defined(&p, pow));
            // The u64 and Natural versions agree.
            assert_eq!(
                NaturalPolynomial::from((&p).mod_power_of_2_integral(pow)),
                NaturalPolynomial::from(p).mod_power_of_2_integral(pow)
            );
        },
    );
}
