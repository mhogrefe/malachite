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
use malachite_base::num::modular::traits::ModPowerOf2IsReduced;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{CheckedModPowerOf2Lcm, ModPowerOf2Rem};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;

// The polynomial with the same coefficients as `p`, which must fit in limbs.
fn to_limbs(p: &NaturalPolynomial) -> UnsignedPolynomial<Limb> {
    UnsignedPolynomial::from_coefficients_asc(
        p.coefficients_asc()
            .iter()
            .map(Limb::wrapping_from)
            .collect(),
    )
}

// Checks the LCM of `p` and `q` modulo $2^k$ in every way of calling it, and against the word-sized
// version when $2^k$ fits in a limb; and returns it.
#[allow(clippy::type_complexity)]
fn verify(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) -> Option<NaturalPolynomial> {
    let r = p.checked_mod_power_of_2_lcm(q, pow);
    assert_eq!(p.clone().checked_mod_power_of_2_lcm(q.clone(), pow), r);
    assert_eq!(p.clone().checked_mod_power_of_2_lcm(q, pow), r);
    assert_eq!(p.checked_mod_power_of_2_lcm(q.clone(), pow), r);
    if pow <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).checked_mod_power_of_2_lcm(to_limbs(q), pow),
            r.as_ref().map(to_limbs)
        );
    }
    if pow == 1 {
        assert!(r.is_some());
    }
    if let Some(l) = &r {
        assert!(l.is_valid());
        assert!(l.mod_power_of_2_is_reduced(pow));
        if *l != NaturalPolynomial::ZERO {
            assert!(l.is_monic());
            assert_eq!(l.mod_power_of_2_rem(p, pow), NaturalPolynomial::ZERO);
            assert_eq!(l.mod_power_of_2_rem(q, pow), NaturalPolynomial::ZERO);
        }
    }
    r
}

#[test]
fn test_checked_mod_power_of_2_lcm() {
    let test = |p, q, pow: u64, out: Option<&str>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, pow).map(|g| g.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - modulo 1
    test("0", "0", 0, Some("0"));
    // - the second is 0
    test("3*x+5", "0", 3, Some("0"));
    // - an even leading coefficient, with the second 0
    test("2*x+1", "0", 3, Some("0"));
    // - a nontrivial GCD
    test("x^3+x^2+3*x+3", "x^2+3*x+2", 3, Some("x^4+3*x^3+5*x^2+x+6"));
    // - an even leading coefficient met along the way
    test("x^2+1", "x+3", 2, None);
    // - the full width of a limb
    test(
        "x^3+x^2+3*x+3",
        "x^2+3*x+2",
        64,
        Some("x^4+3*x^3+5*x^2+9*x+6"),
    );
    // - two limbs
    test(
        "x^3+x^2+3*x+3",
        "x^2+3*x+2",
        128,
        Some("x^4+3*x^3+5*x^2+9*x+6"),
    );
    // - more than two limbs
    test(
        "x^3+x^2+3*x+3",
        "x^2+3*x+2",
        200,
        Some("x^4+3*x^3+5*x^2+9*x+6"),
    );
    // - more than two limbs and an even leading coefficient
    test("x^2+1", "x+3", 200, None);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_lcm_fail() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .checked_mod_power_of_2_lcm(NaturalPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
fn checked_mod_power_of_2_lcm_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );
}
