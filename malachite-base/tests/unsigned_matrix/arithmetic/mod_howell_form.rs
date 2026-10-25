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
    ModHowellForm, ModHowellFormAssign, ModIsReduced, ModStrongEchelonForm,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_9;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_howell_form::*;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_strong_echelon_form::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Whether enumerating the row span of `a` modulo `m`, as the naive algorithm does, is cheap.
fn a_is_small(a: &UnsignedMatrix<u64>, m: u64) -> bool {
    m.checked_pow(u32::exact_from(a.width() + 1))
        .and_then(|x| x.checked_mul(a.height()))
        .is_some_and(|x| x <= 1 << 14)
}

#[test]
fn test_mod_howell_form() {
    let test = |s, m, out| {
        let a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        let b = (&a).mod_howell_form(m);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_howell_form(m), b);
        let mut c = a.clone();
        c.mod_howell_form_assign(m);
        assert!(c.is_valid());
        assert_eq!(c, b);
        assert_eq!(unsigned_matrix_mod_howell_form_naive(&a, m), b);
    };
    test("[]_{0,0}", 3, "[]_{0,0}");
    test("[]_{0,3}", 4, "[0, 0, 0; 0, 0, 0; 0, 0, 0]");
    test("[]_{2,0}", 4, "[]_{2,0}");
    test("[0]", 1, "[0]");
    test("[0, 0; 0, 0]", 1, "[0, 0; 0, 0]");
    test("[3]", 7, "[1]");
    test("[3]", 6, "[3]");
    test("[4]", 6, "[2]");
    test("[0]", 6, "[0]");
    test("[2, 1]", 4, "[2, 1; 0, 2]");
    test("[2; 3]", 6, "[1; 0]");
    test("[0, 2; 0, 0; 0, 0]", 4, "[0, 2; 0, 0; 0, 0]");
    test("[4, 2; 2, 0]", 8, "[2, 0; 0, 2]");
    test("[1, 2; 3, 4]", 5, "[1, 0; 0, 1]");
    test("[1, 2; 2, 4]", 5, "[1, 2; 0, 0]");
    test("[2, 4; 4, 2]", 6, "[2, 4; 0, 0]");
    test("[3, 6; 6, 3]", 9, "[3, 6; 0, 0]");
    test("[2, 0, 1; 0, 2, 1]", 4, "[2, 0, 1; 0, 2, 1; 0, 0, 2]");
    test(
        "[6, 4, 2; 3, 0, 3; 0, 2, 4]",
        12,
        "[3, 0, 3; 0, 2, 4; 0, 0, 0]",
    );
    test("[0, 0; 0, 3; 0, 0]", 9, "[0, 3; 0, 0; 0, 0]");
    test(
        "[1, 1, 1; 1, 1, 1; 1, 1, 1]",
        2,
        "[1, 1, 1; 0, 0, 0; 0, 0, 0]",
    );
    test("[5, 3; 2, 7; 1, 4; 0, 0]", 8, "[1, 0; 0, 1; 0, 0; 0, 0]");
    test("[4, 6, 2]", 8, "[4, 2, 6; 0, 4, 4; 0, 0, 0]");
    test("[2, 3; 4, 6]", 12, "[2, 3; 0, 6]");
    test("[255, 128; 64, 32]", 256, "[1, 0; 0, 32]");
    test("[8]", 20, "[4]");
    test("[12, 1]", 28, "[4, 5; 0, 7]");
}

#[test]
#[should_panic]
fn mod_howell_form_fail_1() {
    // The modulus is zero.
    let _ = UnsignedMatrix::<u64>::from_str("[]_{0,0}")
        .unwrap()
        .mod_howell_form(0);
}

#[test]
#[should_panic]
fn mod_howell_form_fail_2() {
    // The matrix is not reduced.
    let _ = UnsignedMatrix::<u64>::from_str("[7]")
        .unwrap()
        .mod_howell_form(7);
}

#[test]
#[should_panic]
fn mod_howell_form_assign_fail() {
    let mut a = UnsignedMatrix::<u64>::from_str("[1, 7]").unwrap();
    a.mod_howell_form_assign(7);
}

#[test]
fn mod_howell_form_properties() {
    unsigned_matrix_unsigned_pair_gen_var_9().test_properties(|(a, m)| {
        let b = (&a).mod_howell_form(m);
        assert!(b.is_valid());
        assert_eq!(a.clone().mod_howell_form(m), b);
        let mut c = a.clone();
        c.mod_howell_form_assign(m);
        assert_eq!(c, b);
        assert_eq!(
            (b.height(), b.width()),
            (a.height().max(a.width()), a.width())
        );
        assert!(b.mod_is_reduced(&m));
        // The form is canonical: it is its own form, and so is that of any matrix with the same row
        // span.
        assert_eq!((&b).mod_howell_form(m), b);
        assert_eq!(
            unsigned_matrix_mod_row_equivalent(&a, m).mod_howell_form(m),
            b
        );
        // The Howell form is the strong echelon form with its zero rows moved to the end.
        let s = (&a).mod_strong_echelon_form(m);
        let w = s.width();
        let mut nonzero: Vec<_> = (0..s.height())
            .filter(|&i| s.row_iter(i).any(|x| *x != 0))
            .flat_map(|i| s.row_iter(i).copied().collect::<Vec<_>>())
            .collect();
        nonzero.resize(b.elements_ref().len(), Default::default());
        assert_eq!(b.elements_ref(), nonzero);
        assert_eq!(w, b.width());
        if a_is_small(&a, m) {
            assert_eq!(unsigned_matrix_mod_howell_form_naive(&a, m), b);
        }
    });
}
