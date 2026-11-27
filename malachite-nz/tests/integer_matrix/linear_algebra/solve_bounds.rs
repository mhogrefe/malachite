// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::linear_algebra::traits::{Determinant, Solve};
use malachite_base::num::number_theory::traits::Gcd;
use malachite_base::oracle_row;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_7;
use malachite_nz::test_util::integer_matrix::linear_algebra::determinant_bound::*;
use std::panic::catch_unwind;

#[test]
fn test_solve_bounds() {
    let test = |a, b, out_n, out_d| {
        oracle_row!(
            "flint",
            "fmpz_mat_solve_bound",
            "{a}.solve_bounds({b}) = ({out_n}, {out_d})"
        );

        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerMatrix::from_str(b).unwrap();
        let (n, d) = a.solve_bounds(&b);
        assert_eq!(n.to_string(), out_n);
        assert_eq!(d.to_string(), out_d);
    };
    test("[]_{0,0}", "[]_{0,2}", "1", "1");
    test("[1]", "[5]", "6", "2");
    test("[1, 2; 3, 4]", "[5; 6]", "111", "12");
    test("[1, 2; 3, 4]", "[5, 0; 6, 0]", "111", "12");
    test("[2, 0; 0, 2]", "[]_{2,0}", "0", "5");
    test("[1, 2; 3, 4]", "[0; 0]", "0", "12");
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "[1267650600228229401496703205376; 1]",
        "203703602945433860806572497136341467575706349716924825596808048358362687856150075476331724\
        8",
        "1606938074190543808078853857152627976551679236336291315449856",
    );
    // - the row Hadamard bound is smaller than the column bound, but doesn't bound the numerators:
    //   the solution is (-9, 14, -26, -275), and the row bound times the norm of B is 264
    test(
        "[1, 1, 0, 0; -1, 1, 1, 0; 1, 0, 0, 0; 0, 3, -9, 1]",
        "[5; -3; -9; 1]",
        "561",
        "24",
    );
    test("[0, 0; 0, 0]", "[1; 1]", "0", "0");
}

#[test]
fn solve_bounds_fail() {
    let a = IntegerMatrix::from_str("[1, 2]").unwrap();
    let b = IntegerMatrix::from_str("[1]").unwrap();
    assert_panic!(a.solve_bounds(&b));
    let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    let b = IntegerMatrix::from_str("[1; 2; 3]").unwrap();
    assert_panic!(a.solve_bounds(&b));
}

#[test]
fn solve_bounds_properties() {
    integer_matrix_pair_gen_var_7().test_properties(|(a, b)| {
        let (n, d) = a.solve_bounds(&b);
        // The bounds are computed with a few bits of precision, rounding up.
        let (n_exact, d_exact) = integer_matrix_solve_bounds_naive(&a, &b);
        assert!(n >= n_exact);
        assert!(d >= d_exact);
        // They bound the reduced numerators and denominators of the solution.
        if (&a).determinant() != 0u32 {
            let (x, den) = (&a).solve(&b).unwrap();
            let den = den.unsigned_abs();
            for y in x.elements_ref() {
                let y = y.unsigned_abs_ref();
                let g = y.gcd(&den);
                assert!(y / &g <= n);
                assert!(&den / g <= d);
            }
        }
    });
}
