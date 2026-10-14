// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModEvaluate, ModPowerOf2TaylorShift, ModTaylorShift, ModTaylorShiftAssign, Polynomial,
};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_12,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_13,
    unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_5,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_taylor_shift::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_taylor_shift::{
    mod_taylor_shift_divide_and_conquer, mod_taylor_shift_divide_and_conquer_split,
    mod_taylor_shift_horner,
};

fn check<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, c: T, m: T) -> UnsignedPolynomial<T> {
    let q = p.mod_taylor_shift(c, m);
    assert!(q.is_valid());
    assert!(q.mod_is_reduced(&m));
    assert_eq!(p.clone().mod_taylor_shift(c, m), q);
    let mut r = p.clone();
    r.mod_taylor_shift_assign(c, m);
    assert_eq!(r, q);
    let mut xs = p.coefficients_asc().to_vec();
    mod_taylor_shift_horner(&mut xs, c, m);
    assert_eq!(xs, q.coefficients_asc());
    let mut xs = p.coefficients_asc().to_vec();
    mod_taylor_shift_divide_and_conquer(&mut xs, c, m);
    assert_eq!(xs, q.coefficients_asc());
    if xs.len() > 1 && m > T::ONE {
        // One split, whatever the length, with the halves shifted by the default policy.
        let mut xs = p.coefficients_asc().to_vec();
        mod_taylor_shift_divide_and_conquer_split(&mut xs, c, m);
        assert_eq!(xs, q.coefficients_asc());
    }
    q
}

fn test_mod_taylor_shift_helper<T: PrimitiveUnsigned>() {
    let test = |s, c: u8, m: u8, out| {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let c = T::from(c);
        let m = T::from(m);
        let q = check(&p, c, m);
        assert_eq!(q.to_string(), out);
        assert_eq!(mod_taylor_shift_naive(&p, c, m), q);
    };
    test("0", 0, 1, "0");
    test("0", 0, 5, "0");
    test("0", 3, 5, "0");
    test("5", 0, 7, "5");
    test("5", 3, 7, "5");
    test("x", 3, 5, "x+3");
    test("x^2+3*x+2", 3, 7, "x^2+2*x+6");
    test("x^2+3*x+2", 1, 7, "x^2+5*x+6");
    // Moduli that are not prime: binomial coefficients can vanish.
    test("x^4", 1, 4, "x^4+2*x^2+1");
    test("x^4", 1, 2, "x^4+1");
    test("x^3", 5, 6, "x^3+3*x^2+3*x+5");
    test(
        "5*x^5+4*x^4+3*x^3+2*x^2+x",
        11,
        12,
        "5*x^5+3*x^4+x^3+3*x^2+3*x+9",
    );
    test("x^6+x^3+1", 2, 9, "x^6+3*x^5+6*x^4+8*x^3+3*x^2+6*x+1");
    test(
        "250*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        250,
        251,
        "250*x^7+14*x^6+194*x^5+115*x^4+120*x^3+87*x^2+220*x+5",
    );
    test(
        "254*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        3,
        255,
        "254*x^7+241*x^6+198*x^5+95*x^4+19*x^3+231*x^2+41*x+76",
    );
    test(
        "x^4+25*x^3+50*x^2+100*x+200",
        128,
        255,
        "x^4+27*x^3+89*x^2+233*x+186",
    );
    test(
        "x^8",
        2,
        255,
        "x^8+16*x^7+112*x^6+193*x^5+100*x^4+7*x^3+7*x^2+4*x+1",
    );
}

#[test]
fn test_mod_taylor_shift() {
    apply_fn_to_unsigneds!(test_mod_taylor_shift_helper);
}

fn test_mod_taylor_shift_long_helper<T: PrimitiveUnsigned>() {
    // Polynomials long enough to be split, every coefficient 1, modulo prime, composite, and
    // largest moduli.
    for (len, c, m) in [
        // - c = 1, split
        (600, T::ONE, T::MAX),
        (600, T::ONE, T::from(6u8)),
        // - c not 1, too short to split
        (100, T::from(3u8), T::MAX),
        // - c not 1, split
        (200, T::from(3u8), T::MAX),
        (200, T::from(3u8), T::from(251u8)),
        (200, T::from(5u8), T::from(120u8)),
        (200, T::MAX - T::ONE, T::MAX),
    ] {
        let p = UnsignedPolynomial::<T>::from_coefficients_asc(vec![T::ONE; len]);
        let q = check(&p, c, m);
        assert_eq!(mod_taylor_shift_naive(&p, c, m), q);
    }
}

#[test]
fn test_mod_taylor_shift_long() {
    apply_fn_to_unsigneds!(test_mod_taylor_shift_long_helper);
}

#[test]
#[should_panic]
fn mod_taylor_shift_fail_1() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .mod_taylor_shift(1, 7);
}

#[test]
#[should_panic]
fn mod_taylor_shift_fail_2() {
    // The shift is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_taylor_shift(7, 7);
}

#[test]
#[should_panic]
fn mod_taylor_shift_fail_3() {
    // The modulus is 0.
    (&UnsignedPolynomial::<u8>::from_str("0").unwrap()).mod_taylor_shift(0, 0);
}

#[test]
#[should_panic]
fn mod_taylor_shift_fail_4() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    p.mod_taylor_shift_assign(7, 7);
}

fn mod_taylor_shift_properties_helper_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    c: T,
    m: T,
) {
    let q = check(p, c, m);
    assert_eq!(mod_taylor_shift_naive(p, c, m), q);
    // The degree and the leading coefficient are unchanged.
    assert_eq!(q.len(), p.len());
    assert_eq!(q.leading_coefficient(), p.leading_coefficient());
    // Shifting back by -c recovers p.
    assert_eq!(check(&q, c.mod_neg(m), m), *p);
    // q(0) is p(c).
    assert_eq!(q.coefficient(0), p.mod_evaluate(c, m));
    // Modulo a power of 2, it agrees with mod_power_of_2_taylor_shift.
    if m.is_power_of_2() {
        assert_eq!(
            p.mod_power_of_2_taylor_shift(c, m.significant_bits() - 1),
            q
        );
    }
}

fn mod_taylor_shift_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(p, c, m)| {
        mod_taylor_shift_properties_helper_helper(&p, c, m);
        // Shifting by 0 changes nothing.
        assert_eq!(check(&p, T::ZERO, m), p);
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_12::<T>().test_properties(|(p, c, m)| {
        mod_taylor_shift_properties_helper_helper(&p, c, m);
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_13::<T>().test_properties(|(p, c, m)| {
        mod_taylor_shift_properties_helper_helper(&p, c, m);
    });

    unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_5::<T>().test_properties(
        |(p, a, b, m)| {
            // Shifting by a and then by b is shifting by a + b.
            let q = (&p).mod_taylor_shift(a, m);
            assert_eq!(
                q.mod_taylor_shift(b, m),
                p.mod_taylor_shift(a.mod_add(b, m), m)
            );
        },
    );
}

#[test]
fn mod_taylor_shift_properties() {
    apply_fn_to_unsigneds!(mod_taylor_shift_properties_helper);
}
