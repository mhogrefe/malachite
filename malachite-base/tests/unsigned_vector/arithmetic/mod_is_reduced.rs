// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModPowerOf2IsReduced, PowerOf2};
use malachite_base::test_util::generators::{
    unsigned_vector_gen, unsigned_vector_unsigned_pair_gen_var_3,
};
use malachite_base::unsigned_vector::UnsignedVector;

#[test]
fn test_mod_is_reduced() {
    let test = |s, m: u64, out| {
        assert_eq!(
            UnsignedVector::<u64>::from_str(s)
                .unwrap()
                .mod_is_reduced(&m),
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
    // An element equal to `u64::MAX` is reduced modulo nothing, since no `u64` modulus exceeds it.
    test("(18446744073709551614)", u64::MAX, true);
    test("(18446744073709551615)", u64::MAX, false);
}

#[test]
#[should_panic]
fn mod_is_reduced_fail() {
    UnsignedVector::<u64>::from_str("(1)")
        .unwrap()
        .mod_is_reduced(&0);
}

// The 0-dimensional vector has no elements to compare with the modulus, so only the explicit check
// makes this panic.
#[test]
#[should_panic]
fn mod_is_reduced_empty_vector_fail() {
    UnsignedVector::<u64>::from_str("()")
        .unwrap()
        .mod_is_reduced(&0);
}

#[test]
fn mod_is_reduced_properties() {
    unsigned_vector_unsigned_pair_gen_var_3().test_properties(|(v, m)| {
        let reduced = v.mod_is_reduced(&m);
        // The defining property: every element is below the modulus.
        assert_eq!(reduced, v.elements.iter().all(|&x| x < m));
        // A vector is reduced exactly when reducing it changes nothing.
        assert_eq!(reduced, &v % m == v);
        // Reducing always gives a reduced vector.
        assert!((&v % m).mod_is_reduced(&m));
        // Being reduced modulo m means being reduced modulo every larger modulus.
        if reduced && m != u64::MAX {
            assert!(v.mod_is_reduced(&(m + 1)));
        }
    });

    unsigned_vector_gen().test_properties(|v| {
        // The largest element decides it: a vector is reduced modulo anything above it, and modulo
        // nothing at or below it.
        let max = v.elements.iter().max().copied().unwrap_or(0);
        if let Some(above) = max.checked_add(1) {
            assert!(v.mod_is_reduced(&above));
        }
        if max != 0 {
            assert!(!v.mod_is_reduced(&max));
        }
        // The power-of-2 form agrees.
        for pow in 0..64 {
            assert_eq!(
                v.mod_power_of_2_is_reduced(pow),
                v.mod_is_reduced(&u64::power_of_2(pow))
            );
        }
    });
}
