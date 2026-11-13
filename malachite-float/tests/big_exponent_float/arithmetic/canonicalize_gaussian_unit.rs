// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Abs, CanonicalizeGaussianUnit, CanonicalizeGaussianUnitAssign,
};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::generators::big_exponent_float_gen;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_canonicalize_gaussian_unit() {
    let nan = BigExponentFloat::NAN;
    assert!(same(&(&nan).canonicalize_gaussian_unit(), &nan));
    assert!(same(
        &BigExponentFloat::NEGATIVE_INFINITY.canonicalize_gaussian_unit(),
        &BigExponentFloat::INFINITY
    ));
    assert!(same(
        &BigExponentFloat::from(-1.5).canonicalize_gaussian_unit(),
        &BigExponentFloat::from(1.5)
    ));
}

#[test]
fn canonicalize_gaussian_unit_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let g = (&x).canonicalize_gaussian_unit();
        assert!(same(&x.clone().canonicalize_gaussian_unit(), &g));
        let mut y = x.clone();
        y.canonicalize_gaussian_unit_assign();
        assert!(same(&y, &g));
        assert!(same(&g, &(&x).abs()));
    });
}
