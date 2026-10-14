// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{IsPowerOf2, ModAdd, ModIsReduced, ModNeg, PowerOf2};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{
    ModEvaluate, ModPowerOf2TaylorShift, ModTaylorShift, ModTaylorShiftAssign, Polynomial,
};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::arithmetic::mod_taylor_shift::{
    mod_taylor_shift_divide_and_conquer, mod_taylor_shift_divide_and_conquer_split,
    mod_taylor_shift_horner,
};
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_natural_natural_quadruple_gen_var_1,
    natural_polynomial_natural_natural_triple_gen_var_1,
    natural_polynomial_natural_natural_triple_gen_var_2,
    natural_polynomial_natural_natural_triple_gen_var_3,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_taylor_shift::*;

fn check(p: &NaturalPolynomial, c: &Natural, m: &Natural) -> NaturalPolynomial {
    let q = p.mod_taylor_shift(c, m);
    assert!(q.is_valid());
    assert!(q.mod_is_reduced(m));
    assert_eq!(p.mod_taylor_shift(c, m.clone()), q);
    assert_eq!(p.mod_taylor_shift(c.clone(), m), q);
    assert_eq!(p.mod_taylor_shift(c.clone(), m.clone()), q);
    assert_eq!(p.clone().mod_taylor_shift(c, m), q);
    assert_eq!(p.clone().mod_taylor_shift(c, m.clone()), q);
    assert_eq!(p.clone().mod_taylor_shift(c.clone(), m), q);
    assert_eq!(p.clone().mod_taylor_shift(c.clone(), m.clone()), q);
    let mut r = p.clone();
    r.mod_taylor_shift_assign(c, m);
    assert_eq!(r, q);
    let mut r = p.clone();
    r.mod_taylor_shift_assign(c, m.clone());
    assert_eq!(r, q);
    let mut r = p.clone();
    r.mod_taylor_shift_assign(c.clone(), m);
    assert_eq!(r, q);
    let mut r = p.clone();
    r.mod_taylor_shift_assign(c.clone(), m.clone());
    assert_eq!(r, q);
    let mut xs = p.coefficients_asc().to_vec();
    mod_taylor_shift_horner(&mut xs, c, m);
    assert_eq!(xs, q.coefficients_asc());
    let mut xs = p.coefficients_asc().to_vec();
    mod_taylor_shift_divide_and_conquer(&mut xs, c, m);
    assert_eq!(xs, q.coefficients_asc());
    if xs.len() > 1 && *m > 1u32 {
        // One split, whatever the length, with the halves shifted by the default policy.
        let mut xs = p.coefficients_asc().to_vec();
        mod_taylor_shift_divide_and_conquer_split(&mut xs, c, m);
        assert_eq!(xs, q.coefficients_asc());
    }
    q
}

#[test]
fn test_mod_taylor_shift() {
    let test = |s, c, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let c = Natural::from_str(c).unwrap();
        let m = Natural::from_str(m).unwrap();
        let q = check(&p, &c, &m);
        assert_eq!(q.to_string(), out);
        assert_eq!(mod_taylor_shift_naive(&p, &c, &m), q);
    };
    test("0", "0", "1", "0");
    test("0", "0", "5", "0");
    test("0", "3", "5", "0");
    test("5", "0", "7", "5");
    test("5", "3", "7", "5");
    test("x", "3", "5", "x+3");
    test("x^2+3*x+2", "3", "7", "x^2+2*x+6");
    test("x^2+3*x+2", "1", "7", "x^2+5*x+6");
    // Moduli that are not prime: binomial coefficients can vanish.
    test("x^4", "1", "4", "x^4+2*x^2+1");
    test("x^4", "1", "2", "x^4+1");
    test("x^3", "5", "6", "x^3+3*x^2+3*x+5");
    test(
        "5*x^5+4*x^4+3*x^3+2*x^2+x",
        "11",
        "12",
        "5*x^5+3*x^4+x^3+3*x^2+3*x+9",
    );
    test("x^6+x^3+1", "2", "9", "x^6+3*x^5+6*x^4+8*x^3+3*x^2+6*x+1");
    // Moduli just at and above a limb.
    test(
        "x^3+2*x+1",
        "18446744073709551615",
        "18446744073709551616",
        "x^3+18446744073709551613*x^2+5*x+18446744073709551614",
    );
    test(
        "x^3+2*x+1",
        "18446744073709551615",
        "18446744073709551617",
        "x^3+18446744073709551611*x^2+14*x+18446744073709551606",
    );
    test(
        "x^4+100*x^2+7",
        "340282366920938463463374607431768211455",
        "340282366920938463463374607431768211457",
        "x^4+340282366920938463463374607431768211449*x^3+124*x^2+\
        340282366920938463463374607431768211025*x+423",
    );
    test(
        "x^5+x",
        "3",
        "1000000000000000000000000000000",
        "x^5+15*x^4+90*x^3+270*x^2+406*x+246",
    );
}

#[test]
fn test_mod_taylor_shift_long() {
    // Polynomials long enough to be split, every coefficient 1, shifted by 1, by 3, and by a
    // multi-limb c, modulo word-sized, prime and composite, and multi-limb moduli.
    let two_limbs = Natural::power_of_2(64) + Natural::ONE;
    let big = Natural::power_of_2(300) + Natural::from(3u32);
    let huge = Natural::power_of_2(1100) + Natural::ONE;
    for (len, c, m) in [
        // - c = 1, word-sized modulus
        (300, Natural::ONE, Natural::from(1000003u32)),
        (300, Natural::ONE, Natural::from(6u32)),
        // - c = 1, modulus of at most 256 bits
        (300, Natural::ONE, two_limbs.clone()),
        // - c = 1, larger modulus
        (400, Natural::ONE, big.clone()),
        // - c not 1, word-sized modulus, too short to split
        (100, Natural::from(3u32), Natural::from(1000003u32)),
        // - c not 1, word-sized modulus, split
        (300, Natural::from(3u32), Natural::from(1000003u32)),
        (300, Natural::from(5u32), Natural::from(360u32)),
        (
            300,
            Natural::from(3u32),
            Natural::power_of_2(62) * Natural::from(3u32),
        ),
        // - c not 1, modulus of at most 1024 bits, split
        (100, Natural::from(3u32), two_limbs.clone()),
        (100, Natural::power_of_2(70), big.clone()),
        // - c not 1, larger modulus, split
        (100, Natural::from(3u32), huge.clone()),
    ] {
        let p = NaturalPolynomial::from_coefficients_asc(vec![Natural::ONE; len]);
        let q = check(&p, &c, &m);
        assert_eq!(mod_taylor_shift_naive(&p, &c, &m), q);
    }
}

#[test]
#[should_panic]
fn mod_taylor_shift_fail_1() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("7*x+1")
        .unwrap()
        .mod_taylor_shift(Natural::ONE, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_taylor_shift_fail_2() {
    // The shift is not reduced.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_taylor_shift(Natural::from(7u32), Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_taylor_shift_fail_3() {
    // The modulus is 0.
    NaturalPolynomial::ZERO.mod_taylor_shift(Natural::ZERO, Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_taylor_shift_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    p.mod_taylor_shift_assign(Natural::from(7u32), Natural::from(7u32));
}

fn mod_taylor_shift_properties_helper(p: &NaturalPolynomial, c: &Natural, m: &Natural) {
    let q = check(p, c, m);
    // It is the exact shift, reduced.
    assert_eq!(mod_taylor_shift_naive(p, c, m), q);
    // The degree and the leading coefficient are unchanged.
    assert_eq!(q.len(), p.len());
    assert_eq!(q.leading_coefficient(), p.leading_coefficient());
    // Shifting back by -c recovers p.
    assert_eq!(check(&q, &c.mod_neg(m), m), *p);
    // q(0) is p(c).
    assert_eq!(q.coefficient(0).clone(), p.mod_evaluate(c, m));
    // Modulo a power of 2, it agrees with mod_power_of_2_taylor_shift.
    if m.is_power_of_2() {
        let pow = m.significant_bits() - 1;
        assert_eq!(p.mod_power_of_2_taylor_shift(c, pow), q);
    }
}

#[test]
fn mod_taylor_shift_properties() {
    natural_polynomial_natural_natural_triple_gen_var_1().test_properties(|(p, c, m)| {
        mod_taylor_shift_properties_helper(&p, &c, &m);
        // Shifting by 0 changes nothing.
        assert_eq!(check(&p, &Natural::ZERO, &m), p);
    });

    natural_polynomial_natural_natural_triple_gen_var_2().test_properties(|(p, c, m)| {
        mod_taylor_shift_properties_helper(&p, &c, &m);
    });

    natural_polynomial_natural_natural_triple_gen_var_3().test_properties(|(p, c, m)| {
        mod_taylor_shift_properties_helper(&p, &c, &m);
    });

    natural_polynomial_natural_natural_natural_quadruple_gen_var_1().test_properties(
        |(p, a, b, m)| {
            // Shifting by a and then by b is shifting by a + b.
            let q = (&p).mod_taylor_shift(&a, &m);
            assert_eq!(
                q.mod_taylor_shift(&b, &m),
                p.mod_taylor_shift(a.mod_add(b, &m), &m)
            );
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<Limb>().test_properties(
        |(p, c, m)| {
            // It agrees with the UnsignedPolynomial implementation.
            let q = (&p).mod_taylor_shift(c, m);
            assert_eq!(
                NaturalPolynomial::from(p).mod_taylor_shift(Natural::from(c), Natural::from(m)),
                NaturalPolynomial::from(q)
            );
        },
    );
}
