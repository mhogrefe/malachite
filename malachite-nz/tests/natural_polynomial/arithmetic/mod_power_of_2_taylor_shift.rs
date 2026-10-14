// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Neg,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::{
    ModPowerOf2Evaluate, ModPowerOf2TaylorShift, ModPowerOf2TaylorShiftAssign, Polynomial,
};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::arithmetic::mod_power_of_2_taylor_shift::{
    mod_power_of_2_taylor_shift_divide_and_conquer,
    mod_power_of_2_taylor_shift_divide_and_conquer_split, mod_power_of_2_taylor_shift_horner,
};
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_natural_unsigned_quadruple_gen_var_1,
    natural_polynomial_natural_unsigned_triple_gen_var_1,
    natural_polynomial_natural_unsigned_triple_gen_var_2,
    natural_polynomial_natural_unsigned_triple_gen_var_3,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_taylor_shift::*;

fn check(p: &NaturalPolynomial, c: &Natural, pow: u64) -> NaturalPolynomial {
    let q = p.mod_power_of_2_taylor_shift(c, pow);
    assert!(q.mod_power_of_2_is_reduced(pow));
    assert_eq!(p.clone().mod_power_of_2_taylor_shift(c.clone(), pow), q);
    assert_eq!(p.clone().mod_power_of_2_taylor_shift(c, pow), q);
    assert_eq!(p.mod_power_of_2_taylor_shift(c.clone(), pow), q);
    let mut r = p.clone();
    r.mod_power_of_2_taylor_shift_assign(c, pow);
    assert_eq!(r, q);
    let mut r = p.clone();
    r.mod_power_of_2_taylor_shift_assign(c.clone(), pow);
    assert_eq!(r, q);
    let mut xs = p.coefficients_asc().to_vec();
    mod_power_of_2_taylor_shift_horner(&mut xs, c, pow);
    assert_eq!(xs, q.coefficients_asc());
    let mut xs = p.coefficients_asc().to_vec();
    mod_power_of_2_taylor_shift_divide_and_conquer(&mut xs, c, pow);
    assert_eq!(xs, q.coefficients_asc());
    if xs.len() > 1 {
        // One split, whatever the length, with the halves shifted by the default policy.
        let mut xs = p.coefficients_asc().to_vec();
        mod_power_of_2_taylor_shift_divide_and_conquer_split(&mut xs, c, pow);
        assert_eq!(xs, q.coefficients_asc());
    }
    q
}

#[test]
fn test_mod_power_of_2_taylor_shift() {
    let test = |s, c, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let c = Natural::from_str(c).unwrap();
        let q = check(&p, &c, pow);
        assert_eq!(q.to_string(), out);
        assert_eq!(mod_power_of_2_taylor_shift_naive(&p, &c, pow), q);
    };
    test("0", "0", 0, "0");
    test("0", "0", 4, "0");
    test("0", "3", 4, "0");
    test("5", "0", 4, "5");
    test("5", "3", 4, "5");
    test("x", "3", 4, "x+3");
    test("x^2+3*x+2", "1", 4, "x^2+5*x+6");
    test("x^2+3*x+2", "10", 4, "x^2+7*x+4");
    test("x^3", "1", 1, "x^3+x^2+x+1");
    test(
        "11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "3",
        4,
        "11*x^10+4*x^9+14*x^8+9*x^6+12*x^5+8*x^4+13*x^2+12*x+6",
    );
    test(
        "11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "1",
        8,
        "11*x^10+120*x^9+82*x^8+224*x^7+137*x^6+144*x^5+12*x^4+96*x^3+205*x^2+184*x+66",
    );
    test(
        "11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "6",
        8,
        "11*x^10+158*x^9+193*x^8+24*x^7+231*x^6+34*x^5+29*x^4+140*x^3+179*x^2+86*x+41",
    );
    test(
        "9223372036854775808*x^2+3*x+18446744073709551615",
        "18446744073709551615",
        64,
        "9223372036854775808*x^2+3*x+9223372036854775804",
    );
    test(
        "7*x^2+5*x+1267650600228229401496703205375",
        "633825300114114700748351602689",
        100,
        "7*x^2+19*x+633825300114114700748351602699",
    );
    test("x+3", "2", 2, "x+1");
}

#[test]
fn test_mod_power_of_2_taylor_shift_long() {
    // Polynomials long enough to be split: every coefficient 1, shifted by 1, by 3, and by a
    // multi-limb c, modulo a word-sized and a multi-limb power of 2.
    let p = NaturalPolynomial::from_coefficients_asc(vec![Natural::ONE; 300]);
    for (c, pow) in [
        // - c = 1, word-sized modulus, split
        (Natural::ONE, 64),
        // - c = 1, larger modulus, split
        (Natural::ONE, 100),
        // - c not 1, word-sized modulus, too short to split
        (Natural::from(3u32), 8),
        // - c not 1, larger modulus, split
        (Natural::from(3u32), 100),
        ((Natural::ONE << 90u32) + Natural::ONE, 100),
    ] {
        let q = check(&p, &c, pow);
        assert_eq!(mod_power_of_2_taylor_shift_naive(&p, &c, pow), q);
    }
    // - c not 1, modulus of more than 1024 bits, split
    let c = Natural::from(3u32);
    let q = check(&p, &c, 2000);
    assert_eq!(mod_power_of_2_taylor_shift_naive(&p, &c, 2000), q);
    // - c = 1, modulus of more than 256 bits, split
    let p = NaturalPolynomial::from_coefficients_asc(vec![Natural::ONE; 400]);
    let q = check(&p, &Natural::ONE, 300);
    assert_eq!(mod_power_of_2_taylor_shift_naive(&p, &Natural::ONE, 300), q);
    // - c not 1, word-sized modulus, split
    let p = NaturalPolynomial::from_coefficients_asc(vec![Natural::ONE; 700]);
    let c = Natural::from(3u32);
    let q = check(&p, &c, 8);
    assert_eq!(mod_power_of_2_taylor_shift_naive(&p, &c, 8), q);
}

#[test]
#[should_panic]
fn mod_power_of_2_taylor_shift_fail_1() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("16*x+1")
        .unwrap()
        .mod_power_of_2_taylor_shift(Natural::ONE, 4);
}

#[test]
#[should_panic]
fn mod_power_of_2_taylor_shift_fail_2() {
    // The shift is not reduced.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_taylor_shift(Natural::from(16u32), 4);
}

fn mod_power_of_2_taylor_shift_properties_helper(p: &NaturalPolynomial, c: &Natural, pow: u64) {
    let q = check(p, c, pow);
    // It is the exact shift, reduced.
    assert_eq!(mod_power_of_2_taylor_shift_naive(p, c, pow), q);
    // The degree and the leading coefficient are unchanged.
    assert_eq!(q.len(), p.len());
    assert_eq!(q.leading_coefficient(), p.leading_coefficient());
    // Shifting back by -c recovers p.
    assert_eq!(check(&q, &c.mod_power_of_2_neg(pow), pow), *p);
    // q(0) is p(c).
    assert_eq!(q.coefficient(0).clone(), p.mod_power_of_2_evaluate(c, pow));
}

#[test]
fn mod_power_of_2_taylor_shift_properties() {
    natural_polynomial_natural_unsigned_triple_gen_var_1().test_properties(|(p, c, pow)| {
        mod_power_of_2_taylor_shift_properties_helper(&p, &c, pow);
        // Shifting by 0 changes nothing.
        assert_eq!(check(&p, &Natural::ZERO, pow), p);
    });

    natural_polynomial_natural_unsigned_triple_gen_var_2().test_properties(|(p, c, pow)| {
        mod_power_of_2_taylor_shift_properties_helper(&p, &c, pow);
    });

    natural_polynomial_natural_unsigned_triple_gen_var_3().test_properties(|(p, c, pow)| {
        mod_power_of_2_taylor_shift_properties_helper(&p, &c, pow);
    });

    natural_polynomial_natural_natural_unsigned_quadruple_gen_var_1().test_properties(
        |(p, a, b, pow)| {
            // Shifting by a and then by b is shifting by a + b.
            let q = (&p).mod_power_of_2_taylor_shift(&a, pow);
            assert_eq!(
                q.mod_power_of_2_taylor_shift(&b, pow),
                p.mod_power_of_2_taylor_shift(a.mod_power_of_2_add(b, pow), pow)
            );
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<Limb>().test_properties(
        |(p, c, pow)| {
            // It agrees with the UnsignedPolynomial implementation.
            let q = (&p).mod_power_of_2_taylor_shift(c, pow);
            assert_eq!(
                NaturalPolynomial::from(p).mod_power_of_2_taylor_shift(Natural::from(c), pow),
                NaturalPolynomial::from(q)
            );
        },
    );
}
