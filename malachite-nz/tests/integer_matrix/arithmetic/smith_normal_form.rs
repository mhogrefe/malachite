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
    Determinant, ElementaryDivisors, Rank, SmithNormalForm, SmithNormalFormAssign, UnsignedAbs,
};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::smith_normal_form::*;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_gen_var_3, integer_matrix_pair_gen_var_4,
};

// The diagonal elements of `s`.
fn diagonal(s: &IntegerMatrix) -> Vec<Natural> {
    (0..usize::try_from(s.height().min(s.width())).unwrap())
        .map(|i| Natural::try_from(&s[(i, i)]).unwrap())
        .collect()
}

// Checks the Smith normal form S of `a`: the ways of computing it agree, including the
// Kannan-Bachem algorithm for small matrices, the diagonal algorithm for diagonal matrices, and the
// Iliopoulos algorithm for nonsingular ones; S is in Smith normal form, is its own Smith normal
// form, and has the rank of `a`; the Smith normal forms of the transpose and the negative of `a`
// are Sᵀ and S; the elementary divisors are the nonzero diagonal elements of S; and, for a square
// matrix, the product of the diagonal elements is the absolute value of the determinant. Returns S.
fn verify_snf(a: &IntegerMatrix) -> String {
    let s = a.smith_normal_form();
    assert!(s.is_valid());
    assert_eq!(a.clone().smith_normal_form(), s);
    let mut b = a.clone();
    b.smith_normal_form_assign();
    assert_eq!(b, s);
    assert_eq!(s.height(), a.height());
    assert_eq!(s.width(), a.width());
    assert!(s.is_in_smith_normal_form());
    assert_eq!(snf_iterative_hermite(a), s);
    // The Kannan-Bachem algorithm's intermediate elements grow quickly, so it is only cross-checked
    // on small matrices.
    if a.height() <= 4 && a.width() <= 4 {
        assert_eq!(snf_kannan_bachem(a), s);
    }
    if a.is_diagonal() {
        assert_eq!(snf_diagonal(a), s);
    }
    let rank = a.rank();
    if a.is_square() && rank == a.height() {
        let d = a.determinant().unsigned_abs();
        assert_eq!(snf_iliopoulos(a, &d), s);
        assert_eq!(snf_iliopoulos(a, &(d * Natural::from(6u32))), s);
    }
    assert_eq!((&s).smith_normal_form(), s);
    assert_eq!((&s).rank(), rank);
    assert_eq!(a.transpose().smith_normal_form(), s.transpose());
    assert_eq!((-a).smith_normal_form(), s);
    let ed = a.elementary_divisors();
    assert_eq!(a.clone().elementary_divisors(), ed);
    let diagonal = diagonal(&s);
    assert_eq!(ed.len(), usize::try_from(rank).unwrap());
    assert_eq!(ed[..], diagonal[..ed.len()]);
    if a.is_square() {
        assert_eq!(
            diagonal.iter().product::<Natural>(),
            a.determinant().unsigned_abs()
        );
    }
    s.to_string()
}

#[test]
fn test_smith_normal_form() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(verify_snf(&a), out);
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[]_{0,3}");
    test("[]_{2,0}", "[]_{2,0}");
    test("[0, 0; 0, 0]", "[0, 0; 0, 0]");
    test("[3]", "[3]");
    test("[-3]", "[3]");
    test("[1, 2; 3, 4]", "[1, 0; 0, 2]");
    test("[2, 4; 1, 2]", "[1, 0; 0, 0]");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[1, 0, 0; 0, 3, 0; 0, 0, 0]");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "[1, 0, 0; 0, 1, 0; 0, 0, 3]",
    );
    test("[2, 0; 0, 3]", "[1, 0; 0, 6]");
    test(
        "[6, 0, 0; 0, 4, 0; 0, 0, 0]",
        "[2, 0, 0; 0, 12, 0; 0, 0, 0]",
    );
    test("[0, 2, 1; 0, 4, 3]", "[1, 0, 0; 0, 2, 0]");
    test(
        "[2, 4, 4; -6, 6, 12; 10, -4, -16]",
        "[2, 0, 0; 0, 6, 0; 0, 0, 12]",
    );
    test("[1, 2, 3, 4]", "[1, 0, 0, 0]");
    test("[1; 2; 3]", "[1; 0; 0]");
    test("[2, 1; 4, 2; 6, 3]", "[1, 0; 0, 0; 0, 0]");
    test("[6, 4; 4, 6; 9, 3]", "[1, 0; 0, 2; 0, 0]");
    test(
        "[1267650600228229401496703205376, 1; 2535301200456458802993406410752, 3]",
        "[1, 0; 0, 1267650600228229401496703205376]",
    );
    // - snf_kannan_bachem clears a column element equal to the one above it
    test("[1, 2; 1, 3]", "[1, 0; 0, 1]");
    // - snf_kannan_bachem clears a column element that is the negative of the one above it
    test("[1, 2; -1, 3]", "[1, 0; 0, 5]");
    // - snf_kannan_bachem clears a row element equal to the pivot
    test("[2, 2; 0, 1]", "[1, 0; 0, 2]");
    // - snf_kannan_bachem clears a row element that is the negative of the pivot
    test("[2, -2; 0, 1]", "[1, 0; 0, 2]");
}

#[test]
fn smith_normal_form_properties() {
    integer_matrix_gen().test_properties(|a| {
        verify_snf(&a);
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        verify_snf(&(a * b));
    });

    integer_matrix_gen_var_3().test_properties(|a| {
        verify_snf(&a);
    });
}
