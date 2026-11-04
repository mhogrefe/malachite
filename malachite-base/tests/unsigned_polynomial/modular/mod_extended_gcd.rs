// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_base::num::modular::traits::{ModAdd, ModIsReduced, ModMul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{
    CheckedModExtendedGcd, CheckedModGcd, ModExtendedGcd,
};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10,
};
use malachite_base::test_util::unsigned_polynomial::modular::mod_extended_gcd::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_mul::mod_generated_coefficients;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the extended GCD of `p` and `q` modulo `m` in every way of calling it, against the naive
// version when the polynomials are short, and, where they succeed, against Euclid's algorithm and
// the half-GCD algorithm, which always succeed when `m` is prime; checks that the GCD is the one
// `checked_mod_gcd` finds, that the cofactors combine `p` and `q` into it, and, when `m` is prime
// and the GCD has a lower degree than both, that the cofactors have the lowest degrees possible;
// and returns them.
#[allow(clippy::type_complexity)]
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
    prime: bool,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    let r = p.checked_mod_extended_gcd(q, m);
    assert_eq!(p.clone().checked_mod_extended_gcd(q.clone(), m), r);
    assert_eq!(p.clone().checked_mod_extended_gcd(q, m), r);
    assert_eq!(p.checked_mod_extended_gcd(q.clone(), m), r);
    if p.len() <= 100 && q.len() <= 100 {
        assert_eq!(mod_extended_gcd_naive(p, q, m), r);
    }
    for alternative in [mod_extended_gcd_euclidean(p, q, m), mod_extended_gcd_hgcd(p, q, m)] {
        if prime {
            assert_eq!(alternative, r);
        } else if let (Some(alternative), Some(r)) = (&alternative, &r) {
            assert_eq!(alternative, r);
        }
    }
    if prime {
        assert!(r.is_some());
        if p.len() != q.len() {
            assert_eq!(
                q.checked_mod_extended_gcd(p, m),
                r.clone().map(|(g, s, t)| (g, t, s))
            );
        }
    }
    if let Some((g, s, t)) = &r {
        for x in [g, s, t] {
            assert!(x.is_valid());
            assert!(x.mod_is_reduced(&m));
        }
        assert_eq!(p.checked_mod_gcd(q, m).as_ref(), Some(g));
        assert_eq!(s.mod_mul(p, m).mod_add(t.mod_mul(q, m), m), *g);
        if prime
            && *p != UnsignedPolynomial::ZERO
            && *q != UnsignedPolynomial::ZERO
            && g.len() < p.len().min(q.len())
        {
            assert!(s.len() + g.len() <= q.len());
            assert!(t.len() + g.len() <= p.len());
        }
        let r = r.clone().unwrap();
        assert_eq!(p.mod_extended_gcd(q, m), r);
        assert_eq!(p.clone().mod_extended_gcd(q.clone(), m), r);
        assert_eq!(p.clone().mod_extended_gcd(q, m), r);
        assert_eq!(p.mod_extended_gcd(q.clone(), m), r);
    }
    r
}

fn test_mod_extended_gcd_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, m: u8, out: Option<(&str, &str, &str)>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        let prime = u64::from(m).is_prime();
        assert_eq!(
            verify(&p, &q, T::from(m), prime).map(|(g, s, t)| (
                g.to_string(),
                s.to_string(),
                t.to_string()
            )),
            out.map(|(g, s, t)| (g.to_string(), s.to_string(), t.to_string()))
        );
    };
    // - modulo 1
    // - m == T::ONE
    test("0", "0", 1, Some(("0", "0", "0")));
    // - both 0
    // - xs.is_empty()
    test("0", "0", 7, Some(("0", "0", "0")));
    // - the second is 0
    // - xs.len() < ys.len()
    // - ys.is_empty()
    test("3*x+5", "0", 7, Some(("x+4", "5", "0")));
    // - the first is 0
    test("0", "3*x+5", 7, Some(("x+4", "0", "5")));
    // - a nonzero constant
    // - ys.len() == 1, the second
    test("x^2+1", "3", 7, Some(("1", "0", "5")));
    // - coprime
    // - !v3.is_empty()
    // - ys.len() < cutoffs.xgcd_hgcd
    test("x^2+1", "x+1", 7, Some(("1", "4", "3*x+4")));
    // - a nontrivial GCD
    test("x^2+3*x+2", "x^2+4*x+3", 7, Some(("x+1", "6", "1")));
    // - one divides the other
    // - r.is_empty()
    test("x^3+1", "x+1", 7, Some(("x+1", "0", "1")));
    // - equal lengths
    test("2*x+4", "3*x+6", 7, Some(("x+2", "0", "5")));
    // - modulo 2
    test("x^2+x", "x^2+1", 2, Some(("x+1", "1", "1")));
    // - a composite modulus, with only invertible leading coefficients
    test("x^2+5*x", "x+5", 6, Some(("x+5", "0", "1")));
    // - a composite modulus and a non-invertible leading coefficient
    test("x^2+1", "2*x+1", 6, None);
    // - a composite modulus and a non-invertible constant remainder
    test("x^2+1", "x+3", 4, None);
    // - a composite modulus and a non-invertible leading coefficient of the second
    test("x+1", "2", 4, None);
    // - a composite modulus and a non-invertible leading coefficient, with the second 0
    test("2*x+1", "0", 4, None);
    // - a modulus near the top of the word
    test(
        "250*x^2+3*x+1",
        "249*x+7",
        251,
        Some(("1", "166", "168*x+84")),
    );
    // - deg g < min deg with nontrivial cofactors
    test("x^3+2*x+1", "x^2+3", 7, Some(("1", "2*x+2", "5*x^2+5*x+2")));
}

#[test]
fn test_mod_extended_gcd() {
    apply_fn_to_unsigneds!(test_mod_extended_gcd_helper);
}

// The largest prime that is a `T`.
fn largest_prime<T: PrimitiveUnsigned + IsPrime>() -> T {
    let mut p = T::MAX;
    while !p.is_prime() {
        p -= T::ONE;
    }
    p
}

// Polynomials of lengths that reach the half-GCD algorithm from the dispatcher, sharing a factor of
// half their length, against each algorithm.
// - !(ys.len() < cutoffs.xgcd_hgcd)
fn mod_extended_gcd_algorithms_helper<T: PrimitiveUnsigned + IsPrime>() {
    for &(p_len, q_len) in &[(10, 8), (300, 250), (1200, 1000)] {
        for m in [T::from(7u8), largest_prime::<T>()] {
            let c = UnsignedPolynomial::<T>::from_coefficients_asc(mod_generated_coefficients(
                q_len >> 1,
                m,
                1,
            ));
            let p = UnsignedPolynomial::<T>::from_coefficients_asc(mod_generated_coefficients(
                p_len - (q_len >> 1),
                m,
                2,
            ))
            .mod_mul(&c, m);
            let q = UnsignedPolynomial::<T>::from_coefficients_asc(mod_generated_coefficients(
                q_len - (q_len >> 1),
                m,
                3,
            ))
            .mod_mul(&c, m);
            verify(&p, &q, m, true);
        }
    }
}

#[test]
fn test_mod_extended_gcd_algorithms() {
    mod_extended_gcd_algorithms_helper::<u8>();
    mod_extended_gcd_algorithms_helper::<u16>();
    mod_extended_gcd_algorithms_helper::<u32>();
    mod_extended_gcd_algorithms_helper::<u64>();
    mod_extended_gcd_algorithms_helper::<usize>();
}

#[test]
#[should_panic]
fn mod_extended_gcd_fail_1() {
    // A leading coefficient met along the way is not invertible.
    UnsignedPolynomial::<u8>::from_str("x^2+1")
        .unwrap()
        .mod_extended_gcd(UnsignedPolynomial::from_str("2*x+1").unwrap(), 6);
}

#[test]
#[should_panic]
fn mod_extended_gcd_fail_2() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .mod_extended_gcd(UnsignedPolynomial::from_str("x+1").unwrap(), 7);
}

#[test]
#[should_panic]
fn checked_mod_extended_gcd_fail() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .checked_mod_extended_gcd(UnsignedPolynomial::from_str("7*x+1").unwrap(), 7);
}

fn mod_extended_gcd_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            verify(&p, &q, m, false);
        },
    );
}

fn mod_extended_gcd_prime_properties_helper<T: PrimitiveUnsigned + IsPrime>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10::<T>().test_properties(
        |(p, q, m)| {
            verify(&p, &q, m, true);
        },
    );
}

#[test]
fn mod_extended_gcd_properties() {
    apply_fn_to_unsigneds!(mod_extended_gcd_properties_helper);
    mod_extended_gcd_prime_properties_helper::<u8>();
    mod_extended_gcd_prime_properties_helper::<u16>();
    mod_extended_gcd_prime_properties_helper::<u32>();
    mod_extended_gcd_prime_properties_helper::<u64>();
    mod_extended_gcd_prime_properties_helper::<usize>();
}
