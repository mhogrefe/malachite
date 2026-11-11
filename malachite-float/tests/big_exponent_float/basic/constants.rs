// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::comparison::traits::{Max, Min};
use malachite_base::named::Named;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, One, OneHalf, Two, Zero,
};
use malachite_float::BigExponentFloat;

#[test]
fn test_constants() {
    let test = |x: BigExponentFloat, out: &str| {
        assert!(x.is_valid());
        assert_eq!(format!("{x:#x}"), out);
    };
    test(BigExponentFloat::ZERO, "0x0.0");
    test(BigExponentFloat::ONE, "0x1.0");
    test(BigExponentFloat::TWO, "0x2.0");
    test(BigExponentFloat::NEGATIVE_ONE, "-0x1.0");
    test(BigExponentFloat::ONE_HALF, "0x0.8");
    test(BigExponentFloat::INFINITY, "Infinity");
    test(BigExponentFloat::NEGATIVE_INFINITY, "-Infinity");
    test(BigExponentFloat::NAN, "NaN");
    test(BigExponentFloat::default(), "NaN");
    test(BigExponentFloat::MIN, "-Infinity");
    test(BigExponentFloat::MAX, "Infinity");
    assert_eq!(BigExponentFloat::NAME, "BigExponentFloat");
    for x in [
        BigExponentFloat::ONE,
        BigExponentFloat::TWO,
        BigExponentFloat::NEGATIVE_ONE,
        BigExponentFloat::ONE_HALF,
    ] {
        assert_eq!(x.get_prec(), Some(1));
    }
}
