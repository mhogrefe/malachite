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
    Nullspace, Rank, ReducedRowEchelonForm, ReducedRowEchelonFormAssign,
};
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_matrix::arithmetic::reduced_row_echelon_form::*;
use malachite_q::test_util::generators::{
    rational_matrix_gen_var_3, rational_matrix_pair_gen_var_6,
};
use malachite_q::test_util::rational_matrix::arithmetic::solve::rational_matrix_low_rank;

// Checks the reduced row echelon form of `a`: the ways of computing it, classical and
// fraction-free, agree; it is unchanged by another reduction; and its number of nonzero rows is the
// rank. Returns it.
fn verify_rref(a: &RationalMatrix) -> String {
    let r = a.reduced_row_echelon_form();
    assert!(r.is_valid());
    assert_eq!(a.clone().reduced_row_echelon_form(), r);
    let mut b = a.clone();
    b.reduced_row_echelon_form_assign();
    assert_eq!(b, r);
    let mut c = a.clone();
    let rank_classical = rref_classical(&mut c);
    assert_eq!(c, r);
    let mut d = a.clone();
    let rank_fraction_free = rref_fraction_free(&mut d);
    assert_eq!(d, r);
    assert_eq!(rank_classical, rank_fraction_free);
    assert_eq!((&r).reduced_row_echelon_form(), r);
    assert_eq!(u64::try_from(rank_classical).unwrap(), a.rank());
    r.to_string()
}

// Checks the nullspace basis N of `a`: both ways of computing it agree, AN = 0, it has n - r
// columns, and it has full column rank. Returns it.
fn verify_nullspace(a: &RationalMatrix) -> String {
    let x = a.nullspace();
    assert!(x.is_valid());
    assert_eq!(a.clone().nullspace(), x);
    assert_eq!(x.height(), a.width());
    assert_eq!(x.width(), a.width() - a.rank());
    assert!((a * &x).is_zero());
    assert_eq!((&x).rank(), x.width());
    x.to_string()
}

#[test]
fn test_reduced_row_echelon_form() {
    let test = |s, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        assert_eq!(verify_rref(&a), out);
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[]_{0,3}");
    test("[]_{2,0}", "[]_{2,0}");
    test("[0, 0; 0, 0]", "[0, 0; 0, 0]");
    test("[3]", "[1]");
    test("[1, 2; 3, 4]", "[1, 0; 0, 1]");
    test("[2, 4; 1, 2]", "[1, 2; 0, 0]");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        "[1, 0, -1; 0, 1, 2; 0, 0, 0]",
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "[1, 0, 0; 0, 1, 0; 0, 0, 1]",
    );
    test("[0, 2, 1; 0, 4, 3]", "[0, 1, 0; 0, 0, 1]");
    test("[1, 2, 3, 4]", "[1, 2, 3, 4]");
    test("[1; 2; 3]", "[1; 0; 0]");
    test("[2, 1; 4, 2; 6, 3]", "[1, 1/2; 0, 0; 0, 0]");
    test(
        "[1, 2, 3, 4; 2, 4, 6, 8; 1, 0, 1, 0; 3, 2, 4, 4]",
        "[1, 0, 0, 0; 0, 1, 0, 2; 0, 0, 1, 0; 0, 0, 0, 0]",
    );
    test(
        "[0, 0, 1, 2; 0, 0, 2, 4; 1, 1, 0, 0; 2, 2, 1, 2; 3, 3, 1, 2]",
        "[1, 1, 0, 0; 0, 0, 1, 2; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
    );
    test("[1/2, 1/3; 1/4, 1/5]", "[1, 0; 0, 1]");
    test("[1/2, 1/3, 1/4; 1, 2/3, 1/2]", "[1, 2/3, 1/2; 0, 0, 0]");
    test(
        "[1, 1/2, 1/3; 1/2, 1/3, 1/4; 1/3, 1/4, 1/5]",
        "[1, 0, 0; 0, 1, 0; 0, 0, 1]",
    );
}

#[test]
fn test_rank() {
    let test = |s, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        let rank = (&a).rank();
        assert_eq!(a.clone().rank(), rank);
        assert_eq!(a.transpose().rank(), rank);
        assert_eq!(rank.to_string(), out);
    };
    test("[]_{0,0}", "0");
    test("[]_{0,3}", "0");
    test("[]_{2,0}", "0");
    test("[0, 0; 0, 0]", "0");
    test("[3]", "1");
    test("[1, 2; 3, 4]", "2");
    test("[2, 4; 1, 2]", "1");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "2");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "3");
    test("[0, 2, 1; 0, 4, 3]", "2");
    test("[1, 2, 3, 4]", "1");
    test("[1; 2; 3]", "1");
    test("[2, 1; 4, 2; 6, 3]", "1");
    test("[1, 2, 3, 4; 2, 4, 6, 8; 1, 0, 1, 0; 3, 2, 4, 4]", "3");
    test(
        "[0, 0, 1, 2; 0, 0, 2, 4; 1, 1, 0, 0; 2, 2, 1, 2; 3, 3, 1, 2]",
        "2",
    );
    test("[1/2, 1/3; 1/4, 1/5]", "2");
    test("[1/2, 1/3, 1/4; 1, 2/3, 1/2]", "1");
    test("[1, 1/2, 1/3; 1/2, 1/3, 1/4; 1/3, 1/4, 1/5]", "3");
}

#[test]
fn test_nullspace() {
    let test = |s, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        assert_eq!(verify_nullspace(&a), out);
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[1, 0, 0; 0, 1, 0; 0, 0, 1]");
    test("[]_{2,0}", "[]_{0,0}");
    test("[0, 0; 0, 0]", "[1, 0; 0, 1]");
    test("[3]", "[]_{1,0}");
    test("[1, 2; 3, 4]", "[]_{2,0}");
    test("[2, 4; 1, 2]", "[-2; 1]");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[1; -2; 1]");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "[]_{3,0}");
    test("[0, 2, 1; 0, 4, 3]", "[1; 0; 0]");
    test("[1, 2, 3, 4]", "[-2, -3, -4; 1, 0, 0; 0, 1, 0; 0, 0, 1]");
    test("[1; 2; 3]", "[]_{1,0}");
    test("[2, 1; 4, 2; 6, 3]", "[-1/2; 1]");
    test(
        "[1, 2, 3, 4; 2, 4, 6, 8; 1, 0, 1, 0; 3, 2, 4, 4]",
        "[0; -2; 0; 1]",
    );
    test(
        "[0, 0, 1, 2; 0, 0, 2, 4; 1, 1, 0, 0; 2, 2, 1, 2; 3, 3, 1, 2]",
        "[-1, 0; 1, 0; 0, -2; 0, 1]",
    );
    test("[1/2, 1/3; 1/4, 1/5]", "[]_{2,0}");
    test("[1/2, 1/3, 1/4; 1, 2/3, 1/2]", "[-2/3, -1/2; 1, 0; 0, 1]");
    test("[1, 1/2, 1/3; 1/2, 1/3, 1/4; 1/3, 1/4, 1/5]", "[]_{3,0}");
}

#[test]
fn reduced_row_echelon_form_properties() {
    rational_matrix_gen_var_3().test_properties(|a| {
        verify_rref(&a);
        verify_nullspace(&a);
        assert_eq!(a.transpose().rank(), (&a).rank());
    });

    rational_matrix_pair_gen_var_6().test_properties(|(a, b)| {
        let a = rational_matrix_low_rank(&a, &b);
        verify_rref(&a);
        verify_nullspace(&a);
    });
}
