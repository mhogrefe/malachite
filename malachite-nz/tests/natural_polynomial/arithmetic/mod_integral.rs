// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::traits::{Two, Zero};
use malachite_base::polynomial::{ModDerivative, ModIntegral, ModIntegralAssign, Polynomial};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_integral as unsigned_integral;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_natural_natural_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_integral::*;

#[test]
fn test_mod_integral() {
    let test = |s, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).mod_integral(&m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_integral(m.clone()), r);
        assert_eq!(p.clone().mod_integral(&m), r);
        assert_eq!(p.clone().mod_integral(m.clone()), r);
        let mut q = p.clone();
        q.mod_integral_assign(&m);
        assert_eq!(q, r);
        let mut q = p.clone();
        q.mod_integral_assign(m.clone());
        assert_eq!(q, r);
        assert_eq!(mod_integral_naive(&p, &m).unwrap(), r);
    };
    // - p.coefficients.is_empty(), for every m, even when no k is a unit
    test("0", "1", "0");
    test("0", "2", "0");
    // - n == 1: a constant moves to x without a division
    test("5", "7", "5*x");
    test("1", "2", "x");
    // - n >= 2, n < 3: one division, by 2
    test("4*x+5", "7", "2*x^2+5*x");
    // - n >= 3
    test("3*x^2+4*x+5", "7", "x^3+2*x^2+5*x");
    test("x^2", "7", "5*x^3");
    // The loop past x^3.
    test("x^4+x^3+x^2+x+1", "7", "3*x^5+2*x^4+5*x^3+4*x^2+x");
    // - *c == 0u32: indices whose coefficients are zero need not be units. Here 2 is not a unit
    //   modulo 4 or 8, but the coefficient of x is zero; FLINT cannot integrate these.
    test("x^2", "4", "3*x^3");
    test("x^2", "8", "3*x^3");
    test("x^4+1", "16", "13*x^5+x");
    test("5*x^4+1", "6", "x^5+x");
    // A composite modulus whose prime factors exceed the degree plus 1.
    test("x^3+x^2+x+1", "35", "9*x^4+12*x^3+18*x^2+x");
    // Moduli wider than a word: a prime, and a product of two large numbers.
    test(
        "2*x^2+3",
        "170141183460469231731687303715884105727",
        "56713727820156410577229101238628035243*x^3+3*x",
    );
    test(
        "x^3+x^2+x+1",
        "170141183460469231731687303715884105727",
        "42535295865117307932921825928971026432*x^4+113427455640312821154458202477256070485*x^3+\
        85070591730234615865843651857942052864*x^2+x",
    );
    test(
        "8*x^3+7*x^2+6*x+5",
        "10000000000000000016800000000000000005031",
        "2*x^4+6666666666666666677866666666666666670023*x^3+3*x^2+5*x",
    );
    test(
        "x+12345678901234567890123",
        "10000000000000000016800000000000000005031",
        "5000000000000000008400000000000000002516*x^2+12345678901234567890123*x",
    );
}

#[test]
#[should_panic]
fn mod_integral_fail_1() {
    // 2 is not a unit modulo 2.
    let _ = (&NaturalPolynomial::from_str("x+1").unwrap()).mod_integral(Natural::TWO);
}

#[test]
#[should_panic]
fn mod_integral_fail_2() {
    // 3 is 0 modulo 3, so the product of the indices is 0.
    let _ = (&NaturalPolynomial::from_str("x^2+1").unwrap()).mod_integral(Natural::from(3u32));
}

#[test]
#[should_panic]
fn mod_integral_fail_3() {
    // The coefficient of x is nonzero, and 2 is not a unit modulo 4.
    let _ = (&NaturalPolynomial::from_str("x^2+x").unwrap()).mod_integral(Natural::from(4u32));
}

#[test]
#[should_panic]
fn mod_integral_fail_4() {
    // A coefficient is not reduced.
    let _ = NaturalPolynomial::from_str("7")
        .unwrap()
        .mod_integral(Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_integral_fail_5() {
    // m is 0.
    let _ = NaturalPolynomial::ZERO.mod_integral(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_integral_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    p.mod_integral_assign(&Natural::TWO);
}

#[test]
fn mod_integral_properties() {
    natural_polynomial_natural_natural_triple_gen_var_1().test_properties(|(p, _, m)| {
        if !mod_integral_is_defined(&p, &m) {
            assert!(mod_integral_naive(&p, &m).is_none());
            return;
        }
        let r = (&p).mod_integral(&m);
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!((&p).mod_integral(m.clone()), r);
        assert_eq!(p.clone().mod_integral(&m), r);
        let mut q = p.clone();
        q.mod_integral_assign(&m);
        assert_eq!(q, r);

        // It is reduced, and is the coefficient-wise integral.
        assert!(r.mod_is_reduced(&m));
        assert_eq!(mod_integral_naive(&p, &m).unwrap(), r);
        // Its constant term is zero, and differentiating it gives back the polynomial.
        assert_eq!(*r.coefficient(0), Natural::ZERO);
        assert_eq!((&r).mod_derivative(&m), p);
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>().test_properties(|(p, _, m)| {
        if !unsigned_integral::mod_integral_is_defined(&p, m) {
            return;
        }
        // The u64 and Natural versions agree.
        assert_eq!(
            NaturalPolynomial::from((&p).mod_integral(m)),
            NaturalPolynomial::from(p).mod_integral(Natural::from(m))
        );
    });
}
