// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::{ModAdd, ModIsReduced, ModMul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{
    CheckedModExtendedGcd, CheckedModGcd, ModExtendedGcd,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1,
    natural_polynomial_natural_polynomial_natural_triple_gen_var_5,
};
use malachite_nz::test_util::natural_polynomial::modular::mod_extended_gcd::*;

// The polynomial with the same coefficients as `p`, which must fit in limbs.
fn to_limbs(p: &NaturalPolynomial) -> UnsignedPolynomial<Limb> {
    UnsignedPolynomial::from_coefficients_asc(
        p.coefficients_asc()
            .iter()
            .map(Limb::wrapping_from)
            .collect(),
    )
}

// Whether `m` is prime, as far as the tests can tell: moduli of more than one limb used by the unit
// tests are always prime or have small factors.
fn probably_prime(m: &Natural) -> bool {
    if m.significant_bits() <= Limb::WIDTH {
        Limb::wrapping_from(m).is_prime()
    } else {
        (2u32..1000).all(|d| (m % Natural::from(d)) != 0u32)
    }
}

// `len` coefficients reduced modulo `m`, all large and different, and different for each `seed`:
// the reduced fifth powers of `seed * 7919 + i + 1`; the last is never 0.
fn generated(len: usize, m: &Natural, seed: u64) -> Vec<Natural> {
    let mut xs: Vec<Natural> = (0..len)
        .map(|i| Natural::from(seed * 7919 + u64::wrapping_from(i) + 1).pow(5) % m)
        .collect();
    if let Some(x) = xs.last_mut()
        && *x == 0u32
    {
        *x = Natural::ONE;
    }
    xs
}

// Polynomials of lengths `p_len` and `q_len` modulo `m`, sharing a factor of half the second
// length, together with that factor.
fn shared_factor_pair(
    p_len: usize,
    q_len: usize,
    m: &Natural,
) -> (NaturalPolynomial, NaturalPolynomial, NaturalPolynomial) {
    let c = NaturalPolynomial::from_coefficients_asc(generated(q_len >> 1, m, 1));
    let p = NaturalPolynomial::from_coefficients_asc(generated(p_len - (q_len >> 1), m, 2))
        .mod_mul(&c, m);
    let q = NaturalPolynomial::from_coefficients_asc(generated(q_len - (q_len >> 1), m, 3))
        .mod_mul(&c, m);
    (p, q, c)
}

// Checks the extended GCD of `p` and `q` modulo `m` in every way of calling it, against Euclid's
// algorithm and the half-GCD algorithm where they succeed, and against the word-sized version when
// `m` fits in a limb; checks that the GCD is the one `checked_mod_gcd` finds, that the cofactors
// combine `p` and `q` into it, and, when `m` is prime and the GCD has a lower degree than both,
// that the cofactors have the lowest degrees possible; and returns them.
#[allow(clippy::type_complexity)]
fn verify(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
    prime: bool,
) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
    let r = p.checked_mod_extended_gcd(q, m);
    assert_eq!(p.clone().checked_mod_extended_gcd(q.clone(), m), r);
    assert_eq!(p.clone().checked_mod_extended_gcd(q, m), r);
    assert_eq!(p.checked_mod_extended_gcd(q.clone(), m), r);
    assert_eq!(p.checked_mod_extended_gcd(q, m.clone()), r);
    for alternative in [mod_extended_gcd_euclidean(p, q, m), mod_extended_gcd_hgcd(p, q, m)] {
        if prime {
            assert_eq!(alternative, r);
        } else if let (Some(alternative), Some(r)) = (&alternative, &r) {
            assert_eq!(alternative, r);
        }
    }
    if m.significant_bits() <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).checked_mod_extended_gcd(to_limbs(q), Limb::wrapping_from(m)),
            r.as_ref()
                .map(|(g, s, t)| (to_limbs(g), to_limbs(s), to_limbs(t)))
        );
    }
    if prime {
        assert!(r.is_some());
    }
    if let Some((g, s, t)) = &r {
        for x in [g, s, t] {
            assert!(x.is_valid());
            assert!(x.mod_is_reduced(m));
        }
        assert_eq!(p.checked_mod_gcd(q, m).as_ref(), Some(g));
        assert_eq!(s.mod_mul(p, m).mod_add(t.mod_mul(q, m), m), *g);
        if prime
            && *p != NaturalPolynomial::ZERO
            && *q != NaturalPolynomial::ZERO
            && g.len() < p.len().min(q.len())
        {
            assert!(s.len() + g.len() <= q.len());
            assert!(t.len() + g.len() <= p.len());
        }
        let r = r.clone().unwrap();
        assert_eq!(p.mod_extended_gcd(q, m), r);
        assert_eq!(p.clone().mod_extended_gcd(q.clone(), m.clone()), r);
    }
    r
}

#[test]
fn test_mod_extended_gcd() {
    let test = |p, q, m, out: Option<(&str, &str, &str)>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        let prime = probably_prime(&m);
        assert_eq!(
            verify(&p, &q, &m, prime).map(|(g, s, t)| (
                g.to_string(),
                s.to_string(),
                t.to_string()
            )),
            out.map(|(g, s, t)| (g.to_string(), s.to_string(), t.to_string()))
        );
    };
    // - modulo 1
    // - *m == 1u32
    test("0", "0", "1", Some(("0", "0", "0")));
    // - both 0
    // - bits <= Limb::WIDTH
    test("0", "0", "7", Some(("0", "0", "0")));
    // - the second is 0
    test("3*x+5", "0", "7", Some(("x+4", "5", "0")));
    // - the first is 0
    test("0", "3*x+5", "7", Some(("x+4", "0", "5")));
    // - a nonzero constant
    test("x^2+1", "3", "7", Some(("1", "0", "5")));
    // - a nontrivial GCD
    test("x^2+3*x+2", "x^2+4*x+3", "7", Some(("x+1", "6", "1")));
    // - a composite modulus, with only invertible leading coefficients
    test("x^2+5*x", "x+5", "6", Some(("x+5", "0", "1")));
    // - a composite modulus and a non-invertible leading coefficient
    test("x^2+1", "2*x+1", "6", None);
    // - a two-limb prime modulus
    // - bits <= DoubleLimb::WIDTH
    test(
        "x^2+3*x+2",
        "x^2+4*x+3",
        "18446744073709551629",
        Some(("x+1", "18446744073709551628", "1")),
    );
    // - a modulus of more than two limbs
    test(
        "x^2+3*x+2",
        "x^2+4*x+3",
        "170141183460469231731687303715884105727",
        Some(("x+1", "170141183460469231731687303715884105726", "1")),
    );
    // - a modulus of more than two limbs and large coefficients
    test(
        "170141183460469231731687303715884105726*x^2+5",
        "3*x+170141183460469231731687303715884105",
        "170141183460469231731687303715884105727",
        Some((
            "1",
            "87301098706239444592046010873584817065",
            "85814094055569558774577771529822974264*x+119250646955257885171775733050985369944",
        )),
    );
    // - a large composite modulus and a non-invertible leading coefficient
    test("x^2+1", "3*x+1", "5559060566555523", None);
    // - a modulus of more than two limbs
    // - xs.len() >= ys.len()
    // - !(xs.len() >= ys.len())
    // - ys.len() > out.len()
    // - !(*x == 0u32)
    // - !(bits <= Limb::WIDTH) && !(bits <= DoubleLimb::WIDTH)
    test(
        "x^2+3*x+2",
        "x^2+4*x+3",
        "6277101735386680763835789423207666416102355444464034513029",
        Some((
            "x+1",
            "6277101735386680763835789423207666416102355444464034513028",
            "1",
        )),
    );
    // - a modulus of more than two limbs and large coefficients
    test(
        "6277101735386680763835789423207666416102355444464034513028*x^2+5",
        "3*x+62771017353866807638357894232076664161023554444640345",
        "6277101735386680763835789423207666416102355444464034513029",
        Some((
            "1",
            "4260226108527393194368784193024253976782061185526391543847",
            "3512442614638024652734857872077306797628138876663475352292*x+324313919318871555155046\
            4157794844070853145798049806767385",
        )),
    );
    // - a composite modulus of more than two limbs and a non-invertible leading coefficient
    test(
        "x^2+1",
        "3*x+1",
        "18831305206160042291507368269622999248307066333392103539087",
        None,
    );
}

// Polynomials of lengths that reach the half-GCD algorithm, sharing a factor of half their length,
// modulo a prime of more than two limbs.
#[test]
fn test_mod_extended_gcd_algorithms() {
    let m =
        Natural::from_str("6277101735386680763835789423207666416102355444464034513029").unwrap();
    for &(p_len, q_len) in &[(10, 8), (300, 280)] {
        let (p, q, _) = shared_factor_pair(p_len, q_len, &m);
        verify(&p, &q, &m, true);
    }
}

#[test]
#[should_panic]
fn mod_extended_gcd_fail() {
    // A leading coefficient met along the way is not invertible.
    NaturalPolynomial::from_str("x^2+1")
        .unwrap()
        .mod_extended_gcd(
            NaturalPolynomial::from_str("2*x+1").unwrap(),
            Natural::from(6u32),
        );
}

#[test]
#[should_panic]
fn checked_mod_extended_gcd_fail() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .checked_mod_extended_gcd(
            NaturalPolynomial::from_str("7*x+1").unwrap(),
            Natural::from(7u32),
        );
}

#[test]
fn mod_extended_gcd_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            verify(&p, &q, &m, false);
        },
    );

    natural_polynomial_natural_polynomial_natural_triple_gen_var_5().test_properties(
        |(p, q, m)| {
            verify(&p, &q, &m, true);
        },
    );
}
