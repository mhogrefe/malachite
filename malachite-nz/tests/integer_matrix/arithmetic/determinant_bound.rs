// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::Two;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_matrix_gen_var_1;
use malachite_nz::test_util::integer_matrix::arithmetic::determinant::*;
use malachite_nz::test_util::integer_matrix::arithmetic::determinant_bound::*;
use std::panic::catch_unwind;

// The bound is computed with floating-point magnitudes that round up, so it can exceed the exact
// Hadamard bound, but only by a small relative error.
fn assert_close_above(bound: &Natural, exact: &Natural) {
    assert!(bound >= exact);
    assert!(*bound <= exact + (exact >> 10u32) + Natural::TWO);
}

#[test]
fn test_determinant_bound() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let bound = a.determinant_bound();
        assert_eq!(bound.to_string(), out);
        assert_close_above(&bound, &integer_matrix_hadamard_bound_naive(&a, true));
    };
    test("[]_{0,0}", "1");
    test("[0]", "0");
    test("[1]", "2");
    test("[5]", "6");
    test("[-7]", "8");
    test("[1, 2; 3, 4]", "12");
    test("[0, 0; 0, 0]", "0");
    test("[1, 2; 0, 0]", "0");
    test("[1, 0, 0; 0, 1, 0; 0, 0, 1]", "2");
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "1606938068204233101571475504190334901745783987825591619420160",
    );
    test(
        "[1267650600228229401496703205376]",
        "1267650609672962367235993632768",
    );
    test(
        "[-3802951800684688204490109616128, 2; 5, 7]",
        "27685907175177997436194335490048",
    );
    test("[3, -4; 4, 3]", "26");
    test("[1073741823, 1073741825; 1, 1]", "2147483672");
    // - an element of more than one 64-bit word, whose top word has fewer than 30 bits
    test("[92233720368547758087]", "92233720780864618496");
    // - the same, where rounding up overflows to the next power of 2
    test("[1180591620717411303423]", "1180591627314481070080");
    test("[18446744073709551616]", "18446744211148505088");
}

#[test]
fn determinant_bound_fail() {
    assert_panic!(
        IntegerMatrix::from_str("[1, 2]")
            .unwrap()
            .determinant_bound()
    );
    assert_panic!(
        IntegerMatrix::from_str("[]_{2,0}")
            .unwrap()
            .determinant_bound()
    );
}

#[test]
fn determinant_bound_properties() {
    integer_matrix_gen_var_1().test_properties(|a| {
        let bound = a.determinant_bound();
        assert_close_above(&bound, &integer_matrix_hadamard_bound_naive(&a, true));
        assert!(bound >= *integer_matrix_determinant_naive(&a).unsigned_abs_ref());
        assert_eq!(a.transpose().determinant_bound(), bound);
        assert_eq!((-&a).determinant_bound(), bound);
        // Leaving out zero rows and columns changes the bound only if there are some, in which case
        // the bound is 0.
        let has_zero_line = (0..a.height()).any(|i| a.is_zero_row(i))
            || (0..a.width()).any(|j| a.column_iter(j).all(|x| *x == 0u32));
        if has_zero_line {
            assert_eq!(bound, 0u32);
        } else {
            assert_eq!(a.submatrix_determinant_bound(), bound);
        }
    });
}
