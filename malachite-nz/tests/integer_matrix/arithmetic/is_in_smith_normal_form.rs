// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::SmithNormalForm;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::integer_matrix_gen;

#[test]
fn test_is_in_smith_normal_form() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(a.is_in_smith_normal_form().to_string(), out);
    };
    test("[]_{0,0}", "true");
    test("[]_{0,3}", "true");
    test("[0, 0; 0, 0]", "true");
    test("[3]", "true");
    test("[-3]", "false");
    test("[1, 0; 0, 2]", "true");
    test("[2, 0; 0, 3]", "false");
    test("[1, 0; 0, -2]", "false");
    test("[1, 1; 0, 2]", "false");
    test("[0, 0; 0, 1]", "false");
    test("[1, 0; 0, 0]", "true");
    test("[2, 0, 0; 0, 6, 0]", "true");
    test("[2, 0; 0, 6; 0, 0]", "true");
}

#[test]
fn is_in_smith_normal_form_properties() {
    integer_matrix_gen().test_properties(|a| {
        assert_eq!(a.is_in_smith_normal_form(), (&a).smith_normal_form() == a);
        assert!(a.smith_normal_form().is_in_smith_normal_form());
    });
}
