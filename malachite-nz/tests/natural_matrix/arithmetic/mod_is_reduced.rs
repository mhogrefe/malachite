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
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_gen, natural_matrix_natural_pair_gen_var_1,
};

#[test]
fn test_mod_is_reduced() {
    let test = |s, m: u32, out| {
        assert_eq!(
            NaturalMatrix::from_str(s)
                .unwrap()
                .mod_is_reduced(&Natural::from(m)),
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
}

#[test]
fn test_mod_is_reduced_big() {
    let a = NaturalMatrix::from_str("[123456789012345678901234567890, 1]").unwrap();
    let m = Natural::from_str("123456789012345678901234567890").unwrap();
    assert_eq!(a.mod_is_reduced(&m), false);
    assert_eq!(a.mod_is_reduced(&(&m + Natural::ONE)), true);
}

#[test]
#[should_panic]
fn mod_is_reduced_fail() {
    NaturalMatrix::from_str("[1]")
        .unwrap()
        .mod_is_reduced(&Natural::ZERO);
}

// A matrix with no elements has nothing to compare with the modulus, so only the explicit check
// makes this panic.
#[test]
#[should_panic]
fn mod_is_reduced_empty_matrix_fail() {
    NaturalMatrix::from_str("[]_{0,0}")
        .unwrap()
        .mod_is_reduced(&Natural::ZERO);
}

#[test]
fn mod_is_reduced_properties() {
    natural_matrix_natural_pair_gen_var_1().test_properties(|(a, m)| {
        let reduced = a.mod_is_reduced(&m);
        // The defining property: every element is below the modulus.
        assert_eq!(reduced, a.elements.iter().all(|x| *x < m));
        // A matrix is reduced exactly when reducing it changes nothing.
        assert_eq!(reduced, &a % &m == a);
        // Reducing always gives a reduced matrix.
        assert!((&a % &m).mod_is_reduced(&m));
        // Being reduced modulo m means being reduced modulo every larger modulus.
        if reduced {
            assert!(a.mod_is_reduced(&(&m + Natural::ONE)));
        }
    });

    natural_matrix_gen().test_properties(|a| {
        // The largest element decides it: a matrix is reduced modulo anything above it, and modulo
        // nothing at or below it.
        let max = a.elements.iter().max().cloned().unwrap_or_default();
        assert!(a.mod_is_reduced(&(&max + Natural::ONE)));
        if max != 0u32 {
            assert!(!a.mod_is_reduced(&max));
        }
        // The power-of-2 form agrees.
        for pow in 0..8 {
            assert_eq!(
                a.mod_power_of_2_is_reduced(pow),
                a.mod_is_reduced(&Natural::power_of_2(pow))
            );
        }
    });

    unsigned_matrix_gen().test_properties(|a| {
        // A matrix of `u64`s is reduced modulo 2^64, and the conversion changes nothing.
        let b = NaturalMatrix::from(a.clone());
        assert!(b.mod_is_reduced(&Natural::power_of_2(64)));
        for m in [1u64, 2, 3, 10, 1000] {
            assert_eq!(
                b.mod_is_reduced(&Natural::from(m)),
                a.elements.iter().all(|&x| x < m)
            );
        }
    });
}
