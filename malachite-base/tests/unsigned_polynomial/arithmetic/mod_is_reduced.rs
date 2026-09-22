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
use malachite_base::test_util::generators::unsigned_polynomial_gen;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_is_reduced() {
    let test = |s, m, out| {
        assert_eq!(
            UnsignedPolynomial::<u64>::from_str(s)
                .unwrap()
                .mod_is_reduced(&m),
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
    test("18446744073709551615", 18446744073709551615, false);
}

#[test]
#[should_panic]
fn mod_is_reduced_fail() {
    UnsignedPolynomial::<u64>::from_str("x")
        .unwrap()
        .mod_is_reduced(&0);
}

#[test]
fn test_mod_power_of_2_is_reduced() {
    let test = |s, pow, out| {
        assert_eq!(
            UnsignedPolynomial::<u64>::from_str(s)
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
    test("18446744073709551615", 64, true);
    test("18446744073709551615", 63, false);
}

#[test]
fn mod_is_reduced_properties() {
    unsigned_polynomial_gen().test_properties(|p| {
        let height = p.to_height();
        // A polynomial is reduced modulo anything above its height, and modulo nothing at or below
        // it.
        if let Some(above) = height.checked_add(1) {
            assert!(p.mod_is_reduced(&above));
        }
        if height != 0 {
            assert!(!p.mod_is_reduced(&height));
        }

        // The defining property: every coefficient is below the modulus.
        for m in [1, 2, 3, 10, 1000] {
            assert_eq!(
                p.mod_is_reduced(&m),
                p.coefficients_asc().iter().all(|&c| c < m)
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
                p.mod_is_reduced(&u64::power_of_2(pow))
            );
        }
    });
}
