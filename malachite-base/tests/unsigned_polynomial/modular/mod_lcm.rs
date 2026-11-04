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
    CheckedModLcm, ModGcd, ModLcm, ModLcmAssign, ModMakeMonic, ModRem,
};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_pair_gen_var_4,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10,
};
use malachite_base::test_util::unsigned_polynomial::modular::mod_lcm::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the LCM of `p` and `q` modulo `m` in every way of calling it, against the naive version;
// checks that it is 0 or monic, and a multiple of both, and, when `m` is prime, that it always
// exists and its product with the GCD is the monic multiple of `p * q`; and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
    prime: bool,
) -> Option<UnsignedPolynomial<T>> {
    let l = p.checked_mod_lcm(q, m);
    assert_eq!(p.clone().checked_mod_lcm(q.clone(), m), l);
    assert_eq!(p.clone().checked_mod_lcm(q, m), l);
    assert_eq!(p.checked_mod_lcm(q.clone(), m), l);
    assert_eq!(mod_lcm_naive(p, q, m), l);
    if prime {
        assert!(l.is_some());
        assert_eq!(q.checked_mod_lcm(p, m), l);
    }
    if let Some(l) = &l {
        assert!(l.is_valid());
        assert!(l.mod_is_reduced(&m));
        if *l != UnsignedPolynomial::ZERO {
            assert!(l.is_monic());
            assert_eq!(l.mod_rem(p, m), UnsignedPolynomial::ZERO);
            assert_eq!(l.mod_rem(q, m), UnsignedPolynomial::ZERO);
            if prime {
                assert_eq!(
                    l.mod_mul(p.mod_gcd(q, m), m),
                    p.mod_mul(q, m).mod_make_monic(m).unwrap()
                );
            }
        }
        assert_eq!(p.mod_lcm(q, m), *l);
        assert_eq!(p.clone().mod_lcm(q.clone(), m), *l);
        assert_eq!(p.clone().mod_lcm(q, m), *l);
        assert_eq!(p.mod_lcm(q.clone(), m), *l);
        let mut u = p.clone();
        u.mod_lcm_assign(q, m);
        assert_eq!(u, *l);
        let mut u = p.clone();
        u.mod_lcm_assign(q.clone(), m);
        assert_eq!(u, *l);
    }
    l
}

fn test_mod_lcm_helper<T: PrimitiveUnsigned>() {
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
    // - the first is 0
    // - xs.is_empty() || ys.is_empty()
    test("0", "3*x+5", 7, Some("0"));
    // - the second is 0
    test("3*x+5", "0", 7, Some("0"));
    // - constants
    test("3", "5", 7, Some("1"));
    // - coprime
    test("x^2+1", "x+1", 7, Some("x^3+x^2+x+1"));
    // - a nontrivial GCD
    test("x^2+3*x+2", "x^2+4*x+3", 7, Some("x^3+6*x^2+4*x+6"));
    // - one divides the other
    test("x^3+1", "x+1", 7, Some("x^3+1"));
    // - a composite modulus, with only invertible leading coefficients
    test("x^2+5*x", "x+5", 6, Some("x^2+5*x"));
    // - a composite modulus and a non-invertible product of leading coefficients
    // - *x == T::ZERO
    test("2*x+1", "3*x+1", 6, None);
    // - a composite modulus and a non-invertible leading coefficient met along the way
    test("x^2+1", "x+3", 4, None);
}

#[test]
fn test_mod_lcm() {
    apply_fn_to_unsigneds!(test_mod_lcm_helper);
}

#[test]
#[should_panic]
fn mod_lcm_fail_1() {
    // The product of the leading coefficients is not invertible.
    UnsignedPolynomial::<u8>::from_str("2*x+1")
        .unwrap()
        .mod_lcm(UnsignedPolynomial::from_str("3*x+1").unwrap(), 6);
}

#[test]
#[should_panic]
fn mod_lcm_fail_2() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .mod_lcm(UnsignedPolynomial::from_str("x+1").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_lcm_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    p.mod_lcm_assign(UnsignedPolynomial::from_str("3*x+1").unwrap(), 6);
}

#[test]
#[should_panic]
fn checked_mod_lcm_fail() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .checked_mod_lcm(UnsignedPolynomial::from_str("7*x+1").unwrap(), 7);
}

fn mod_lcm_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            verify(&p, &q, m, false);
        },
    );
}

fn mod_lcm_prime_properties_helper<T: PrimitiveUnsigned + IsPrime>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10::<T>().test_properties(
        |(p, q, m)| {
            verify(&p, &q, m, true);
        },
    );

    unsigned_polynomial_unsigned_pair_gen_var_4::<T>().test_properties(|(p, m)| {
        let monic = if p == UnsignedPolynomial::ZERO {
            p.clone()
        } else {
            (&p).mod_make_monic(m).unwrap()
        };
        assert_eq!((&p).mod_lcm(&p, m), monic);
        assert_eq!(
            (&p).mod_lcm(UnsignedPolynomial::ZERO, m),
            UnsignedPolynomial::ZERO
        );
        assert_eq!((&p).mod_lcm(UnsignedPolynomial::one(), m), monic);
    });
}

#[test]
fn mod_lcm_properties() {
    apply_fn_to_unsigneds!(mod_lcm_properties_helper);
    mod_lcm_prime_properties_helper::<u8>();
    mod_lcm_prime_properties_helper::<u16>();
    mod_lcm_prime_properties_helper::<u32>();
    mod_lcm_prime_properties_helper::<u64>();
    mod_lcm_prime_properties_helper::<usize>();
}
