// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::conversion::natural_matrix_from_integer_matrix::*;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{integer_matrix_gen, natural_matrix_gen};

#[test]
fn test_natural_matrix_from_integer_matrix() {
    let test = |s, out: Option<&str>| {
        let m = IntegerMatrix::from_str(s).unwrap();
        let n = NaturalMatrix::try_from(&m);
        assert_eq!(n.as_ref().ok().map(ToString::to_string).as_deref(), out);
        if let Ok(n) = &n {
            assert!(n.is_valid());
        } else {
            assert_eq!(n, Err(NaturalMatrixFromIntegerMatrixError));
        }
        assert_eq!(NaturalMatrix::try_from(m), n);
    };
    test("[]_{0,0}", Some("[]_{0,0}"));
    test("[]_{0,3}", Some("[]_{0,3}"));
    test("[]_{3,0}", Some("[]_{3,0}"));
    test("[0]", Some("[0]"));
    test("[1, 2, 3; 4, 5, 6]", Some("[1, 2, 3; 4, 5, 6]"));
    test(
        "[123456789012345678901234567890; 0]",
        Some("[123456789012345678901234567890; 0]"),
    );
    // One negative element is enough, wherever it is.
    test("[-1]", None);
    test("[-1, 2; 3, 4]", None);
    test("[1, 2; 3, -4]", None);
    test("[1; -123456789012345678901234567890]", None);
}

#[test]
fn natural_matrix_from_integer_matrix_properties() {
    integer_matrix_gen().test_properties(|m| {
        let n = NaturalMatrix::try_from(&m);
        assert_eq!(NaturalMatrix::try_from(m.clone()), n);
        // The conversion succeeds exactly when no element is negative.
        assert_eq!(n.is_ok(), m.elements.iter().all(|x| *x >= 0u32));
        if let Ok(n) = n {
            assert!(n.is_valid());
            assert_eq!((n.height(), n.width()), (m.height(), m.width()));
            assert_eq!(n.to_string(), m.to_string());
            assert_eq!(IntegerMatrix::from(n), m);
        }
    });

    natural_matrix_gen().test_properties(|m| {
        // Converting to an `IntegerMatrix` and back is the identity.
        assert_eq!(
            NaturalMatrix::try_from(IntegerMatrix::from(m.clone())),
            Ok(m)
        );
    });

    assert_eq!(
        NaturalMatrix::try_from(IntegerMatrix::from_owned_elements(
            1,
            1,
            vec![-Integer::ONE]
        )),
        Err(NaturalMatrixFromIntegerMatrixError)
    );
}

#[test]
fn test_natural_matrix_convertible_from_integer_matrix() {
    let test = |s, out| {
        let m = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(NaturalMatrix::convertible_from(&m), out);
    };
    test("[]_{0,0}", true);
    test("[]_{4,0}", true);
    test("[3, 123456789012345678901234567890]", true);
    test("[3; -1]", false);
    test("[0]", true);
}

#[test]
fn natural_matrix_convertible_from_integer_matrix_properties() {
    integer_matrix_gen().test_properties(|m| {
        assert_eq!(
            NaturalMatrix::convertible_from(&m),
            NaturalMatrix::try_from(&m).is_ok()
        );
    });
}
