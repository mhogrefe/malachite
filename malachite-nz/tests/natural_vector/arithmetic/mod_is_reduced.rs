// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModPowerOf2IsReduced, PowerOf2};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_gen, natural_vector_natural_pair_gen_var_1,
};

#[test]
fn test_mod_is_reduced() {
    let test = |s, m: u32, out| {
        assert_eq!(
            NaturalVector::from_str(s)
                .unwrap()
                .mod_is_reduced(&Natural::from(m)),
            out
        );
    };
    // The 0-dimensional vector has no elements, so it is reduced modulo everything.
    test("()", 1, true);
    test("()", 1000, true);
    // Every element must be below the modulus, not just the last one.
    test("(2, 3, 1)", 4, true);
    test("(2, 3, 1)", 3, false);
    test("(3, 1)", 3, false);
    test("(1, 3)", 3, false);
    test("(0, 0)", 1, true);
    test("(0, 1)", 1, false);
}

#[test]
fn test_mod_is_reduced_big() {
    let v = NaturalVector::from_str("(123456789012345678901234567890, 1)").unwrap();
    let m = Natural::from_str("123456789012345678901234567890").unwrap();
    assert_eq!(v.mod_is_reduced(&m), false);
    assert_eq!(v.mod_is_reduced(&(&m + Natural::ONE)), true);
}

#[test]
#[should_panic]
fn mod_is_reduced_fail() {
    NaturalVector::from_str("(1)")
        .unwrap()
        .mod_is_reduced(&Natural::ZERO);
}

// The 0-dimensional vector has no elements to compare with the modulus, so only the explicit check
// makes this panic.
#[test]
#[should_panic]
fn mod_is_reduced_empty_vector_fail() {
    NaturalVector::from_str("()")
        .unwrap()
        .mod_is_reduced(&Natural::ZERO);
}

#[test]
fn mod_is_reduced_properties() {
    natural_vector_natural_pair_gen_var_1().test_properties(|(v, m)| {
        let reduced = v.mod_is_reduced(&m);
        // The defining property: every element is below the modulus.
        assert_eq!(reduced, v.elements.iter().all(|x| *x < m));
        // A vector is reduced exactly when reducing it changes nothing.
        assert_eq!(reduced, &v % &m == v);
        // Reducing always gives a reduced vector.
        assert!((&v % &m).mod_is_reduced(&m));
        // Being reduced modulo m means being reduced modulo every larger modulus.
        if reduced {
            assert!(v.mod_is_reduced(&(&m + Natural::ONE)));
        }
    });

    natural_vector_gen().test_properties(|v| {
        // The largest element decides it: a vector is reduced modulo anything above it, and modulo
        // nothing at or below it.
        let max = v.elements.iter().max().cloned().unwrap_or_default();
        assert!(v.mod_is_reduced(&(&max + Natural::ONE)));
        if max != 0u32 {
            assert!(!v.mod_is_reduced(&max));
        }
        // The power-of-2 form agrees.
        for pow in 0..8 {
            assert_eq!(
                v.mod_power_of_2_is_reduced(pow),
                v.mod_is_reduced(&Natural::power_of_2(pow))
            );
        }
    });

    unsigned_vector_gen().test_properties(|v| {
        // A vector of `u64`s is reduced modulo 2^64, and the conversion changes nothing.
        let w = NaturalVector::from(v.clone());
        assert!(w.mod_is_reduced(&Natural::power_of_2(64)));
        for m in [1u64, 2, 3, 10, 1000] {
            assert_eq!(
                w.mod_is_reduced(&Natural::from(m)),
                v.elements.iter().all(|&x| x < m)
            );
        }
    });
}
