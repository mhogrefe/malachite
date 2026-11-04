// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::modular::traits::{ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Mul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{
    CheckedModPowerOf2ExtendedGcd, CheckedModPowerOf2Gcd,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;
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

// Checks the extended GCD of `p` and `q` modulo $2^k$ in every way of calling it, against Euclid's
// algorithm and the half-GCD algorithm where they succeed, and against the word-sized version when
// $2^k$ fits in a limb; checks that, when the GCD has a lower degree than both, the cofactors have
// the lowest degrees possible; and returns it.
#[allow(clippy::type_complexity)]
fn verify(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
    let r = p.checked_mod_power_of_2_extended_gcd(q, pow);
    assert_eq!(
        p.clone()
            .checked_mod_power_of_2_extended_gcd(q.clone(), pow),
        r
    );
    assert_eq!(p.clone().checked_mod_power_of_2_extended_gcd(q, pow), r);
    assert_eq!(p.checked_mod_power_of_2_extended_gcd(q.clone(), pow), r);
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
    if pow <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).checked_mod_power_of_2_extended_gcd(to_limbs(q), pow),
            r.as_ref()
                .map(|(g, s, t)| (to_limbs(g), to_limbs(s), to_limbs(t)))
        );
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

#[test]
fn test_checked_mod_power_of_2_extended_gcd() {
    let test = |p, q, pow: u64, out: Option<(&str, &str, &str)>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, pow).map(|(g, s, t)| (g.to_string(), s.to_string(), t.to_string())),
            out.map(|(g, s, t)| (g.to_string(), s.to_string(), t.to_string()))
        );
    };
    // - modulo 1
    test("0", "0", 0, Some(("0", "0", "0")));
    // - the second is 0
    test("3*x+5", "0", 3, Some(("x+7", "3", "0")));
    // - an even leading coefficient, with the second 0
    test("2*x+1", "0", 3, None);
    // - a nontrivial GCD
    test("x^3+x^2+3*x+3", "x^2+3*x+2", 3, Some(("x+1", "7", "x+6")));
    // - an even leading coefficient met along the way
    test("x^2+1", "x+3", 2, None);
    // - the full width of a limb
    test(
        "x^3+x^2+3*x+3",
        "x^2+3*x+2",
        64,
        Some((
            "x+1",
            "7905747460161236407",
            "10540996613548315209*x+15811494920322472814",
        )),
    );
    // - two limbs
    test(
        "x^3+x^2+3*x+3",
        "x^2+3*x+2",
        128,
        Some((
            "x+1",
            "243058833514956045330981862451263008183",
            "97223533405982418132392744980505203273*x+145835300108973627198589117470757804910",
        )),
    );
    // - more than two limbs
    test(
        "x^3+x^2+3*x+3",
        "x^2+3*x+2",
        200,
        Some((
            "x+1",
            "1147812888756421625387115780243687573230144995559137739500983",
            "459125155502568650154846312097475029292057998223655095800393*x+6886877332538529752322\
            69468146212543938086997335482643700590",
        )),
    );
    // - more than two limbs and an even leading coefficient
    test("x^2+1", "x+3", 200, None);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_extended_gcd_fail() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .checked_mod_power_of_2_extended_gcd(NaturalPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
fn checked_mod_power_of_2_extended_gcd_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );
}
