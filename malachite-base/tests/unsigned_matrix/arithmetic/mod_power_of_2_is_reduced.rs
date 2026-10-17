// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2IsReduced};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::{
    unsigned_matrix_gen, unsigned_matrix_unsigned_pair_gen,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_is_reduced() {
    let test = |s, pow, out| {
        assert_eq!(
            UnsignedMatrix::<u64>::from_str(s)
                .unwrap()
                .mod_power_of_2_is_reduced(pow),
            out
        );
    };
    // A matrix with no elements, of any shape, is reduced modulo every power of 2, including 2^0.
    test("[]_{0,0}", 0, true);
    test("[]_{0,3}", 8, true);
    test("[]_{3,0}", 0, true);
    // So is a matrix of zeros.
    test("[0, 0; 0, 0]", 0, true);
    test("[1, 3; 2, 0]", 2, true);
    test("[1, 3; 2, 0]", 1, false);
    test("[1, 3; 2, 0]", 0, false);
    // One unreduced element is enough, wherever it is.
    test("[4, 3]", 2, false);
    test("[3; 4]", 2, false);
    test("[3, 3; 3, 3]", 2, true);
    // Every matrix is reduced modulo a power at least as wide as a `u64`.
    test("[18446744073709551615]", 64, true);
    test("[18446744073709551615]", 100, true);
    test("[18446744073709551615]", 63, false);
}

#[test]
fn mod_power_of_2_is_reduced_properties() {
    unsigned_matrix_unsigned_pair_gen().test_properties(|(m, pow)| {
        let reduced = m.mod_power_of_2_is_reduced(pow);
        // The defining property: every element is reduced.
        assert_eq!(
            reduced,
            m.elements.iter().all(|x| x.mod_power_of_2_is_reduced(pow))
        );
        // A matrix is reduced exactly when reducing it changes nothing.
        assert_eq!(reduced, (&m).mod_power_of_2(pow) == m);
        // Reducing always gives a reduced matrix.
        assert!((&m).mod_power_of_2(pow).mod_power_of_2_is_reduced(pow));
        // Transposing does not change it.
        assert_eq!(m.transpose().mod_power_of_2_is_reduced(pow), reduced);
        // Being reduced modulo 2^k means being reduced modulo every larger power.
        if reduced {
            assert!(m.mod_power_of_2_is_reduced(pow + 1));
        }
    });

    unsigned_matrix_gen().test_properties(|m| {
        // Every matrix is reduced modulo 2^64.
        assert!(m.mod_power_of_2_is_reduced(u64::WIDTH));
        // The bit length of the largest element decides it.
        let bits = m
            .elements
            .iter()
            .map(|x| x.significant_bits())
            .max()
            .unwrap_or(0);
        assert!(m.mod_power_of_2_is_reduced(bits));
        if bits != 0 {
            assert!(!m.mod_power_of_2_is_reduced(bits - 1));
        }
    });
}
