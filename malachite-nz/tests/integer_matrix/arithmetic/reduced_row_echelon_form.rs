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
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::is_in_reduced_row_echelon_form_with_rank::*;
use malachite_nz::integer_matrix::arithmetic::rank::rank_small;
use malachite_nz::integer_matrix::arithmetic::reduced_row_echelon_form::*;
use malachite_nz::test_util::generators::{integer_matrix_gen, integer_matrix_pair_gen_var_4};

// The number of nonzero rows of `r`, a matrix in fraction-free reduced row echelon form.
fn nonzero_rows(r: &IntegerMatrix) -> usize {
    let w = usize::try_from(r.width()).unwrap();
    if w == 0 {
        return 0;
    }
    r.elements_ref()
        .chunks_exact(w)
        .take_while(|row| row.iter().any(|x| *x != 0u32))
        .count()
}

// Checks the fraction-free reduced row echelon form (R, d) of `a`: the three ways of computing it
// agree, it is in reduced row echelon form with denominator d and its number of nonzero rows as the
// rank, d is nonzero, the fraction-free LU and multimodular methods give the same form as rational
// matrices, and the rank is the matrix's rank. Returns it.
fn verify_rref(a: &IntegerMatrix) -> String {
    let (r, den) = a.reduced_row_echelon_form();
    assert_eq!(
        a.clone().reduced_row_echelon_form(),
        (r.clone(), den.clone())
    );
    let mut b = a.clone();
    assert_eq!(b.reduced_row_echelon_form_assign(), den);
    assert_eq!(b, r);
    assert!(r.is_valid());
    assert_eq!(r.height(), a.height());
    assert_eq!(r.width(), a.width());
    assert_ne!(den, 0u32);
    let rank = nonzero_rows(&r);
    assert!(is_in_reduced_row_echelon_form_with_rank(&r, &den, rank));
    assert_eq!(u64::try_from(rank).unwrap(), a.rank());
    let mut f = a.clone();
    let (rank_fflu, den_fflu) = rref_fflu(&mut f);
    let (rank_mul, m, den_mul) = rref_mul(a);
    assert_eq!(rank_fflu, rank);
    assert_eq!(rank_mul, rank);
    assert_eq!(&f * &den_mul, &m * &den_fflu);
    assert_eq!(&r * &den_fflu, &f * &den);
    format!("({r}, {den})")
}

// Checks the nullspace basis N of `a`: both ways of computing it agree, AN = 0, N has n - r
// columns, where n is the width of `a` and r its rank, and N has full column rank. Returns it.
fn verify_nullspace(a: &IntegerMatrix) -> String {
    let x = a.nullspace();
    assert_eq!(a.clone().nullspace(), x);
    assert!(x.is_valid());
    assert_eq!(x.height(), a.width());
    assert_eq!(x.width(), a.width() - a.rank());
    assert!((a * &x).is_zero());
    assert_eq!((&x).rank(), x.width());
    x.to_string()
}

// Checks the rank of `a`: both ways of computing it agree, it is that of the transpose, and, for
// elements of at most 30 bits, the word-sized algorithm gives it too. Returns it.
fn verify_rank(a: &IntegerMatrix) -> u64 {
    let rank = a.rank();
    assert_eq!(a.clone().rank(), rank);
    assert_eq!(a.transpose().rank(), rank);
    assert!(rank <= a.height().min(a.width()));
    if a.elements_ref().iter().all(|x| x.significant_bits() <= 30) {
        let small = rank_small(
            a.elements_ref()
                .iter()
                .map(|x| i64::try_from(x).unwrap())
                .collect(),
            usize::try_from(a.height()).unwrap(),
            usize::try_from(a.width()).unwrap(),
        );
        assert_eq!(u64::try_from(small).unwrap(), rank);
    }
    rank
}

#[test]
fn test_reduced_row_echelon_form() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(verify_rref(&a), out);
    };
    test("[]_{0,0}", "([]_{0,0}, 1)");
    test("[]_{0,3}", "([]_{0,3}, 1)");
    test("[]_{2,0}", "([]_{2,0}, 1)");
    test("[0, 0; 0, 0]", "([0, 0; 0, 0], 1)");
    test("[3]", "([3], 3)");
    test("[1, 2; 3, 4]", "([-2, 0; 0, -2], -2)");
    test("[2, 4; 1, 2]", "([2, 4; 0, 0], 2)");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        "([-3, 0, 3; 0, -3, -6; 0, 0, 0], -3)",
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "([-3, 0, 0; 0, -3, 0; 0, 0, -3], -3)",
    );
    test("[0, 2, 1; 0, 4, 3]", "([0, 2, 0; 0, 0, 2], 2)");
    test("[1, 2, 3, 4]", "([1, 2, 3, 4], 1)");
    test("[1; 2; 3]", "([1; 0; 0], 1)");
    test("[2, 1; 4, 2; 6, 3]", "([2, 1; 0, 0; 0, 0], 2)");
    test(
        "[1, 2, 3, 4; 2, 4, 6, 8; 1, 0, 1, 0; 3, 2, 4, 4]",
        "([2, 0, 0, 0; 0, 2, 0, 4; 0, 0, 2, 0; 0, 0, 0, 0], 2)",
    );
    test(
        "[0, 0, 1, 2; 0, 0, 2, 4; 1, 1, 0, 0; 2, 2, 1, 2; 3, 3, 1, 2]",
        "([2, 2, 0, 0; 0, 0, 2, 4; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0], 2)",
    );
}

#[test]
fn test_rank() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(verify_rank(&a).to_string(), out);
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
    test(
        "[1267650600228229401496703205376, 1; 2535301200456458802993406410752, 2]",
        "1",
    );
}

#[test]
fn test_nullspace() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(verify_nullspace(&a), out);
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[1, 0, 0; 0, 1, 0; 0, 0, 1]");
    test("[]_{2,0}", "[]_{0,0}");
    test("[0, 0; 0, 0]", "[1, 0; 0, 1]");
    test("[3]", "[]_{1,0}");
    test("[1, 2; 3, 4]", "[]_{2,0}");
    test("[2, 4; 1, 2]", "[4; -2]");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[3; -6; 3]");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "[]_{3,0}");
    test("[0, 2, 1; 0, 4, 3]", "[-2; 0; 0]");
    test("[1, 2, 3, 4]", "[2, 3, 4; -1, 0, 0; 0, -1, 0; 0, 0, -1]");
    test("[1; 2; 3]", "[]_{1,0}");
    test("[2, 1; 4, 2; 6, 3]", "[1; -2]");
    test(
        "[1, 2, 3, 4; 2, 4, 6, 8; 1, 0, 1, 0; 3, 2, 4, 4]",
        "[0; 4; 0; -2]",
    );
    test(
        "[0, 0, 1, 2; 0, 0, 2, 4; 1, 1, 0, 0; 2, 2, 1, 2; 3, 3, 1, 2]",
        "[2, 0; -2, 0; 0, 4; 0, -2]",
    );
}

#[test]
fn reduced_row_echelon_form_properties() {
    integer_matrix_gen().test_properties(|a| {
        verify_rref(&a);
        verify_rank(&a);
        verify_nullspace(&a);
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        let a = a * b;
        verify_rref(&a);
        verify_rank(&a);
        verify_nullspace(&a);
    });
}
