// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAdd, ModIsReduced, ModMul};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::*;
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_div_rem::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::mod_generated_coefficients;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the quotient and remainder of `p` by `q` in every way of calling them, against the naive
// version, each algorithm, and the quotient and remainder alone; checks that they exist exactly
// when the divisor's leading coefficient is invertible, that `q` times the quotient plus the
// remainder is `p`, and that the remainder is shorter than `q`; and returns them.
#[allow(clippy::type_complexity)]
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
    let r = p.checked_mod_div_rem(q, m);
    assert_eq!(p.clone().checked_mod_div_rem(q.clone(), m), r);
    assert_eq!(p.clone().checked_mod_div_rem(q, m), r);
    assert_eq!(p.checked_mod_div_rem(q.clone(), m), r);
    assert_eq!(mod_div_rem_naive(p, q, m), r);
    assert_eq!(mod_div_rem_basecase_wrapper(p, q, m), r);
    assert_eq!(mod_div_rem_newton_wrapper(p, q, m), r);
    assert_eq!(
        p.checked_mod_quotient(q, m),
        r.as_ref().map(|(s, _)| s.clone())
    );
    assert_eq!(p.checked_mod_rem(q, m), r.as_ref().map(|(_, t)| t.clone()));
    assert_eq!(
        r.is_some(),
        m == T::ONE
            || q.coefficients_asc()
                .last()
                .is_some_and(|y| y.coprime_with(m))
    );
    if let Some((s, t)) = &r {
        assert!(s.is_valid());
        assert!(t.is_valid());
        assert!(s.mod_is_reduced(&m));
        assert!(t.mod_is_reduced(&m));
        assert!(*q == UnsignedPolynomial::ZERO || t.len() < q.len());
        assert_eq!(p.mod_div_rem(q, m), (s.clone(), t.clone()));
        assert_eq!(p.clone().mod_div_rem(q.clone(), m), (s.clone(), t.clone()));
        assert_eq!(p.clone().mod_div_rem(q, m), (s.clone(), t.clone()));
        assert_eq!(p.mod_div_rem(q.clone(), m), (s.clone(), t.clone()));
        let mut u = p.clone();
        assert_eq!(u.mod_div_assign_rem(q, m), *t);
        assert_eq!(u, *s);
        let mut u = p.clone();
        assert_eq!(u.mod_div_assign_rem(q.clone(), m), *t);
        assert_eq!(u, *s);
        assert_eq!(q.mod_mul(s, m).mod_add(t, m), *p);
    }
    r
}

fn test_mod_div_rem_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, m: u8, out: Option<(&str, &str)>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, T::from(m)).map(|(s, t)| (s.to_string(), t.to_string())),
            out.map(|(s, t)| (s.to_string(), t.to_string()))
        );
    };
    // - modulo 1
    test("0", "0", 1, Some(("0", "0")));
    // - the divisor is 0
    test("3*x+5", "0", 7, None);
    // - the divisor's leading coefficient is not invertible
    test("x^3+x+1", "2*x+1", 4, None);
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", 7, Some(("0", "5")));
    // - the dividend is 0
    test("0", "x+1", 7, Some(("0", "0")));
    // - a constant divisor
    test("6*x^2+5", "3", 7, Some(("2*x^2+4", "0")));
    // - a monic divisor
    test("x^2+3*x+2", "x+1", 7, Some(("x+2", "0")));
    // - a quotient of one coefficient
    test("3*x+5", "x+1", 7, Some(("3", "2")));
    // - a quotient of two coefficients
    test("x^3+2*x+1", "x^2+3", 7, Some(("x", "6*x+1")));
    // - a non-monic divisor
    test(
        "x^5+3*x^4+2*x+1",
        "3*x^2+x+1",
        7,
        Some(("5*x^3+4*x^2+4*x+2", "3*x+6")),
    );
    // - a composite modulus
    test("4*x^3+2*x+1", "5*x+3", 6, Some(("2*x^2+4", "1")));
    // - a modulus near the top of the word
    test(
        "250*x^4+3*x+1",
        "249*x^2+7",
        251,
        Some(("126*x^2+190", "3*x+177")),
    );
}

#[test]
fn test_mod_div_rem() {
    apply_fn_to_unsigneds!(test_mod_div_rem_helper);
}

// Every algorithm, at lengths that reach the method of Karp and Markstein from the dispatcher,
// against the naive version.
fn mod_div_rem_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(p_len, q_len) in &[(10, 5), (200, 100), (300, 2), (3000, 1500), (3000, 2)] {
        for m in [T::from(7u8), T::MAX >> 1 | T::ONE, T::MAX] {
            let p = UnsignedPolynomial::<T>::from_coefficients_asc(mod_generated_coefficients(
                p_len, m, 1,
            ));
            let mut ys = mod_generated_coefficients(q_len, m, 2);
            *ys.last_mut().unwrap() = T::ONE;
            let q = UnsignedPolynomial::<T>::from_coefficients_asc(ys);
            assert!(verify(&p, &q, m).is_some());
        }
    }
}

#[test]
fn test_mod_div_rem_algorithms() {
    apply_fn_to_unsigneds!(mod_div_rem_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_div_rem_fail_1() {
    // The divisor is 0.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_div_rem(UnsignedPolynomial::ZERO, 7);
}

#[test]
#[should_panic]
fn mod_div_rem_fail_2() {
    // The divisor's leading coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("x^2+1")
        .unwrap()
        .mod_div_rem(UnsignedPolynomial::from_str("2*x+1").unwrap(), 6);
}

#[test]
#[should_panic]
fn mod_div_rem_fail_3() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .mod_div_rem(UnsignedPolynomial::from_str("x+1").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_div_assign_rem_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    p.mod_div_assign_rem(UnsignedPolynomial::from_str("2*x+1").unwrap(), 6);
}

#[test]
#[should_panic]
fn checked_mod_div_rem_fail() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .checked_mod_div_rem(UnsignedPolynomial::from_str("x+1").unwrap(), 7);
}

fn mod_div_rem_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            verify(&p, &q, m);
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>().test_properties(
        |(p, q, m)| {
            let s = verify(&p, &q, m).unwrap().0;
            // Dividing a multiple of the divisor leaves no remainder.
            assert_eq!(
                (&q).mod_mul(&s, m).mod_div_rem(&q, m),
                (s, UnsignedPolynomial::ZERO)
            );
            // Dividing the divisor by itself gives 1.
            assert_eq!(
                (&q).mod_div_rem(&q, m),
                (UnsignedPolynomial::one() % m, UnsignedPolynomial::ZERO)
            );
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(p, _, m)| {
        // Dividing by 1 leaves the polynomial unchanged.
        assert_eq!(
            (&p).mod_div_rem(UnsignedPolynomial::one() % m, m),
            (p, UnsignedPolynomial::ZERO)
        );
    });
}

#[test]
fn mod_div_rem_properties() {
    apply_fn_to_unsigneds!(mod_div_rem_properties_helper);
}
