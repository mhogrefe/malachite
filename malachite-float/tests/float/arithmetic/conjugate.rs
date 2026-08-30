// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Conjugate, ConjugateAssign};
use malachite_float::ComparableFloat;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::float_gen;

#[test]
fn test_conjugate() {
    let test = |s, s_hex| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let conjugate = x.clone().conjugate();
        assert!(conjugate.is_valid());
        assert_eq!(ComparableFloat(conjugate), ComparableFloat(x.clone()));

        let conjugate = (&x).conjugate();
        assert!(conjugate.is_valid());
        assert_eq!(ComparableFloat(conjugate), ComparableFloat(x.clone()));

        let mut conjugate = x.clone();
        conjugate.conjugate_assign();
        assert_eq!(ComparableFloat(conjugate), ComparableFloat(x));
    };
    test("NaN", "NaN");
    test("Infinity", "Infinity");
    test("-Infinity", "-Infinity");
    test("0.0", "0x0.0");
    test("-0.0", "-0x0.0");
    test("-1.5", "-0x1.8#2");
    test("123.0", "0x7b.0#7");
}

#[test]
fn conjugate_properties() {
    float_gen().test_properties(|x| {
        let conjugate = x.clone().conjugate();
        assert!(conjugate.is_valid());
        assert_eq!(
            ComparableFloat(conjugate.clone()),
            ComparableFloat(x.clone())
        );
        assert_eq!(
            ComparableFloat((&x).conjugate()),
            ComparableFloat(x.clone())
        );
        let mut x_alt = x.clone();
        x_alt.conjugate_assign();
        assert_eq!(ComparableFloat(x_alt), ComparableFloat(x));
    });
}
