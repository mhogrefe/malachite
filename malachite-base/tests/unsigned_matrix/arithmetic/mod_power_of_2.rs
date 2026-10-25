// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, ModPowerOf2IsReduced,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::{
    unsigned_matrix_gen, unsigned_matrix_unsigned_pair_gen,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2() {
    let test = |s, pow, out| {
        let m = UnsignedMatrix::<u64>::from_str(s).unwrap();
        // by reference
        assert_eq!((&m).mod_power_of_2(pow).to_string(), out);
        // by value
        assert_eq!(m.clone().mod_power_of_2(pow).to_string(), out);
        // in place
        let mut n = m;
        n.mod_power_of_2_assign(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("[]_{0,0}", 0, "[]_{0,0}");
    test("[]_{3,0}", 8, "[]_{3,0}");
    test("[0]", 3, "[0]");
    test("[1, 3; 2, 5]", 1, "[1, 1; 0, 1]");
    test("[1, 3; 2, 5]", 3, "[1, 3; 2, 5]");
    // Modulo 2^0 every element is zero, but the shape is unchanged.
    test("[1, 3; 2, 5]", 0, "[0, 0; 0, 0]");
    // Elements that reduce to zero stay, wherever they are.
    test("[4, 3]", 2, "[0, 3]");
    test("[3; 4]", 2, "[3; 0]");
    test("[8, 9; 10, 1]", 3, "[0, 1; 2, 1]");
    // A power at least as wide as a `u64` leaves the matrix alone.
    test("[18446744073709551615, 1]", 64, "[18446744073709551615, 1]");
    test(
        "[18446744073709551615; 1]",
        100,
        "[18446744073709551615; 1]",
    );
    test("[18446744073709551615, 1]", 63, "[9223372036854775807, 1]");
}

#[test]
fn mod_power_of_2_properties() {
    unsigned_matrix_unsigned_pair_gen().test_properties(|(m, pow)| {
        let n = (&m).mod_power_of_2(pow);
        // The three forms agree.
        assert_eq!(m.clone().mod_power_of_2(pow), n);
        let mut o = m.clone();
        o.mod_power_of_2_assign(pow);
        assert_eq!(o, n);

        // The shape is unchanged.
        assert!(n.is_valid());
        assert_eq!((n.height(), n.width()), (m.height(), m.width()));
        // The result is reduced, and reducing it again changes nothing.
        assert!(n.mod_power_of_2_is_reduced(pow));
        assert_eq!((&n).mod_power_of_2(pow), n);
        // Reducing commutes with transposing.
        assert_eq!(m.transpose().mod_power_of_2(pow), n.transpose());

        // Element by element, this is the `u64` operation.
        for (&x, &y) in m.elements_ref().iter().zip(n.elements_ref()) {
            assert_eq!(y, x.mod_power_of_2(pow));
        }

        // A matrix that is already reduced is left alone.
        assert_eq!(m.mod_power_of_2_is_reduced(pow), m == n);
    });

    unsigned_matrix_gen().test_properties(|m| {
        // A power at least as wide as a `u64` changes nothing, since no element reaches it.
        assert_eq!((&m).mod_power_of_2(u64::WIDTH), m);
        // Modulo 2^0 every element vanishes, but the shape is unchanged.
        assert!(
            (&m).mod_power_of_2(0)
                .elements_ref()
                .iter()
                .all(|&x| x == 0)
        );
        // Reducing to the width of the largest element leaves the matrix alone, and to one bit less
        // does not, unless every element was zero already.
        let bits = m
            .elements_ref()
            .iter()
            .map(|x| x.significant_bits())
            .max()
            .unwrap_or(0);
        assert_eq!((&m).mod_power_of_2(bits), m);
        if bits != 0 {
            assert_ne!((&m).mod_power_of_2(bits - 1), m);
        }
    });
}
