// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Abs, CanonicalUnitIPow, CanonicalizeUnit, CanonicalizeUnitAssign,
};
use malachite_float::test_util::generators::float_gen;
use malachite_float::{ComparableFloat, Float};
use std::str::FromStr;

#[test]
fn test_canonicalize_unit() {
    let test = |s, out| {
        let x = Float::from_str(s).unwrap();

        let y = x.clone().canonicalize_unit();
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);

        let y = (&x).canonicalize_unit();
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);

        let mut y = x;
        y.canonicalize_unit_assign();
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);
    };
    test("0.0", "0.0");
    test("-0.0", "0.0");
    test("1.5", "1.5");
    test("-1.5", "1.5");
    test("NaN", "NaN");
    test("Infinity", "Infinity");
    test("-Infinity", "Infinity");
}

#[test]
fn canonicalize_unit_properties() {
    float_gen().test_properties(|x| {
        let y = x.clone().canonicalize_unit();
        assert!(y.is_valid());
        assert_eq!(
            ComparableFloat((&x).canonicalize_unit()),
            ComparableFloat(y.clone())
        );
        let mut x_alt = x.clone();
        x_alt.canonicalize_unit_assign();
        assert_eq!(ComparableFloat(x_alt), ComparableFloat(y.clone()));

        assert_eq!(ComparableFloat(y.clone()), ComparableFloat((&x).abs()));
        assert!(!y.is_sign_negative());
        assert_eq!(
            ComparableFloat((&y).canonicalize_unit()),
            ComparableFloat(y.clone())
        );
        assert_eq!(y.canonical_unit_i_pow(), 0);
        assert_eq!(
            ComparableFloat((-&x).canonicalize_unit()),
            ComparableFloat(y.clone())
        );
    });
}
