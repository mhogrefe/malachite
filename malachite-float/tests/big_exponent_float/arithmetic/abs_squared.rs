// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, AbsSquaredAssign, Square};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::generators::big_exponent_float_gen;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_abs_squared() {
    assert!(same(
        &BigExponentFloat::NEGATIVE_INFINITY.abs_squared(),
        &BigExponentFloat::INFINITY
    ));
    assert!(same(
        &BigExponentFloat::NAN.abs_squared(),
        &BigExponentFloat::NAN
    ));
    // (-1.5)^2 = 2.25
    assert!(same(
        &BigExponentFloat::from(-1.5).abs_squared(),
        &BigExponentFloat::from(2.25)
    ));
}

#[test]
fn abs_squared_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let s = (&x).abs_squared();
        assert!(same(&x.clone().abs_squared(), &s));
        assert!(same(&(&x).square(), &s));
        let mut y = x.clone();
        y.abs_squared_assign();
        assert!(same(&y, &s));
    });
}
