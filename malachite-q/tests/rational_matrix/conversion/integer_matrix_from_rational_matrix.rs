// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::OneHalf;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::integer_matrix_gen;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_matrix::conversion::integer_matrix_from_rational_matrix::*;
use malachite_q::test_util::generators::rational_matrix_gen;

#[test]
fn test_integer_matrix_from_rational_matrix() {
    let test = |s, out: Option<&str>| {
        let m = RationalMatrix::from_str(s).unwrap();
        let n = IntegerMatrix::try_from(&m);
        assert_eq!(n.as_ref().ok().map(ToString::to_string).as_deref(), out);
        if let Ok(n) = &n {
            assert!(n.is_valid());
        } else {
            assert_eq!(n, Err(IntegerMatrixFromRationalMatrixError));
        }
        assert_eq!(IntegerMatrix::try_from(m), n);
    };
    test("[]_{0,0}", Some("[]_{0,0}"));
    test("[]_{0,3}", Some("[]_{0,3}"));
    test("[]_{3,0}", Some("[]_{3,0}"));
    test("[0]", Some("[0]"));
    test("[1, -2, 3; -4, 5, -6]", Some("[1, -2, 3; -4, 5, -6]"));
    test(
        "[-123456789012345678901234567890; 0]",
        Some("[-123456789012345678901234567890; 0]"),
    );
    test("[-4/2, 0]", Some("[-2, 0]"));
    // One non-integer element is enough, wherever it is.
    test("[1/2]", None);
    test("[-1/2]", None);
    test("[1, 2; 3, 7/2]", None);
    test("[1; -123456789012345678901234567891/2]", None);
}

#[test]
fn integer_matrix_from_rational_matrix_properties() {
    rational_matrix_gen().test_properties(|m| {
        let n = IntegerMatrix::try_from(&m);
        assert_eq!(IntegerMatrix::try_from(m.clone()), n);
        // The conversion succeeds exactly when every element is an integer.
        assert_eq!(
            n.is_ok(),
            m.elements.iter().all(|x| *x.denominator_ref() == 1u32)
        );
        if let Ok(n) = n {
            assert!(n.is_valid());
            assert_eq!((n.height(), n.width()), (m.height(), m.width()));
            assert_eq!(n.to_string(), m.to_string());
            assert_eq!(RationalMatrix::from(n), m);
        }
    });

    integer_matrix_gen().test_properties(|m| {
        // Converting to a `RationalMatrix` and back is the identity.
        assert_eq!(
            IntegerMatrix::try_from(RationalMatrix::from(m.clone())),
            Ok(m)
        );
    });

    assert_eq!(
        IntegerMatrix::try_from(RationalMatrix::from_owned_elements(
            1,
            1,
            vec![Rational::ONE_HALF]
        )),
        Err(IntegerMatrixFromRationalMatrixError)
    );
}

#[test]
fn test_integer_matrix_convertible_from_rational_matrix() {
    let test = |s, out| {
        let m = RationalMatrix::from_str(s).unwrap();
        assert_eq!(IntegerMatrix::convertible_from(&m), out);
    };
    test("[]_{0,0}", true);
    test("[]_{4,0}", true);
    test("[3, 123456789012345678901234567890]", true);
    test("[3; -1]", true);
    test("[3; 1/2]", false);
    test("[0]", true);
}

#[test]
fn integer_matrix_convertible_from_rational_matrix_properties() {
    rational_matrix_gen().test_properties(|m| {
        assert_eq!(
            IntegerMatrix::convertible_from(&m),
            IntegerMatrix::try_from(&m).is_ok()
        );
    });
}
