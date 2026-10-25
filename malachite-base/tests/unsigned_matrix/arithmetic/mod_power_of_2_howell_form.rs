// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2HowellForm;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_howell_form() {
    let test = |s: &str, pow: u64, out: &str| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        assert_eq!((&a).mod_power_of_2_howell_form(pow).to_string(), out);
    };
    test("[]_{0,0}", 3, "[]_{0,0}");
    test("[]_{2,0}", 3, "[]_{2,0}");
    test("[]_{0,2}", 3, "[0, 0; 0, 0]");
    test("[0, 0; 0, 0]", 3, "[0, 0; 0, 0]");
    test("[0]", 0, "[0]");
    test("[5]", 3, "[1]");
    test("[6]", 3, "[2]");
    test("[2, 4; 6, 0]", 3, "[2, 0; 0, 4]");
    test("[1, 2; 3, 4]", 3, "[1, 0; 0, 2]");
    test("[2, 6; 4, 2]", 3, "[2, 0; 0, 2]");
    test("[4, 2, 6; 2, 0, 4]", 3, "[2, 0, 4; 0, 2, 6; 0, 0, 0]");
    test("[0, 2; 0, 4; 0, 6]", 3, "[0, 2; 0, 0; 0, 0]");
    test("[3, 5; 7, 1]", 4, "[1, 7; 0, 0]");
    test("[12, 8; 4, 6]", 4, "[4, 0; 0, 2]");
}
