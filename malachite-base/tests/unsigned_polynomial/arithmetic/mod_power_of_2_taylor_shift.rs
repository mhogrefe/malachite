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
    ModPowerOf2Evaluate, ModPowerOf2TaylorShift, ModPowerOf2TaylorShiftAssign, Polynomial,
};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_10,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_11,
    unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_4,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_taylor_shift::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_taylor_shift::{
    mod_power_of_2_taylor_shift_divide_and_conquer, mod_power_of_2_taylor_shift_horner,
};

fn check<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, c: T, pow: u64) -> UnsignedPolynomial<T> {
    let q = p.mod_power_of_2_taylor_shift(c, pow);
    assert!(q.is_valid());
    assert!(q.mod_power_of_2_is_reduced(pow));
    assert_eq!(p.clone().mod_power_of_2_taylor_shift(c, pow), q);
    let mut r = p.clone();
    r.mod_power_of_2_taylor_shift_assign(c, pow);
    assert_eq!(r, q);
    let mut xs = p.coefficients_asc().to_vec();
    mod_power_of_2_taylor_shift_horner(&mut xs, c, pow);
    assert_eq!(xs, q.coefficients_asc());
    let mut xs = p.coefficients_asc().to_vec();
    mod_power_of_2_taylor_shift_divide_and_conquer(&mut xs, c, pow);
    assert_eq!(xs, q.coefficients_asc());
    q
}

fn test_mod_power_of_2_taylor_shift_helper<T: PrimitiveUnsigned>() {
    let test = |s, c: u8, pow, out| {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let c = T::from(c);
        let q = check(&p, c, pow);
        assert_eq!(q.to_string(), out);
        assert_eq!(mod_power_of_2_taylor_shift_naive(&p, c, pow), q);
    };
    test("0", 0, 0, "0");
    test("0", 0, 4, "0");
    test("0", 3, 4, "0");
    test("5", 0, 4, "5");
    test("5", 3, 4, "5");
    test("x", 3, 4, "x+3");
    test("x^3+7*x^2+5", 0, 8, "x^3+7*x^2+5");
    test("x^2+3*x+2", 1, 4, "x^2+5*x+6");
    test("x^2+3*x+2", 10, 4, "x^2+7*x+4");
    test("x^3", 1, 1, "x^3+x^2+x+1");
    test("x+3", 2, 2, "x+1");
    test(
        "255*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        255,
        8,
        "255*x^7+14*x^6+199*x^5+115*x^4+125*x^3+87*x^2+225*x+5",
    );
    test(
        "255*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        3,
        8,
        "255*x^7+242*x^6+199*x^5+95*x^4+13*x^3+211*x^2+9*x+57",
    );
    // Binomial coefficients divisible by 2^8 vanish.
    test("x^8", 2, 8, "x^8+16*x^7+112*x^6+192*x^5+96*x^4");
    test(
        "x^4+25*x^3+50*x^2+100*x+200",
        128,
        8,
        "x^4+25*x^3+178*x^2+100*x+200",
    );
}

#[test]
fn test_mod_power_of_2_taylor_shift() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_taylor_shift_helper);
}

fn test_mod_power_of_2_taylor_shift_long_helper<T: PrimitiveUnsigned>() {
    // Polynomials long enough to be split.
    for (len, c, pow) in [
        // - c = 1, split
        (1100, 1, T::WIDTH),
        (1100, 1, 5),
        // - c not 1, too short to split
        (300, 3, T::WIDTH),
        // - c not 1, split
        (700, 3, T::WIDTH),
        (700, 3, 7),
        (700, 6, 8),
    ] {
        let p = UnsignedPolynomial::<T>::from_coefficients_asc(vec![T::ONE; len]);
        let c = T::from(c);
        let q = check(&p, c, pow);
        assert_eq!(mod_power_of_2_taylor_shift_naive(&p, c, pow), q);
    }
}

#[test]
fn test_mod_power_of_2_taylor_shift_long() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_taylor_shift_long_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_taylor_shift_fail_1() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("16*x+1")
        .unwrap()
        .mod_power_of_2_taylor_shift(1, 4);
}

#[test]
#[should_panic]
fn mod_power_of_2_taylor_shift_fail_2() {
    // The shift is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_taylor_shift(16, 4);
}

#[test]
#[should_panic]
fn mod_power_of_2_taylor_shift_fail_3() {
    // The modulus is too large for the coefficient type.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_taylor_shift(1, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_taylor_shift_fail_4() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    p.mod_power_of_2_taylor_shift_assign(16, 4);
}

fn mod_power_of_2_taylor_shift_properties_helper_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    c: T,
    pow: u64,
) {
    let q = check(p, c, pow);
    assert_eq!(mod_power_of_2_taylor_shift_naive(p, c, pow), q);
    // The degree and the leading coefficient are unchanged.
    assert_eq!(q.len(), p.len());
    assert_eq!(q.leading_coefficient(), p.leading_coefficient());
    // Shifting back by -c recovers p.
    assert_eq!(check(&q, c.mod_power_of_2_neg(pow), pow), *p);
    // q(0) is p(c).
    assert_eq!(q.coefficient(0), p.mod_power_of_2_evaluate(c, pow));
}

fn mod_power_of_2_taylor_shift_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, c, pow)| {
        mod_power_of_2_taylor_shift_properties_helper_helper(&p, c, pow);
        // Shifting by 0 changes nothing.
        assert_eq!(check(&p, T::ZERO, pow), p);
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_10::<T>().test_properties(
        |(p, c, pow)| {
            mod_power_of_2_taylor_shift_properties_helper_helper(&p, c, pow);
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_11::<T>().test_properties(
        |(p, c, pow)| {
            mod_power_of_2_taylor_shift_properties_helper_helper(&p, c, pow);
        },
    );

    unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_4::<T>().test_properties(
        |(p, a, b, pow)| {
            // Shifting by a and then by b is shifting by a + b.
            let q = (&p).mod_power_of_2_taylor_shift(a, pow);
            assert_eq!(
                q.mod_power_of_2_taylor_shift(b, pow),
                p.mod_power_of_2_taylor_shift(a.mod_power_of_2_add(b, pow), pow)
            );
        },
    );
}

#[test]
fn mod_power_of_2_taylor_shift_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_taylor_shift_properties_helper);
}
