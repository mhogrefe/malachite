// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::modular::traits::{ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Mul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{
    CheckedModPowerOf2ExtendedGcd, CheckedModPowerOf2Gcd,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_extended_gcd::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the extended GCD of `p` and `q` modulo $2^k$ in every way of calling it, against the naive
// version when the polynomials are short, and, where they succeed, against Euclid's algorithm and
// the half-GCD algorithm, which always succeed when $k$ is 1; checks that the GCD is the one
// `checked_mod_power_of_2_gcd` finds, that the cofactors combine `p` and `q` into it, and, when the
// GCD has a lower degree than both, that the cofactors have the lowest degrees possible; and
// returns them.
#[allow(clippy::type_complexity)]
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    let r = p.checked_mod_power_of_2_extended_gcd(q, pow);
    assert_eq!(
        p.clone()
            .checked_mod_power_of_2_extended_gcd(q.clone(), pow),
        r
    );
    assert_eq!(p.clone().checked_mod_power_of_2_extended_gcd(q, pow), r);
    assert_eq!(p.checked_mod_power_of_2_extended_gcd(q.clone(), pow), r);
    if p.len() <= 100 && q.len() <= 100 {
        assert_eq!(mod_power_of_2_extended_gcd_naive(p, q, pow), r);
    }
    for alternative in [
        mod_power_of_2_extended_gcd_euclidean(p, q, pow),
        mod_power_of_2_extended_gcd_hgcd(p, q, pow),
    ] {
        if pow == 1 {
            assert_eq!(alternative, r);
        } else if let (Some(alternative), Some(r)) = (&alternative, &r) {
            assert_eq!(alternative, r);
        }
    }
    if pow == 1 {
        assert!(r.is_some());
    }
    if let Some((g, s, t)) = &r {
        for x in [g, s, t] {
            assert!(x.is_valid());
            assert!(x.mod_power_of_2_is_reduced(pow));
        }
        assert_eq!(p.checked_mod_power_of_2_gcd(q, pow).as_ref(), Some(g));
        assert_eq!(
            s.mod_power_of_2_mul(p, pow)
                .mod_power_of_2_add(t.mod_power_of_2_mul(q, pow), pow),
            *g
        );
        if g.len() < p.len().min(q.len()) {
            assert!(s.len() + g.len() <= q.len());
            assert!(t.len() + g.len() <= p.len());
        }
    }
    r
}

fn test_checked_mod_power_of_2_extended_gcd_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, pow: u64, out: Option<(&str, &str, &str)>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, pow).map(|(g, s, t)| (g.to_string(), s.to_string(), t.to_string())),
            out.map(|(g, s, t)| (g.to_string(), s.to_string(), t.to_string()))
        );
    };
    // - modulo 1
    // - pow == 0
    test("0", "0", 0, Some(("0", "0", "0")));
    // - both 0
    test("0", "0", 3, Some(("0", "0", "0")));
    // - the second is 0
    test("3*x+5", "0", 3, Some(("x+7", "3", "0")));
    // - an even leading coefficient, with the second 0
    test("2*x+1", "0", 3, None);
    // - the first is 0
    test("0", "x+5", 3, Some(("x+5", "0", "1")));
    // - an odd constant
    test("x^2+1", "3", 3, Some(("1", "0", "3")));
    // - an even constant
    test("x^2+1", "2", 3, None);
    // - modulo 2
    test("x^2+x", "x^2+1", 1, Some(("x+1", "1", "1")));
    // - a nontrivial GCD
    test("x^3+x^2+3*x+3", "x^2+3*x+2", 3, Some(("x+1", "7", "x+6")));
    // - an even leading coefficient met along the way
    test("x^2+1", "x+3", 2, None);
    // - one divides the other
    test("x^2+4*x+3", "x+1", 4, Some(("x+1", "0", "1")));
    // - the full width of a byte
    test("x^2+255", "x+1", 8, Some(("x+1", "0", "1")));
}

#[test]
fn test_checked_mod_power_of_2_extended_gcd() {
    apply_fn_to_unsigneds!(test_checked_mod_power_of_2_extended_gcd_helper);
}

// Polynomials modulo 2 of lengths that reach the half-GCD algorithm from the dispatcher, sharing a
// factor of half their length, against each algorithm.
fn checked_mod_power_of_2_extended_gcd_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(p_len, q_len) in &[(10, 8), (300, 250), (1200, 1000)] {
        let c = UnsignedPolynomial::<T>::from_coefficients_asc(
            mod_power_of_2_generated_coefficients(q_len >> 1, 1, 1),
        );
        let p = UnsignedPolynomial::<T>::from_coefficients_asc(
            mod_power_of_2_generated_coefficients(p_len - (q_len >> 1), 1, 2),
        )
        .mod_power_of_2_mul(&c, 1);
        let q = UnsignedPolynomial::<T>::from_coefficients_asc(
            mod_power_of_2_generated_coefficients(q_len - (q_len >> 1), 1, 3),
        )
        .mod_power_of_2_mul(&c, 1);
        verify(&p, &q, 1);
    }
}

#[test]
fn test_checked_mod_power_of_2_extended_gcd_algorithms() {
    apply_fn_to_unsigneds!(checked_mod_power_of_2_extended_gcd_algorithms_helper);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_extended_gcd_fail_1() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .checked_mod_power_of_2_extended_gcd(UnsignedPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_extended_gcd_fail_2() {
    // The power is too large.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .checked_mod_power_of_2_extended_gcd(UnsignedPolynomial::from_str("x+1").unwrap(), 9);
}

fn checked_mod_power_of_2_extended_gcd_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );
}

#[test]
fn checked_mod_power_of_2_extended_gcd_properties() {
    apply_fn_to_unsigneds!(checked_mod_power_of_2_extended_gcd_properties_helper);
}
