// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{natural_matrix_gen, natural_matrix_pair_gen_var_3};

#[test]
fn test_concat_horizontal() {
    let test = |s, t, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = NaturalMatrix::from_str(t).unwrap();
        let c = a.concat_horizontal(&b);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        let c_alt = a.into_concat_horizontal(b);
        assert!(c_alt.is_valid());
        assert_eq!(c_alt, c);
    };
    test("[1, 2; 3, 7]", "[5; 6]", "[1, 2, 5; 3, 7, 6]");
    test("[5; 6]", "[1, 2; 3, 7]", "[5, 1, 2; 6, 3, 7]");
    test("[1; 2]", "[]_{2,0}", "[1; 2]");
    test("[]_{2,0}", "[3, 7; 5, 6]", "[3, 7; 5, 6]");
    test("[1, 2]", "[3, 7, 5]", "[1, 2, 3, 7, 5]");
    test("[]_{0,2}", "[]_{0,3}", "[]_{0,5}");
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}");
    test("[]_{3,0}", "[]_{3,0}", "[]_{3,0}");
}

#[test]
#[should_panic]
fn concat_horizontal_fail_1() {
    let a = NaturalMatrix::from_str("[1; 2]").unwrap();
    let b = NaturalMatrix::from_str("[1; 2; 3]").unwrap();
    let _ = a.concat_horizontal(&b);
}

#[test]
#[should_panic]
fn concat_horizontal_fail_2() {
    let a = NaturalMatrix::from_str("[]_{2,0}").unwrap();
    let b = NaturalMatrix::from_str("[]_{3,0}").unwrap();
    let _ = a.concat_horizontal(&b);
}

#[test]
#[should_panic]
fn concat_horizontal_fail_3() {
    let a = NaturalMatrix::from_str("[]_{0,2}").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    let _ = a.concat_horizontal(&b);
}

#[test]
#[should_panic]
fn into_concat_horizontal_fail_1() {
    let a = NaturalMatrix::from_str("[1; 2]").unwrap();
    let b = NaturalMatrix::from_str("[1; 2; 3]").unwrap();
    let _ = a.into_concat_horizontal(b);
}

#[test]
#[should_panic]
fn into_concat_horizontal_fail_2() {
    let a = NaturalMatrix::from_str("[]_{2,0}").unwrap();
    let b = NaturalMatrix::from_str("[]_{3,0}").unwrap();
    let _ = a.into_concat_horizontal(b);
}

#[test]
#[should_panic]
fn into_concat_horizontal_fail_3() {
    let a = NaturalMatrix::from_str("[]_{0,2}").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    let _ = a.into_concat_horizontal(b);
}

#[test]
fn concat_horizontal_properties() {
    natural_matrix_pair_gen_var_3().test_properties(|(a, b)| {
        let c = a.concat_horizontal(&b);
        assert!(c.is_valid());
        let c_alt = a.clone().into_concat_horizontal(b.clone());
        assert!(c_alt.is_valid());
        assert_eq!(c_alt, c);
        let (height, w1, w2) = (a.height(), a.width(), b.width());
        assert_eq!((c.height(), c.width()), (height, w1 + w2));
        assert_eq!(c.window(0..height, 0..w1), a);
        assert_eq!(c.window(0..height, w1..w1 + w2), b);
        assert_eq!(a.transpose().concat_vertical(&b.transpose()), c.transpose());
    });

    natural_matrix_gen().test_properties(|a| {
        let empty = NaturalMatrix::zero(a.height(), 0);
        assert_eq!(a.concat_horizontal(&empty), a);
        assert_eq!(empty.concat_horizontal(&a), a);
    });
}
