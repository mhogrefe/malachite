// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Mul,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::*;
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_div_rem::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the quotient and remainder of `p` by `q` in every way of calling them, against the naive
// version, each algorithm, and the quotient and remainder alone; checks that they exist exactly
// when the divisor's leading coefficient is invertible, that `q` times the quotient plus the
// remainder is `p`, and that the remainder is shorter than `q`; and returns them.
#[allow(clippy::type_complexity)]
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
    let r = p.checked_mod_power_of_2_div_rem(q, pow);
    assert_eq!(p.clone().checked_mod_power_of_2_div_rem(q.clone(), pow), r);
    assert_eq!(p.clone().checked_mod_power_of_2_div_rem(q, pow), r);
    assert_eq!(p.checked_mod_power_of_2_div_rem(q.clone(), pow), r);
    assert_eq!(mod_power_of_2_div_rem_naive(p, q, pow), r);
    assert_eq!(mod_power_of_2_div_rem_basecase_wrapper(p, q, pow), r);
    assert_eq!(mod_power_of_2_div_rem_newton_wrapper(p, q, pow), r);
    assert_eq!(
        p.checked_mod_power_of_2_quotient(q, pow),
        r.as_ref().map(|(s, _)| s.clone())
    );
    assert_eq!(
        p.checked_mod_power_of_2_rem(q, pow),
        r.as_ref().map(|(_, t)| t.clone())
    );
    assert_eq!(
        r.is_some(),
        pow == 0 || q.coefficients_asc().last().is_some_and(|&y| y.odd())
    );
    if let Some((s, t)) = &r {
        assert!(s.is_valid());
        assert!(t.is_valid());
        assert!(s.mod_power_of_2_is_reduced(pow));
        assert!(t.mod_power_of_2_is_reduced(pow));
        assert!(*q == UnsignedPolynomial::ZERO || t.len() < q.len());
        assert_eq!(p.mod_power_of_2_div_rem(q, pow), (s.clone(), t.clone()));
        assert_eq!(
            p.clone().mod_power_of_2_div_rem(q.clone(), pow),
            (s.clone(), t.clone())
        );
        assert_eq!(
            p.clone().mod_power_of_2_div_rem(q, pow),
            (s.clone(), t.clone())
        );
        assert_eq!(
            p.mod_power_of_2_div_rem(q.clone(), pow),
            (s.clone(), t.clone())
        );
        let mut u = p.clone();
        assert_eq!(u.mod_power_of_2_div_assign_rem(q, pow), *t);
        assert_eq!(u, *s);
        let mut u = p.clone();
        assert_eq!(u.mod_power_of_2_div_assign_rem(q.clone(), pow), *t);
        assert_eq!(u, *s);
        assert_eq!(q.mod_power_of_2_mul(s, pow).mod_power_of_2_add(t, pow), *p);
    }
    r
}

fn test_mod_power_of_2_div_rem_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, pow: u64, out: Option<(&str, &str)>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, pow).map(|(s, t)| (s.to_string(), t.to_string())),
            out.map(|(s, t)| (s.to_string(), t.to_string()))
        );
    };
    // - modulo 1
    test("0", "0", 0, Some(("0", "0")));
    // - the divisor is 0
    test("3*x+1", "0", 3, None);
    // - the divisor's leading coefficient is even
    test("x^3+2*x^2+3", "2*x+1", 2, None);
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", 3, Some(("0", "5")));
    // - the dividend is 0
    test("0", "x+1", 3, Some(("0", "0")));
    // - a constant divisor
    test("6*x^2+5", "3", 3, Some(("2*x^2+7", "0")));
    // - a quotient of two coefficients
    test("3*x^2+x+1", "3*x+1", 3, Some(("x", "1")));
    // - a non-monic divisor
    test("x^4+7*x+5", "5*x^2+2", 3, Some(("5*x^2+6", "7*x+1")));
    // - the full width
    test("255*x^3+1", "253*x^2+3", 8, Some(("171*x", "255*x+1")));
}

#[test]
fn test_mod_power_of_2_div_rem() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_div_rem_helper);
}

// Every algorithm, at lengths that reach the method of Karp and Markstein from the dispatcher,
// against the naive version.
fn mod_power_of_2_div_rem_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(p_len, q_len) in &[(10, 5), (200, 100), (300, 2), (3000, 1500), (3000, 2)] {
        for pow in [1, 3, T::WIDTH >> 1, T::WIDTH] {
            let p = UnsignedPolynomial::<T>::from_coefficients_asc(
                mod_power_of_2_generated_coefficients(p_len, pow, 1),
            );
            let mut ys = mod_power_of_2_generated_coefficients(q_len, pow, 2);
            *ys.last_mut().unwrap() = T::ONE;
            let q = UnsignedPolynomial::<T>::from_coefficients_asc(ys);
            assert!(verify(&p, &q, pow).is_some());
        }
    }
}

#[test]
fn test_mod_power_of_2_div_rem_algorithms() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_rem_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_rem_fail_1() {
    // The divisor is 0.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_div_rem(UnsignedPolynomial::ZERO, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_rem_fail_2() {
    // The divisor's leading coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("x^2+1")
        .unwrap()
        .mod_power_of_2_div_rem(UnsignedPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_rem_fail_3() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_div_rem(UnsignedPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_assign_rem_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    p.mod_power_of_2_div_assign_rem(UnsignedPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_div_rem_fail() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .checked_mod_power_of_2_div_rem(UnsignedPolynomial::from_str("x+1").unwrap(), 3);
}

fn mod_power_of_2_div_rem_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>().test_properties(
        |(p, q, pow)| {
            let s = verify(&p, &q, pow).unwrap().0;
            // Dividing a multiple of the divisor leaves no remainder.
            assert_eq!(
                (&q).mod_power_of_2_mul(&s, pow)
                    .mod_power_of_2_div_rem(&q, pow),
                (s, UnsignedPolynomial::ZERO)
            );
            // Dividing the divisor by itself gives 1.
            assert_eq!(
                (&q).mod_power_of_2_div_rem(&q, pow),
                (
                    UnsignedPolynomial::one().mod_power_of_2(pow),
                    UnsignedPolynomial::ZERO
                )
            );
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, _, pow)| {
        // Dividing by 1 leaves the polynomial unchanged.
        assert_eq!(
            (&p).mod_power_of_2_div_rem(UnsignedPolynomial::one().mod_power_of_2(pow), pow),
            (p, UnsignedPolynomial::ZERO)
        );
    });
}

#[test]
fn mod_power_of_2_div_rem_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_rem_properties_helper);
}
