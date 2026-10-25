// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2Solve};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;

#[test]
fn test_mod_power_of_2_solve() {
    let test = |a: &str, b: &str, pow: u64, out| {
        let a = UnsignedMatrix::<u8>::from_str(a).unwrap();
        let b = UnsignedVector::<u8>::from_str(b).unwrap();
        let actual = (&a).mod_power_of_2_solve(&b, pow).to_debug_string();
        assert_eq!(a.mod_power_of_2_solve(b, pow).to_debug_string(), actual);
        assert_eq!(actual, out);
    };
    test("[]_{0,2}", "()", 3, "Some((0, 0))");
    test("[]_{2,0}", "(0, 0)", 3, "Some(())");
    test("[]_{2,0}", "(1, 0)", 3, "None");
    test("[1, 2; 3, 4]", "(5, 6)", 3, "None");
    test("[2, 4; 6, 0]", "(2, 6)", 3, "Some((1, 0))");
    test("[2, 4; 6, 0]", "(1, 6)", 3, "None");
    test("[2, 4; 4, 0]", "(4, 0)", 3, "Some((2, 0))");
    test("[4, 2, 6; 2, 0, 4]", "(6, 2)", 3, "Some((1, 1, 0))");
    test("[0, 2; 0, 4; 0, 6]", "(2, 4, 6)", 3, "Some((0, 1))");
    test("[0, 2; 0, 4; 0, 6]", "(2, 4, 7)", 3, "None");
    test("[3, 5; 7, 1]", "(9, 11)", 4, "None");
}

#[test]
fn mod_power_of_2_solve_properties() {
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        // A consistent system: the right-hand side is the sum of the columns.
        let w = a.width();
        let ones = UnsignedMatrix::<u64>::from_owned_elements(
            w,
            1,
            vec![u64::from(pow != 0); usize::try_from(w).unwrap()],
        );
        let b = (&a).mod_power_of_2_mul(&ones, pow);
        let x = (&a).mod_power_of_2_solve(&b, pow).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.height(), w);
        assert_eq!((&a).mod_power_of_2_mul(&x, pow), b);
        let v = UnsignedVector {
            elements: b.to_elements(),
        };
        assert_eq!(
            (&a).mod_power_of_2_solve(&v, pow).unwrap().elements,
            x.elements_ref()
        );
    });
}
