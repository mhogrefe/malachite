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
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::generators::big_exponent_float_gen;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_canonicalize_unit() {
    let nan = BigExponentFloat::NAN;
    assert!(same(&(&nan).canonicalize_unit(), &nan));
    assert!(same(
        &BigExponentFloat::NEGATIVE_INFINITY.canonicalize_unit(),
        &BigExponentFloat::INFINITY
    ));
    assert!(same(
        &BigExponentFloat::ZERO.canonicalize_unit(),
        &BigExponentFloat::ZERO
    ));
    assert!(same(
        &BigExponentFloat::from(-1.5).canonicalize_unit(),
        &BigExponentFloat::ONE
    ));
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
