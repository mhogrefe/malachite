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
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;
use malachite_q::rational_vector::RationalVector;
use malachite_q::rational_vector::conversion::natural_vector_from_rational_vector::*;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_natural_vector_from_rational_vector() {
    let test = |s, out: Option<&str>| {
        let v = RationalVector::from_str(s).unwrap();
        let w = NaturalVector::try_from(&v);
        assert_eq!(w.as_ref().ok().map(ToString::to_string).as_deref(), out);
        if w.is_err() {
            assert_eq!(w, Err(NaturalVectorFromRationalVectorError));
        }
        assert_eq!(NaturalVector::try_from(v), w);
    };
    test("()", Some("()"));
    test("(0)", Some("(0)"));
    test("(1, 2, 3)", Some("(1, 2, 3)"));
    test("(4/2, 9/3)", Some("(2, 3)"));
    test("(-1)", None);
    test("(1/2)", None);
    test("(-1/2)", None);
    // One bad element is enough, wherever it is.
    test("(-1, 2, 3)", None);
    test("(1, 2/3, 3)", None);
    test("(1, 2, -3)", None);
    test(
        "(1000000000000000000000000, 1)",
        Some("(1000000000000000000000000, 1)"),
    );
}

#[test]
fn natural_vector_from_rational_vector_properties() {
    rational_vector_gen().test_properties(|v| {
        let w = NaturalVector::try_from(&v);
        assert_eq!(NaturalVector::try_from(v.clone()), w);
        // The conversion succeeds exactly when every element is a nonnegative integer.
        assert_eq!(
            w.is_ok(),
            v.elements
                .iter()
                .all(|x| *x >= 0u32 && *x.denominator_ref() == 1u32)
        );
        // It agrees with going through an `IntegerVector`.
        assert_eq!(
            w.as_ref().ok(),
            IntegerVector::try_from(&v)
                .ok()
                .and_then(|i| NaturalVector::try_from(i).ok())
                .as_ref()
        );
        if let Ok(w) = w {
            assert_eq!(w.dimension(), v.dimension());
            assert_eq!(w.to_string(), v.to_string());
            assert_eq!(RationalVector::from(w), v);
        }
    });

    natural_vector_gen().test_properties(|w| {
        // Converting to a `RationalVector` and back is the identity.
        assert_eq!(
            NaturalVector::try_from(RationalVector::from(w.clone())),
            Ok(w)
        );
    });
}

#[test]
fn test_natural_vector_convertible_from_rational_vector() {
    let test = |s, out| {
        let v = RationalVector::from_str(s).unwrap();
        assert_eq!(NaturalVector::convertible_from(&v), out);
    };
    test("()", true);
    test("(3, 1000000000000000000000000)", true);
    test("(3, -1)", false);
    test("(3, 1/2)", false);
    test("(0)", true);
}

#[test]
fn natural_vector_convertible_from_rational_vector_properties() {
    rational_vector_gen().test_properties(|v| {
        assert_eq!(
            NaturalVector::convertible_from(&v),
            NaturalVector::try_from(&v).is_ok()
        );
    });
}
