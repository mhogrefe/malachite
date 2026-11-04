// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::{ModIsReduced, ModMul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{
    CheckedModLcm, ModGcd, ModLcm, ModLcmAssign, ModMakeMonic, ModRem,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1,
    natural_polynomial_natural_polynomial_natural_triple_gen_var_5,
};

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

// Checks the LCM of `p` and `q` modulo `m` in every way of calling it, and against the word-sized
// version when `m` fits in a limb; checks that it is 0 or monic, and a multiple of both, and, when
// `m` is prime, that it always exists and its product with the GCD is the monic multiple of `p *
// q`; and returns it.
fn verify(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
    prime: bool,
) -> Option<NaturalPolynomial> {
    let l = p.checked_mod_lcm(q, m);
    assert_eq!(p.clone().checked_mod_lcm(q.clone(), m), l);
    assert_eq!(p.clone().checked_mod_lcm(q, m), l);
    assert_eq!(p.checked_mod_lcm(q.clone(), m), l);
    assert_eq!(p.checked_mod_lcm(q, m.clone()), l);
    if m.significant_bits() <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).checked_mod_lcm(to_limbs(q), Limb::wrapping_from(m)),
            l.as_ref().map(to_limbs)
        );
    }
    if prime {
        assert!(l.is_some());
        assert_eq!(q.checked_mod_lcm(p, m), l);
    }
    if let Some(l) = &l {
        assert!(l.is_valid());
        assert!(l.mod_is_reduced(m));
        if *l != NaturalPolynomial::ZERO {
            assert!(l.is_monic());
            assert_eq!(l.mod_rem(p, m), NaturalPolynomial::ZERO);
            assert_eq!(l.mod_rem(q, m), NaturalPolynomial::ZERO);
            if prime {
                assert_eq!(
                    l.mod_mul(p.mod_gcd(q, m), m),
                    p.mod_mul(q, m).mod_make_monic(m).unwrap()
                );
            }
        }
        assert_eq!(p.mod_lcm(q, m), *l);
        assert_eq!(p.clone().mod_lcm(q.clone(), m.clone()), *l);
        let mut u = p.clone();
        u.mod_lcm_assign(q, m);
        assert_eq!(u, *l);
    }
    l
}

#[test]
fn test_mod_lcm() {
    let test = |p, q, m, out: Option<&str>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        let prime = probably_prime(&m);
        assert_eq!(
            verify(&p, &q, &m, prime).map(|g| g.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - modulo 1
    // - *m == 1u32
    test("0", "0", "1", Some("0"));
    // - the first is 0
    // - bits <= Limb::WIDTH
    test("0", "3*x+5", "7", Some("0"));
    // - constants
    test("3", "5", "7", Some("1"));
    // - a nontrivial GCD
    test("x^2+3*x+2", "x^2+4*x+3", "7", Some("x^3+6*x^2+4*x+6"));
    // - a composite modulus and a non-invertible product of leading coefficients
    test("2*x+1", "3*x+1", "6", None);
    // - a two-limb prime modulus
    // - bits <= DoubleLimb::WIDTH
    test(
        "x^2+3*x+2",
        "x^2+4*x+3",
        "18446744073709551629",
        Some("x^3+6*x^2+11*x+6"),
    );
    // - a modulus of more than two limbs
    test(
        "x^2+3*x+2",
        "x^2+4*x+3",
        "170141183460469231731687303715884105727",
        Some("x^3+6*x^2+11*x+6"),
    );
    // - a modulus of more than two limbs
    // - !(*x == 0u32)
    // - !(bits <= Limb::WIDTH) && !(bits <= DoubleLimb::WIDTH)
    test(
        "x^2+3*x+2",
        "x^2+4*x+3",
        "6277101735386680763835789423207666416102355444464034513029",
        Some("x^3+6*x^2+11*x+6"),
    );
    // - a composite modulus of more than two limbs and a zero product of leading coefficients
    // - *x == 0u32
    test(
        "2*x+1",
        "6277101735386680763835789423207666416102355444464034513029*x+1",
        "12554203470773361527671578846415332832204710888928069026058",
        None,
    );
}

#[test]
#[should_panic]
fn mod_lcm_fail() {
    // The product of the leading coefficients is not invertible.
    NaturalPolynomial::from_str("2*x+1").unwrap().mod_lcm(
        NaturalPolynomial::from_str("3*x+1").unwrap(),
        Natural::from(6u32),
    );
}

#[test]
#[should_panic]
fn checked_mod_lcm_fail() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("x+1").unwrap().checked_mod_lcm(
        NaturalPolynomial::from_str("7*x+1").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
fn mod_lcm_properties() {
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
