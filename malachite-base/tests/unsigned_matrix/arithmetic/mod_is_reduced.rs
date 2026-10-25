// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModPowerOf2IsReduced, PowerOf2};
use malachite_base::test_util::generators::{
    unsigned_matrix_gen, unsigned_matrix_unsigned_pair_gen_var_1,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_is_reduced() {
    let test = |s, m: u64, out| {
        assert_eq!(
            UnsignedMatrix::<u64>::from_str(s)
                .unwrap()
                .mod_is_reduced(&m),
            out
        );
    };
    // A matrix with no elements, of any shape, is reduced modulo everything.
    test("[]_{0,0}", 1, true);
    test("[]_{0,0}", 1000, true);
    test("[]_{3,0}", 1, true);
    test("[]_{0,4}", 1, true);
    // Every element must be below the modulus, not just the last one.
    test("[2, 3, 1]", 4, true);
    test("[2, 3, 1]", 3, false);
    test("[3, 1]", 3, false);
    test("[1, 3]", 3, false);
    test("[0, 0]", 1, true);
    test("[0, 1]", 1, false);
    // In a matrix with several rows, any element can make it unreduced.
    test("[2, 3; 1, 0]", 4, true);
    test("[2, 3; 1, 4]", 4, false);
    // An element equal to `u64::MAX` is reduced modulo nothing, since no `u64` modulus exceeds it.
    test("[18446744073709551614]", u64::MAX, true);
    test("[18446744073709551615]", u64::MAX, false);
}

#[test]
#[should_panic]
fn mod_is_reduced_fail() {
    UnsignedMatrix::<u64>::from_str("[1]")
        .unwrap()
        .mod_is_reduced(&0);
}

// A matrix with no elements has nothing to compare with the modulus, so only the explicit check
// makes this panic.
#[test]
#[should_panic]
fn mod_is_reduced_empty_matrix_fail() {
    UnsignedMatrix::<u64>::from_str("[]_{0,0}")
        .unwrap()
        .mod_is_reduced(&0);
}

#[test]
fn mod_is_reduced_properties() {
    unsigned_matrix_unsigned_pair_gen_var_1().test_properties(|(a, m)| {
        let reduced = a.mod_is_reduced(&m);
        // The defining property: every element is below the modulus.
        assert_eq!(reduced, a.elements_ref().iter().all(|&x| x < m));
        // A matrix is reduced exactly when reducing it changes nothing.
        assert_eq!(reduced, &a % m == a);
        // Reducing always gives a reduced matrix.
        assert!((&a % m).mod_is_reduced(&m));
        // Being reduced modulo m means being reduced modulo every larger modulus.
        if reduced && m != u64::MAX {
            assert!(a.mod_is_reduced(&(m + 1)));
        }
    });

    unsigned_matrix_gen().test_properties(|a| {
        // The largest element decides it: a matrix is reduced modulo anything above it, and modulo
        // nothing at or below it.
        let max = a.elements_ref().iter().max().copied().unwrap_or(0);
        if let Some(above) = max.checked_add(1) {
            assert!(a.mod_is_reduced(&above));
        }
        if max != 0 {
            assert!(!a.mod_is_reduced(&max));
        }
        // The power-of-2 form agrees.
        for pow in 0..64 {
            assert_eq!(
                a.mod_power_of_2_is_reduced(pow),
                a.mod_is_reduced(&u64::power_of_2(pow))
            );
        }
    });
}
