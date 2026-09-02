// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CanonicalUnitIPow, CanonicalizeUnit};
use malachite_float::test_util::generators::float_gen;
use malachite_float::{ComparableFloat, Float};
use std::str::FromStr;

#[test]
fn test_canonical_unit_i_pow() {
    let test = |s, out| {
        let x = Float::from_str(s).unwrap();
        assert_eq!(x.canonical_unit_i_pow(), out);
    };
    test("0.0", 0);
    test("-0.0", 2);
    test("1.5", 0);
    test("-1.5", 2);
    test("NaN", 0);
    test("Infinity", 0);
    test("-Infinity", 2);
}

#[test]
fn canonical_unit_i_pow_properties() {
    float_gen().test_properties(|x| {
        let k = x.canonical_unit_i_pow();
        assert!(k == 0 || k == 2);
        assert_eq!(k == 2, x.is_sign_negative() && !x.is_nan());
        let y = (&x).canonicalize_unit();
        assert_eq!(y.canonical_unit_i_pow(), 0);
        assert_eq!(
            ComparableFloat(y),
            ComparableFloat(if k == 0 { x } else { -x })
        );
    });
}
