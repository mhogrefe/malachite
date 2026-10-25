// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2HowellForm;
use malachite_nz::natural_matrix::NaturalMatrix;

#[test]
fn test_mod_power_of_2_howell_form() {
    let test = |s: &str, pow: u64, out: &str| {
        let a = NaturalMatrix::from_str(s).unwrap();
        assert_eq!((&a).mod_power_of_2_howell_form(pow).to_string(), out);
    };
    test("[]_{0,0}", 100, "[]_{0,0}");
    test("[]_{2,0}", 100, "[]_{2,0}");
    test("[0, 0; 0, 0]", 100, "[0, 0; 0, 0]");
    test(
        "[633825300114114700748351602688]",
        100,
        "[633825300114114700748351602688]",
    );
    test("[2, 4; 6, 0]", 100, "[2, 0; 0, 4]");
    test("[3, 5; 7, 1]", 100, "[1, 23; 0, 32]");
    test(
        "[633825300114114700748351602688, 6; 4, 2]",
        100,
        "[4, 0; 0, 2]",
    );
    test(
        "[633825300114114700748351602688, 2, 6; 2, 0, 4]",
        100,
        "[2, 0, 4; 0, 2, 6; 0, 0, 0]",
    );
    test("[12, 8; 4, 6]", 4, "[4, 0; 0, 2]");
    test("[2, 4; 6, 0]", 64, "[2, 0; 0, 4]");
}
