// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Conjugate, ConjugateAssign};
use malachite_base::num::basic::traits::{NaN, NegativeInfinity};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::generators::big_exponent_float_gen;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_conjugate() {
    let nan = BigExponentFloat::NAN;
    assert!(same(&nan.clone().conjugate(), &nan));
    assert!(same(
        &BigExponentFloat::NEGATIVE_INFINITY.conjugate(),
        &BigExponentFloat::NEGATIVE_INFINITY
    ));
}

#[test]
fn conjugate_properties() {
    big_exponent_float_gen().test_properties(|x| {
        assert!(same(&x.clone().conjugate(), &x));
        assert!(same(&(&x).conjugate(), &x));
        let mut y = x.clone();
        y.conjugate_assign();
        assert!(same(&y, &x));
    });
}
