// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, ModPowerOf2IsReduced,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{natural_matrix_gen, natural_matrix_unsigned_pair_gen};

#[test]
fn test_mod_power_of_2() {
    let test = |s, pow, out| {
        let m = NaturalMatrix::from_str(s).unwrap();
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
    // A power too wide for a `u64` still does something here.
    test("[340282366920938463463374607431768211457]", 64, "[1]");
    test(
        "[340282366920938463463374607431768211456]",
        129,
        "[340282366920938463463374607431768211456]",
    );
}

#[test]
fn mod_power_of_2_properties() {
    natural_matrix_unsigned_pair_gen().test_properties(|(m, pow)| {
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

        // Element by element, this is the `Natural` operation.
        for (x, y) in m.elements.iter().zip(&n.elements) {
            assert_eq!(*y, x.mod_power_of_2(pow));
        }

        // A matrix that is already reduced is left alone.
        assert_eq!(m.mod_power_of_2_is_reduced(pow), m == n);
    });

    natural_matrix_gen().test_properties(|m| {
        // Modulo 2^0 every element vanishes, but the shape is unchanged.
        assert!((&m).mod_power_of_2(0).elements.iter().all(|x| *x == 0u32));
        // Reducing to the width of the largest element leaves the matrix alone, and to one bit less
        // does not, unless every element was zero already.
        let bits = m
            .elements
            .iter()
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(0);
        assert_eq!((&m).mod_power_of_2(bits), m);
        if bits != 0 {
            assert_ne!((&m).mod_power_of_2(bits - 1), m);
        }
    });

    unsigned_matrix_gen().test_properties(|m| {
        // The `u64` elements reduce as their `Natural` counterparts do.
        let n = NaturalMatrix::from(m.clone());
        for pow in 0..8 {
            assert_eq!(
                (&n).mod_power_of_2(pow).elements,
                m.elements
                    .iter()
                    .map(|&x| Natural::from(x.mod_power_of_2(pow)))
                    .collect::<Vec<_>>()
            );
        }
    });
}
