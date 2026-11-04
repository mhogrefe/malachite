// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{CheckedModPowerOf2Gcd, ModPowerOf2CoprimeWith};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
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

// The polynomial `p`, reduced modulo 2.
fn low_bits(p: &NaturalPolynomial) -> NaturalPolynomial {
    NaturalPolynomial::from_coefficients_asc(
        p.coefficients_asc()
            .iter()
            .map(|x| Natural::from(x.odd()))
            .collect(),
    )
}

// Checks the coprimality of `p` and `q` modulo $2^k$ in every way of calling it, symmetry,
// agreement with the checked GCD where that succeeds, and, when $2^k$ fits in a limb, agreement
// with `UnsignedPolynomial`; and returns it.
fn verify(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) -> bool {
    let coprime = p.mod_power_of_2_coprime_with(q, pow);
    assert_eq!(
        p.clone().mod_power_of_2_coprime_with(q.clone(), pow),
        coprime
    );
    assert_eq!(p.clone().mod_power_of_2_coprime_with(q, pow), coprime);
    assert_eq!(p.mod_power_of_2_coprime_with(q.clone(), pow), coprime);
    assert_eq!(q.mod_power_of_2_coprime_with(p, pow), coprime);
    if pow != 0
        && let Some(g) = p.checked_mod_power_of_2_gcd(q, pow)
    {
        assert_eq!(g == NaturalPolynomial::one(), coprime);
    }
    if pow <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).mod_power_of_2_coprime_with(to_limbs(q), pow),
            coprime
        );
    }
    coprime
}

#[test]
fn test_mod_power_of_2_coprime_with() {
    let test = |p, q, pow, out| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        assert_eq!(verify(&p, &q, pow), out);
    };
    // - pow == 0
    test("0", "0", 0, true);
    // - pow != 0
    test("0", "0", 3, false);
    test("3", "0", 3, true);
    test("2", "0", 3, false);
    test("2*x+1", "0", 3, true);
    test("x+1", "x+3", 3, false);
    test("4*x^2+x", "2*x+1", 3, true);
    // - a large power of 2
    test(
        "340282366920938463463374607431768211456*x+1",
        "0",
        200,
        true,
    );
    test(
        "340282366920938463463374607431768211457*x+1",
        "340282366920938463463374607431768211455*x+3",
        200,
        false,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_coprime_with_fail() {
    NaturalPolynomial::from_str("x+8")
        .unwrap()
        .mod_power_of_2_coprime_with(NaturalPolynomial::from_str("x").unwrap(), 3);
}

#[test]
fn mod_power_of_2_coprime_with_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, pow)| {
            let coprime = verify(&p, &q, pow);
            // Polynomials are coprime modulo $2^k$ exactly when they are coprime modulo 2.
            if pow != 0 {
                assert_eq!(
                    low_bits(&p).mod_power_of_2_coprime_with(low_bits(&q), 1),
                    coprime
                );
            }
            assert_eq!(
                (&p).mod_power_of_2_coprime_with(NaturalPolynomial::ZERO, pow),
                (&p).mod_power_of_2_coprime_with(&p, pow)
            );
        },
    );
}
