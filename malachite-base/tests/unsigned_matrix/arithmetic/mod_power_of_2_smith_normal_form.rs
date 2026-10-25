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
    ModPowerOf2IsReduced, ModPowerOf2SmithNormalForm, ModPowerOf2SmithNormalFormAssign,
    ModSmithNormalForm,
};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_smith_normal_form() {
    let test = |s: &str, pow: u64, out: &str| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        assert_eq!((&a).mod_power_of_2_smith_normal_form(pow).to_string(), out);
    };
    test("[]_{0,0}", 3, "[]_{0,0}");
    test("[]_{2,0}", 3, "[]_{2,0}");
    test("[]_{0,2}", 3, "[]_{0,2}");
    test("[0, 0; 0, 0]", 3, "[0, 0; 0, 0]");
    test("[0]", 0, "[0]");
    test("[5]", 3, "[1]");
    test("[6]", 3, "[2]");
    test("[2, 4; 6, 0]", 3, "[2, 0; 0, 4]");
    test("[1, 2; 3, 4]", 3, "[1, 0; 0, 2]");
    test("[2, 6; 4, 2]", 3, "[2, 0; 0, 2]");
    test("[4, 2, 6; 2, 0, 4]", 3, "[2, 0, 0; 0, 2, 0]");
    test("[0, 2; 0, 4; 0, 6]", 3, "[2, 0; 0, 0; 0, 0]");
    test("[3, 5; 7, 1]", 4, "[1, 0; 0, 0]");
    test("[12, 8; 4, 6]", 4, "[2, 0; 0, 4]");
}

#[test]
fn mod_power_of_2_smith_normal_form_properties() {
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        let s = (&a).mod_power_of_2_smith_normal_form(pow);
        assert!(s.is_valid());
        assert_eq!(a.clone().mod_power_of_2_smith_normal_form(pow), s);
        let mut b = a.clone();
        b.mod_power_of_2_smith_normal_form_assign(pow);
        assert_eq!(b, s);
        assert!(s.mod_power_of_2_is_reduced(pow));
        assert!(s.is_diagonal());
        // The diagonal elements are powers of 2 in increasing order, followed by zeros.
        let d = usize::try_from(s.height().min(s.width())).unwrap();
        let w = usize::try_from(s.width()).unwrap();
        let diagonal: Vec<u64> = (0..d).map(|i| s.elements[i * (w + 1)]).collect();
        for pair in diagonal.windows(2) {
            assert!(pair[1] == 0 || pair[0] != 0 && pair[1] % pair[0] == 0);
        }
        assert!(diagonal.iter().all(|&x| x == 0 || x.is_power_of_two()));
        assert_eq!((&s).mod_power_of_2_smith_normal_form(pow), s);
        assert_eq!(
            a.transpose().mod_power_of_2_smith_normal_form(pow),
            s.transpose()
        );
        if pow < 64 {
            assert_eq!(a.mod_smith_normal_form(1 << pow), s);
        }
    });
}
