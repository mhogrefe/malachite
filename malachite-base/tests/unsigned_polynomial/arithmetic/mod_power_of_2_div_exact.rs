// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Mul};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::*;
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_7,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_div_exact::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_div_rem::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the exact quotient of `p` by `q`, which must divide it, in every way of calling it,
// against the naive version and each algorithm; checks that its product with `q` is `p`; and
// returns it.
fn verify_exact<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    let r = p.mod_power_of_2_div_exact(q, pow);
    assert!(r.is_valid());
    assert_eq!(p.clone().mod_power_of_2_div_exact(q.clone(), pow), r);
    assert_eq!(p.clone().mod_power_of_2_div_exact(q, pow), r);
    assert_eq!(p.mod_power_of_2_div_exact(q.clone(), pow), r);
    let mut s = p.clone();
    s.mod_power_of_2_div_exact_assign(q, pow);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_power_of_2_div_exact_assign(q.clone(), pow);
    assert_eq!(s, r);
    assert_eq!(mod_power_of_2_div_exact_top_wrapper(p, q, pow).unwrap(), r);
    assert_eq!(
        mod_power_of_2_div_exact_bidirectional_basecase_wrapper(p, q, pow).unwrap(),
        r
    );
    assert_eq!(
        mod_power_of_2_div_exact_bidirectional_newton_wrapper(p, q, pow).unwrap(),
        r
    );
    assert_eq!(mod_power_of_2_div_rem_naive(p, q, pow).unwrap().0, r);
    assert_eq!(verify_checked(p, q, pow), Some(r.clone()));
    assert_eq!(q.mod_power_of_2_mul(&r, pow), *p);
    r
}

// Checks the checked exact quotient of `p` by `q` and divisibility in every way of calling them,
// against the quotient and remainder, and returns the checked quotient.
fn verify_checked<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    let r = p.checked_mod_power_of_2_div_exact(q, pow);
    assert_eq!(
        p.clone().checked_mod_power_of_2_div_exact(q.clone(), pow),
        r
    );
    assert_eq!(p.clone().checked_mod_power_of_2_div_exact(q, pow), r);
    assert_eq!(p.checked_mod_power_of_2_div_exact(q.clone(), pow), r);
    let (s, t) = p.mod_power_of_2_div_rem(q, pow);
    assert_eq!(
        r,
        if t == UnsignedPolynomial::ZERO {
            Some(s)
        } else {
            None
        }
    );
    let divisible = p.mod_power_of_2_divisible_by(q, pow);
    assert_eq!(
        p.clone().mod_power_of_2_divisible_by(q.clone(), pow),
        divisible
    );
    assert_eq!(p.clone().mod_power_of_2_divisible_by(q, pow), divisible);
    assert_eq!(p.mod_power_of_2_divisible_by(q.clone(), pow), divisible);
    assert_eq!(divisible, r.is_some());
    r
}

fn test_mod_power_of_2_div_exact_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, pow: u64, out| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(verify_exact(&p, &q, pow).to_string(), out);
    };
    // - modulo 1
    test("0", "0", 0, "0");
    // - the dividend is 0
    test("0", "x+1", 3, "0");
    // - a constant divisor
    test("3*x^2+x+6", "3", 3, "x^2+3*x+2");
    // - a non-monic divisor
    test("x^4+4", "5*x^2+2", 3, "5*x^2+6");
    // - the divisor's low coefficients are 0
    test("x^4+3*x^3", "3*x^3", 3, "3*x+1");
    // - the divisor's lowest coefficient is even
    test("x^4+5*x^3+3*x^2+2*x+2", "x^2+2", 3, "x^2+5*x+1");
    // - the full width
    test("255*x^3+241*x^2+x+15", "253*x^2+3", 8, "171*x+5");
}

#[test]
fn test_mod_power_of_2_div_exact() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_div_exact_helper);
}

fn test_checked_mod_power_of_2_div_exact_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, pow: u64, out: Option<&str>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(
            verify_checked(&p, &q, pow).map(|s| s.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - not divisible
    test("x^4+7*x+5", "5*x^2+2", 3, None);
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", 3, None);
    // - divisible
    test("x^2+3*x+2", "x+1", 3, Some("x+2"));
}

#[test]
fn test_checked_mod_power_of_2_div_exact() {
    apply_fn_to_unsigneds!(test_checked_mod_power_of_2_div_exact_helper);
}

// Every algorithm, at lengths that reach the bidirectional method and the method of Karp and
// Markstein, against the naive version.
fn mod_power_of_2_div_exact_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(q_len, b_len) in &[(10, 5), (100, 50), (25, 41), (3000, 1600), (1600, 3000)] {
        for (pow, low) in [(3, 1), (T::WIDTH, 2), (T::WIDTH, 0)] {
            let qs = mod_power_of_2_generated_coefficients(q_len, pow, 1);
            let mut bs = mod_power_of_2_generated_coefficients(b_len, pow, 2);
            *bs.last_mut().unwrap() = T::ONE;
            bs[0] = T::from(low);
            let q = UnsignedPolynomial::<T>::from_coefficients_asc(qs);
            let b = UnsignedPolynomial::<T>::from_coefficients_asc(bs);
            let p = (&b).mod_power_of_2_mul(&q, pow);
            verify_exact(&p, &b, pow);
        }
    }
}

#[test]
fn test_mod_power_of_2_div_exact_algorithms() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_exact_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_exact_fail_1() {
    // The divisor is 0.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_div_exact(UnsignedPolynomial::ZERO, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_exact_fail_2() {
    // The divisor's leading coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("2*x^2+x")
        .unwrap()
        .mod_power_of_2_div_exact(UnsignedPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_exact_fail_3() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_div_exact(UnsignedPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_exact_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("2*x^2+x").unwrap();
    p.mod_power_of_2_div_exact_assign(UnsignedPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_div_exact_fail() {
    // The divisor's leading coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("2*x^2+x")
        .unwrap()
        .checked_mod_power_of_2_div_exact(UnsignedPolynomial::from_str("2*x+1").unwrap(), 3);
}

fn mod_power_of_2_div_exact_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_7::<T>().test_properties(
        |(p, q, pow)| {
            verify_exact(&p, &q, pow);
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>().test_properties(
        |(p, q, pow)| {
            verify_checked(&p, &q, pow);
            // The product of the polynomials is divisible by each.
            let pq = (&p).mod_power_of_2_mul(&q, pow);
            assert_eq!(verify_exact(&pq, &q, pow), p);
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, _, pow)| {
        // Dividing by 1 leaves the polynomial unchanged.
        assert_eq!(
            (&p).mod_power_of_2_div_exact(UnsignedPolynomial::one().mod_power_of_2(pow), pow),
            p
        );
        // A polynomial divides itself.
        if p != UnsignedPolynomial::ZERO && p.coefficients_asc().last().unwrap().odd() {
            assert_eq!(
                (&p).mod_power_of_2_div_exact(&p, pow),
                UnsignedPolynomial::one().mod_power_of_2(pow)
            );
        }
    });
}

#[test]
fn mod_power_of_2_div_exact_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_exact_properties_helper);
}
