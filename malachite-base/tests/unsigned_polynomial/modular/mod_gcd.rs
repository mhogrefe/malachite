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
use malachite_base::num::modular::traits::{ModIsReduced, ModMul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{
    CheckedModGcd, ModDivRem, ModGcd, ModGcdAssign, ModMakeMonic, ModRem,
};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_pair_gen_var_4,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10,
};
use malachite_base::test_util::unsigned_polynomial::modular::mod_gcd::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_mul::mod_generated_coefficients;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the GCD of `p` and `q` modulo `m` in every way of calling it, against the naive version,
// and, where they succeed, against Euclid's algorithm and the half-GCD algorithm, which always
// succeed when `m` is prime; checks that it is 0 or monic, and divides both; and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
    prime: bool,
) -> Option<UnsignedPolynomial<T>> {
    let g = p.checked_mod_gcd(q, m);
    assert_eq!(p.clone().checked_mod_gcd(q.clone(), m), g);
    assert_eq!(p.clone().checked_mod_gcd(q, m), g);
    assert_eq!(p.checked_mod_gcd(q.clone(), m), g);
    assert_eq!(mod_gcd_naive(p, q, m), g);
    for alternative in [mod_gcd_euclidean(p, q, m), mod_gcd_hgcd(p, q, m)] {
        if prime {
            assert_eq!(alternative, g);
        } else if let (Some(alternative), Some(g)) = (&alternative, &g) {
            assert_eq!(alternative, g);
        }
    }
    if prime {
        assert!(g.is_some());
        assert_eq!(q.checked_mod_gcd(p, m), g);
    }
    if let Some(g) = &g {
        assert!(g.is_valid());
        assert!(g.mod_is_reduced(&m));
        if *g != UnsignedPolynomial::ZERO {
            assert!(g.is_monic());
            assert_eq!(p.mod_rem(g, m), UnsignedPolynomial::ZERO);
            assert_eq!(q.mod_rem(g, m), UnsignedPolynomial::ZERO);
        }
        assert_eq!(p.mod_gcd(q, m), *g);
        assert_eq!(p.clone().mod_gcd(q.clone(), m), *g);
        assert_eq!(p.clone().mod_gcd(q, m), *g);
        assert_eq!(p.mod_gcd(q.clone(), m), *g);
        let mut u = p.clone();
        u.mod_gcd_assign(q, m);
        assert_eq!(u, *g);
        let mut u = p.clone();
        u.mod_gcd_assign(q.clone(), m);
        assert_eq!(u, *g);
    }
    g
}

fn test_mod_gcd_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, m: u8, out: Option<&str>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        let prime = u64::from(m).is_prime();
        assert_eq!(
            verify(&p, &q, T::from(m), prime).map(|g| g.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - modulo 1
    // - m == T::ONE
    test("0", "0", 1, Some("0"));
    // - both 0
    // - !(xs.len() < ys.len())
    // - xs.is_empty()
    test("0", "0", 7, Some("0"));
    // - the second is 0
    // - !(*x == T::ZERO)
    // - xs.len() < ys.len()
    // - ys.is_empty()
    test("3*x+5", "0", 7, Some("x+4"));
    // - the first is 0
    test("0", "3*x+5", 7, Some("x+4"));
    // - a nonzero constant
    // - ys.len() == 1
    // - !(xs.is_empty()) && !(ys.is_empty())
    // - ys.len() < cutoffs.gcd_hgcd
    test("x^2+1", "3", 7, Some("1"));
    // - coprime
    // - r3.is_empty(), the second
    test("x^2+1", "x+1", 7, Some("1"));
    // - a nontrivial GCD
    // - xs.len() >= ys.len(), the first
    // - xs.last().is_some_and(|x| m.is_zero(x))
    test("x^2+3*x+2", "x^2+4*x+3", 7, Some("x+1"));
    // - one divides the other
    // - r3.is_empty(), the first
    test("x^3+1", "x+1", 7, Some("x+1"));
    // - equal lengths
    test("2*x+4", "3*x+6", 7, Some("x+2"));
    // - modulo 2
    test("x^2+x", "x^2+1", 2, Some("x+1"));
    // - a composite modulus, with only invertible leading coefficients
    test("x^2+5*x", "x+5", 6, Some("x+5"));
    // - a composite modulus and a non-invertible leading coefficient
    test("x^2+1", "2*x+1", 6, None);
    // - a composite modulus and a non-invertible constant remainder
    test("x^2+1", "x+3", 4, None);
    // - a composite modulus and a non-invertible leading coefficient of the second
    test("x+1", "2", 4, None);
    // - a composite modulus and a non-invertible leading coefficient, with the second 0
    test("2*x+1", "0", 4, None);
    // - a modulus near the top of the word
    test("250*x^2+3*x+1", "249*x+7", 251, Some("1"));
}

#[test]
fn test_mod_gcd() {
    apply_fn_to_unsigneds!(test_mod_gcd_helper);
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
// half their length, against the naive version and each algorithm.
// - !(ys.len() < cutoffs.gcd_hgcd)
fn mod_gcd_algorithms_helper<T: PrimitiveUnsigned + IsPrime>() {
    for &(p_len, q_len) in &[(10, 8), (700, 600), (3500, 3200)] {
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
            let g = verify(&p, &q, m, true).unwrap();
            assert_eq!(
                g.mod_rem(&c.mod_make_monic(m).unwrap(), m),
                UnsignedPolynomial::ZERO
            );
        }
    }
}

#[test]
fn test_mod_gcd_algorithms() {
    mod_gcd_algorithms_helper::<u8>();
    mod_gcd_algorithms_helper::<u16>();
    mod_gcd_algorithms_helper::<u32>();
    mod_gcd_algorithms_helper::<u64>();
    mod_gcd_algorithms_helper::<usize>();
}

#[test]
#[should_panic]
fn mod_gcd_fail_1() {
    // A leading coefficient met along the way is not invertible.
    UnsignedPolynomial::<u8>::from_str("x^2+1")
        .unwrap()
        .mod_gcd(UnsignedPolynomial::from_str("2*x+1").unwrap(), 6);
}

#[test]
#[should_panic]
fn mod_gcd_fail_2() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .mod_gcd(UnsignedPolynomial::from_str("x+1").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_gcd_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    p.mod_gcd_assign(UnsignedPolynomial::from_str("2*x+1").unwrap(), 6);
}

#[test]
#[should_panic]
fn checked_mod_gcd_fail() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .checked_mod_gcd(UnsignedPolynomial::from_str("7*x+1").unwrap(), 7);
}

fn mod_gcd_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            verify(&p, &q, m, false);
        },
    );
}

fn mod_gcd_prime_properties_helper<T: PrimitiveUnsigned + IsPrime>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10::<T>().test_properties(
        |(p, q, m)| {
            let g = verify(&p, &q, m, true).unwrap();
            if g != UnsignedPolynomial::ZERO {
                // The cofactors are coprime.
                let p_cofactor = (&p).mod_div_rem(&g, m).0;
                let q_cofactor = (&q).mod_div_rem(&g, m).0;
                assert_eq!(p_cofactor.mod_gcd(q_cofactor, m), UnsignedPolynomial::one());
            }
        },
    );

    unsigned_polynomial_unsigned_pair_gen_var_4::<T>().test_properties(|(p, m)| {
        let monic = if p == UnsignedPolynomial::ZERO {
            p.clone()
        } else {
            (&p).mod_make_monic(m).unwrap()
        };
        assert_eq!((&p).mod_gcd(&p, m), monic);
        assert_eq!((&p).mod_gcd(UnsignedPolynomial::ZERO, m), monic);
        assert_eq!(
            (&p).mod_gcd(UnsignedPolynomial::one(), m),
            UnsignedPolynomial::one()
        );
    });
}

#[test]
fn mod_gcd_properties() {
    apply_fn_to_unsigneds!(mod_gcd_properties_helper);
    mod_gcd_prime_properties_helper::<u8>();
    mod_gcd_prime_properties_helper::<u16>();
    mod_gcd_prime_properties_helper::<u32>();
    mod_gcd_prime_properties_helper::<u64>();
    mod_gcd_prime_properties_helper::<usize>();
}
