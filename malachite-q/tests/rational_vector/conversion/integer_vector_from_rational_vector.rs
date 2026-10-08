// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::rational_vector::RationalVector;
use malachite_q::rational_vector::conversion::integer_vector_from_rational_vector::*;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_integer_vector_from_rational_vector() {
    let test = |s, out: Option<&str>| {
        let v = RationalVector::from_str(s).unwrap();
        let w = IntegerVector::try_from(&v);
        assert_eq!(w.as_ref().ok().map(ToString::to_string).as_deref(), out);
        if w.is_err() {
            assert_eq!(w, Err(IntegerVectorFromRationalVectorError));
        }
        assert_eq!(IntegerVector::try_from(v), w);
    };
    test("()", Some("()"));
    test("(0)", Some("(0)"));
    test("(1, -2, 3)", Some("(1, -2, 3)"));
    // An element written as a fraction is fine if it is an integer.
    test("(4/2, -9/3)", Some("(2, -3)"));
    test("(1/2)", None);
    // One non-integer element is enough, wherever it is.
    test("(1/2, 2, 3)", None);
    test("(1, -2/3, 3)", None);
    test("(1, 2, 3/4)", None);
    test(
        "(-1000000000000000000000000, 1)",
        Some("(-1000000000000000000000000, 1)"),
    );
}

#[test]
fn integer_vector_from_rational_vector_properties() {
    rational_vector_gen().test_properties(|v| {
        let w = IntegerVector::try_from(&v);
        assert_eq!(IntegerVector::try_from(v.clone()), w);
        // The conversion succeeds exactly when every element is an integer, which is when the
        // common denominator is 1.
        let (numerators, denominator) = v.to_numerators_and_denominator();
        assert_eq!(w.is_ok(), denominator == 1u32);
        if let Ok(w) = w {
            assert_eq!(w.dimension(), v.dimension());
            assert_eq!(w.to_string(), v.to_string());
            assert_eq!(w, numerators);
            assert_eq!(RationalVector::from(w), v);
        }
    });

    integer_vector_gen().test_properties(|w| {
        // Converting to a `RationalVector` and back is the identity.
        assert_eq!(
            IntegerVector::try_from(RationalVector::from(w.clone())),
            Ok(w)
        );
    });
}

#[test]
fn test_integer_vector_convertible_from_rational_vector() {
    let test = |s, out| {
        let v = RationalVector::from_str(s).unwrap();
        assert_eq!(IntegerVector::convertible_from(&v), out);
    };
    test("()", true);
    test("(3, -1000000000000000000000000)", true);
    test("(3, 1/2)", false);
    test("(-1/3)", false);
    test("(0)", true);
}

#[test]
fn integer_vector_convertible_from_rational_vector_properties() {
    rational_vector_gen().test_properties(|v| {
        assert_eq!(
            IntegerVector::convertible_from(&v),
            IntegerVector::try_from(&v).is_ok()
        );
    });
}
