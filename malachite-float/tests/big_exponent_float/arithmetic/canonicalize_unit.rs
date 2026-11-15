// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Abs, CanonicalizeUnit, CanonicalizeUnitAssign, IsUnit,
};
use malachite_base::num::basic::traits::One;
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::parse_big_exponent_float_hex;
use malachite_float::test_util::generators::big_exponent_float_gen;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_canonicalize_unit() {
    let test = |x: &str, x_hex: &str, out: &str, out_hex: &str| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        assert!(same(&x.canonicalize_unit(), out));
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("-Infinity", "-Infinity", "Infinity", "Infinity");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-1.5", "-0x1.8", "1.0", "0x1.0");
}

#[test]
fn canonicalize_unit_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let c = (&x).canonicalize_unit();
        assert!(same(&x.clone().canonicalize_unit(), &c));
        let mut y = x.clone();
        y.canonicalize_unit_assign();
        assert!(same(&y, &c));
        assert!(same(
            &c,
            &if x.is_unit() {
                BigExponentFloat::ONE
            } else {
                (&x).abs()
            }
        ));
    });
}
