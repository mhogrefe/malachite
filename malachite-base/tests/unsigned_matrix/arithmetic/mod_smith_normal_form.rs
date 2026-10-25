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
    ModIsReduced, ModSmithNormalForm, ModSmithNormalFormAssign,
};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_9;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_smith_normal_form() {
    let test = |s: &str, m: u8, out: &str| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        assert_eq!((&a).mod_smith_normal_form(m).to_string(), out);
    };
    test("[]_{0,0}", 6, "[]_{0,0}");
    test("[]_{2,0}", 6, "[]_{2,0}");
    test("[0, 0; 0, 0]", 6, "[0, 0; 0, 0]");
    test("[0]", 1, "[0]");
    test("[4]", 6, "[2]");
    test("[3]", 6, "[3]");
    test("[2, 3; 4, 6]", 12, "[1, 0; 0, 0]");
    test("[1, 2; 3, 4]", 6, "[1, 0; 0, 2]");
    test("[6, 4; 2, 8]", 12, "[2, 0; 0, 4]");
    // - a diagonal element equals the element being cleared, which an extended GCD would swap in
    //   without dividing it (a regression test for an infinite loop)
    test(
        "[0, 2, 8, 2, 11; 6, 11, 11, 11, 11; 6, 0, 9, 9, 3]",
        12,
        "[1, 0, 0, 0, 0; 0, 3, 0, 0, 0; 0, 0, 3, 0, 0]",
    );
    test("[5, 10; 15, 20]", 25, "[5, 0; 0, 5]");
    test(
        "[2, 4, 4; 6, 6, 0; 2, 8, 10]",
        12,
        "[2, 0, 0; 0, 2, 0; 0, 0, 6]",
    );
}

#[test]
fn mod_smith_normal_form_properties() {
    unsigned_matrix_unsigned_pair_gen_var_9().test_properties(|(a, m)| {
        let s = (&a).mod_smith_normal_form(m);
        assert!(s.is_valid());
        assert_eq!(a.clone().mod_smith_normal_form(m), s);
        let mut b = a.clone();
        b.mod_smith_normal_form_assign(m);
        assert_eq!(b, s);
        assert!(s.mod_is_reduced(&m));
        assert!(s.is_diagonal());
        // The diagonal elements are divisors of m that each divide the next, with 0 standing for m.
        let d = usize::try_from(s.height().min(s.width())).unwrap();
        let w = usize::try_from(s.width()).unwrap();
        let diagonal: Vec<u64> = (0..d)
            .map(|i| {
                if s.elements[i * (w + 1)] == 0 {
                    m
                } else {
                    s.elements[i * (w + 1)]
                }
            })
            .collect();
        for pair in diagonal.windows(2) {
            assert_eq!(pair[1] % pair[0], 0);
        }
        assert!(diagonal.iter().all(|&x| m % x == 0));
        assert_eq!((&s).mod_smith_normal_form(m), s);
        assert_eq!(a.transpose().mod_smith_normal_form(m), s.transpose());
    });
}
