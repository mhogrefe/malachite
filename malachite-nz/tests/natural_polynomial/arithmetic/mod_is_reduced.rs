// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Height, ModIsReduced, ModPowerOf2IsReduced, PowerOf2,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::u64_polynomial_gen;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_gen;

#[test]
fn test_mod_is_reduced() {
    let test = |s, m: u32, out| {
        assert_eq!(
            NaturalPolynomial::from_str(s)
                .unwrap()
                .mod_is_reduced(&Natural::from(m)),
            out
        );
    };
    // The zero polynomial has no coefficients, so it is reduced modulo everything.
    test("0", 1, true);
    test("0", 1000, true);
    // Every coefficient must be below the modulus, not just the leading one.
    test("x^2+3*x+2", 4, true);
    test("x^2+3*x+2", 3, false);
    test("x^2+3*x+2", 1, false);
    test("x^100", 2, true);
    test("x^100", 1, false);
}

#[test]
fn test_mod_is_reduced_big() {
    let p = NaturalPolynomial::from_str("123456789012345678901234567890*x+1").unwrap();
    let m = Natural::from_str("123456789012345678901234567890").unwrap();
    assert_eq!(p.mod_is_reduced(&m), false);
    assert_eq!(p.mod_is_reduced(&(&m + Natural::ONE)), true);
}

#[test]
#[should_panic]
fn mod_is_reduced_fail() {
    NaturalPolynomial::from_str("x")
        .unwrap()
        .mod_is_reduced(&Natural::ZERO);
}

#[test]
fn test_mod_power_of_2_is_reduced() {
    let test = |s, pow, out| {
        assert_eq!(
            NaturalPolynomial::from_str(s)
                .unwrap()
                .mod_power_of_2_is_reduced(pow),
            out
        );
    };
    // The zero polynomial is reduced modulo every power of 2, including 2^0.
    test("0", 0, true);
    test("x^2+3*x+2", 2, true);
    test("x^2+3*x+2", 1, false);
    test("x^100", 1, true);
    test("x^100", 0, false);
}

#[test]
fn mod_is_reduced_properties() {
    natural_polynomial_gen().test_properties(|p| {
        let height = p.to_height();
        // A polynomial is reduced modulo anything above its height, and modulo nothing at or below
        // it.
        assert!(p.mod_is_reduced(&(&height + Natural::ONE)));
        if height != 0u32 {
            assert!(!p.mod_is_reduced(&height));
        }

        // The defining property: every coefficient is below the modulus.
        for m in [1u32, 2, 3, 10, 1000] {
            let m = Natural::from(m);
            assert_eq!(
                p.mod_is_reduced(&m),
                p.coefficients_asc().iter().all(|c| *c < m)
            );
        }

        // The bit count decides the power-of-2 form, and the two forms agree.
        let bits = p.height_significant_bits();
        assert!(p.mod_power_of_2_is_reduced(bits));
        if bits != 0 {
            assert!(!p.mod_power_of_2_is_reduced(bits - 1));
        }
        for pow in 0..8 {
            assert_eq!(
                p.mod_power_of_2_is_reduced(pow),
                p.mod_is_reduced(&Natural::power_of_2(pow))
            );
        }
    });

    u64_polynomial_gen().test_properties(|p| {
        // The `u64` and `Natural` polynomials agree, the conversion changing no coefficient.
        let q = NaturalPolynomial::from(p.clone());
        for pow in 0..8 {
            assert_eq!(
                p.mod_power_of_2_is_reduced(pow),
                q.mod_power_of_2_is_reduced(pow)
            );
        }
        for m in [1u64, 2, 3, 10, 1000] {
            assert_eq!(p.mod_is_reduced(&m), q.mod_is_reduced(&Natural::from(m)));
        }
    });
}
