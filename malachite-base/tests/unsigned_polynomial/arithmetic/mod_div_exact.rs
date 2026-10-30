// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModMul;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::*;
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_6,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_div_exact::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_div_rem::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::mod_generated_coefficients;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the exact quotient of `p` by `q`, which must divide it, in every way of calling it,
// against the naive version and each algorithm; checks that its product with `q` is `p`; and
// returns it.
fn verify_exact<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    let r = p.mod_div_exact(q, m);
    assert!(r.is_valid());
    assert_eq!(p.clone().mod_div_exact(q.clone(), m), r);
    assert_eq!(p.clone().mod_div_exact(q, m), r);
    assert_eq!(p.mod_div_exact(q.clone(), m), r);
    let mut s = p.clone();
    s.mod_div_exact_assign(q, m);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_div_exact_assign(q.clone(), m);
    assert_eq!(s, r);
    assert_eq!(mod_div_exact_top_wrapper(p, q, m).unwrap(), r);
    assert_eq!(
        mod_div_exact_bidirectional_basecase_wrapper(p, q, m).unwrap(),
        r
    );
    assert_eq!(
        mod_div_exact_bidirectional_newton_wrapper(p, q, m).unwrap(),
        r
    );
    assert_eq!(mod_div_rem_naive(p, q, m).unwrap().0, r);
    assert_eq!(verify_checked(p, q, m), Some(r.clone()));
    assert_eq!(q.mod_mul(&r, m), *p);
    r
}

// Checks the checked exact quotient of `p` by `q` and divisibility in every way of calling them,
// against the quotient and remainder, and returns the checked quotient.
fn verify_checked<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    let r = p.checked_mod_div_exact(q, m);
    assert_eq!(p.clone().checked_mod_div_exact(q.clone(), m), r);
    assert_eq!(p.clone().checked_mod_div_exact(q, m), r);
    assert_eq!(p.checked_mod_div_exact(q.clone(), m), r);
    let (s, t) = p.mod_div_rem(q, m);
    assert_eq!(
        r,
        if t == UnsignedPolynomial::ZERO {
            Some(s)
        } else {
            None
        }
    );
    let divisible = p.mod_divisible_by(q, m);
    assert_eq!(p.clone().mod_divisible_by(q.clone(), m), divisible);
    assert_eq!(p.clone().mod_divisible_by(q, m), divisible);
    assert_eq!(p.mod_divisible_by(q.clone(), m), divisible);
    assert_eq!(divisible, r.is_some());
    r
}

fn test_mod_div_exact_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, m: u8, out| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(verify_exact(&p, &q, T::from(m)).to_string(), out);
    };
    // - modulo 1
    test("0", "0", 1, "0");
    // - the dividend is 0
    test("0", "x+1", 7, "0");
    // - a constant divisor
    test("3*x^2+2*x+6", "3", 7, "x^2+3*x+2");
    // - a monic divisor
    test("x^3+4*x^2+5*x+2", "x+1", 7, "x^2+3*x+2");
    // - a non-monic divisor
    test("x^5+3*x^4+6*x+2", "3*x^2+x+1", 7, "5*x^3+4*x^2+4*x+2");
    // - the divisor's low coefficients are 0
    test("6*x^4+2*x^3", "2*x^3", 7, "3*x+1");
    // - the divisor's lowest coefficient is not invertible
    test("x^4+5*x^3+3*x^2+4*x+2", "x^2+2", 6, "x^2+5*x+1");
    // - a modulus near the top of the word
    test(
        "2*x^5+238*x^3+250*x^2+21*x+3",
        "249*x^3+7*x+1",
        251,
        "250*x^2+3",
    );
}

#[test]
fn test_mod_div_exact() {
    apply_fn_to_unsigneds!(test_mod_div_exact_helper);
}

fn test_checked_mod_div_exact_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, m: u8, out: Option<&str>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(
            verify_checked(&p, &q, T::from(m)).map(|s| s.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - not divisible
    test("x^3+2*x+1", "x^2+3", 7, None);
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", 7, None);
    // - divisible
    test("x^2+3*x+2", "x+1", 7, Some("x+2"));
}

#[test]
fn test_checked_mod_div_exact() {
    apply_fn_to_unsigneds!(test_checked_mod_div_exact_helper);
}

// Every algorithm, at lengths that reach the bidirectional method and the method of Karp and
// Markstein, against the naive version.
fn mod_div_exact_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(q_len, b_len) in &[(10, 5), (100, 50), (25, 41), (3000, 1600), (1600, 3000)] {
        for (m, low) in [(T::from(7u8), 1), (T::from(6u8), 2), (T::MAX, 0)] {
            let qs = mod_generated_coefficients(q_len, m, 1);
            let mut bs = mod_generated_coefficients(b_len, m, 2);
            *bs.last_mut().unwrap() = T::ONE;
            bs[0] = T::from(low);
            let q = UnsignedPolynomial::<T>::from_coefficients_asc(qs);
            let b = UnsignedPolynomial::<T>::from_coefficients_asc(bs);
            let p = (&b).mod_mul(&q, m);
            verify_exact(&p, &b, m);
        }
    }
}

#[test]
fn test_mod_div_exact_algorithms() {
    apply_fn_to_unsigneds!(mod_div_exact_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_div_exact_fail_1() {
    // The divisor is 0.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_div_exact(UnsignedPolynomial::ZERO, 7);
}

#[test]
#[should_panic]
fn mod_div_exact_fail_2() {
    // The divisor's leading coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("2*x^2+x")
        .unwrap()
        .mod_div_exact(UnsignedPolynomial::from_str("2*x+1").unwrap(), 6);
}

#[test]
#[should_panic]
fn mod_div_exact_fail_3() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .mod_div_exact(UnsignedPolynomial::from_str("x+1").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_div_exact_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("2*x^2+x").unwrap();
    p.mod_div_exact_assign(UnsignedPolynomial::from_str("2*x+1").unwrap(), 6);
}

#[test]
#[should_panic]
fn checked_mod_div_exact_fail() {
    // The divisor's leading coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("2*x^2+x")
        .unwrap()
        .checked_mod_div_exact(UnsignedPolynomial::from_str("2*x+1").unwrap(), 6);
}

fn mod_div_exact_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_6::<T>().test_properties(
        |(p, q, m)| {
            verify_exact(&p, &q, m);
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>().test_properties(
        |(p, q, m)| {
            verify_checked(&p, &q, m);
            // The product of the polynomials is divisible by each.
            let pq = (&p).mod_mul(&q, m);
            assert_eq!(verify_exact(&pq, &q, m), p);
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(p, _, m)| {
        // Dividing by 1 leaves the polynomial unchanged.
        assert_eq!((&p).mod_div_exact(UnsignedPolynomial::one() % m, m), p);
        // A polynomial divides itself.
        if p != UnsignedPolynomial::ZERO && p.coefficients_asc().last().unwrap().coprime_with(m) {
            assert_eq!((&p).mod_div_exact(&p, m), UnsignedPolynomial::one() % m);
        }
    });
}

#[test]
fn mod_div_exact_properties() {
    apply_fn_to_unsigneds!(mod_div_exact_properties_helper);
}
