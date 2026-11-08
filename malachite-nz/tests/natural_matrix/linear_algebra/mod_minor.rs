// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::{Minor, ModDeterminant, ModMinor};
use malachite_base::num::modular::traits::Mod;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::*;
use std::panic::catch_unwind;

#[test]
fn test_mod_minor() {
    let test = |s, i, j, m: u32, out: &str| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from(m);
        let x = (&a).mod_minor(i, j, &m);
        assert_eq!(x.to_string(), out);
        assert_eq!((&a).mod_minor(i, j, m.clone()), x);
        assert_eq!(a.clone().mod_minor(i, j, &m), x);
        assert_eq!(a.mod_minor(i, j, m), x);
    };
    test("[5]", 0, 0, 7, "1");
    test("[1, 2; 3, 4]", 0, 0, 5, "4");
    test("[1, 2; 3, 4]", 0, 1, 5, "3");
    test("[1, 2; 3, 4]", 1, 0, 5, "2");
    test("[1, 2; 3, 4]", 1, 1, 5, "1");
    test("[1, 2, 3; 4, 5, 6; 0, 1, 6]", 0, 0, 7, "3");
    test("[1, 2, 3; 4, 5, 6; 0, 1, 6]", 0, 1, 7, "3");
    test("[1, 2, 3; 4, 5, 6; 0, 1, 6]", 0, 2, 7, "4");
    test("[1, 2, 3; 4, 5, 6; 0, 1, 6]", 1, 0, 7, "2");
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        0,
        6,
        "5",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        1,
        6,
        "5",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        2,
        6,
        "2",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        3,
        6,
        "2",
    );
    test("[1, 2, 3; 2, 4, 0; 1, 0, 1]", 0, 0, 6, "4");
    test("[1, 2, 3; 2, 4, 0; 1, 0, 1]", 0, 1, 6, "2");
    test("[1, 2, 3; 2, 4, 0; 1, 0, 1]", 0, 2, 6, "2");
    test("[1, 2, 3; 2, 4, 0; 1, 0, 1]", 1, 0, 6, "2");
    test("[0, 0; 0, 0]", 0, 0, 1, "0");
    test("[0, 0; 0, 0]", 0, 1, 1, "0");
    test("[0, 0; 0, 0]", 1, 0, 1, "0");
    test("[0, 0; 0, 0]", 1, 1, 1, "0");
}

#[test]
fn mod_minor_fail() {
    assert_panic!(
        NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .mod_minor(0, 0, Natural::from(7u32))
    );
    assert_panic!(NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap().mod_minor(
        2,
        0,
        Natural::from(7u32)
    ));
    assert_panic!(NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap().mod_minor(
        0,
        2,
        Natural::from(7u32)
    ));
    assert_panic!(NaturalMatrix::from_str("[9, 2; 3, 4]").unwrap().mod_minor(
        0,
        0,
        Natural::from(7u32)
    ));
    assert_panic!(
        NaturalMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .mod_minor(0, 0, Natural::ZERO)
    );
}

#[test]
fn mod_minor_properties() {
    natural_matrix_unsigned_unsigned_natural_quadruple_gen_var_1().test_properties(
        |(a, i, j, m)| {
            let x = (&a).mod_minor(i, j, &m);
            assert_eq!(a.without_row_and_column(i, j).mod_determinant(&m), x);
            assert_eq!(a.transpose().mod_minor(j, i, &m), x);
            assert_eq!(
                Natural::exact_from((&a).minor(i, j).mod_op(Integer::from(&m))),
                x
            );
        },
    );
}
