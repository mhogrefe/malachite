// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{BalancedMod, BalancedModPowerOf2, PowerOf2};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen;

#[test]
fn test_balanced_mod_power_of_2() {
    let test = |s, pow, out| {
        let x = NaturalMatrix::from_str(s).unwrap();
        let r = (&x).balanced_mod_power_of_2(pow);
        assert_eq!(r.to_string(), out);
        assert_eq!(x.clone().balanced_mod_power_of_2(pow), r);
    };
    test("[]_{0,0}", 3, "[]_{0,0}");
    test("[]_{3,0}", 3, "[]_{3,0}");
    test("[]_{0,4}", 3, "[]_{0,4}");
    test("[5, 4; 1, 9]", 2, "[1, 0; 1, 1]");
    test("[4; 3]", 1, "[0; 1]");
    test("[19, 23, 20]", 3, "[3, -1, 4]");
    test("[8, 7]", 3, "[0, -1]");
    test("[1, 3]", 0, "[0, 0]");
    test("[18446744073709551615]", 64, "[-1]");
    test("[12345]", 100, "[12345]");
}

#[test]
fn balanced_mod_power_of_2_properties() {
    natural_matrix_unsigned_pair_gen().test_properties(|(a, pow)| {
        let r = (&a).balanced_mod_power_of_2(pow);
        assert_eq!(a.clone().balanced_mod_power_of_2(pow), r);
        // It is balanced_mod with modulus 2^k.
        assert_eq!((&a).balanced_mod(Natural::power_of_2(pow)), r);
        // It is the same as reducing the matrix as an IntegerMatrix.
        assert_eq!(
            IntegerMatrix::from(a.clone()).balanced_mod_power_of_2(pow),
            r
        );
        // The shape is unchanged, and element by element this is the scalar operation.
        assert_eq!((r.height(), r.width()), (a.height(), a.width()));
        for (x, y) in a.elements.iter().zip(&r.elements) {
            assert_eq!(*y, x.balanced_mod_power_of_2(pow));
        }
        // Reducing again changes nothing.
        assert_eq!((&r).balanced_mod_power_of_2(pow), r);
    });
}
