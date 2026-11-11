// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{NaN, One, Zero};
use malachite_base::test_util::hash::hash;
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::{
    ComparableBigExponentFloat, ComparableBigExponentFloatRef,
};
use malachite_float::test_util::generators::{big_exponent_float_gen, big_exponent_float_pair_gen};

#[test]
fn test_hash() {
    let h = |x: &BigExponentFloat| hash(&ComparableBigExponentFloatRef(x));
    assert_eq!(h(&BigExponentFloat::NAN), h(&BigExponentFloat::NAN));
    assert_eq!(h(&BigExponentFloat::ONE), h(&BigExponentFloat::from(1.0)));
    assert_ne!(h(&BigExponentFloat::ONE), h(&BigExponentFloat::ZERO));
}

#[test]
fn hash_properties() {
    big_exponent_float_gen().test_properties(|x| {
        assert_eq!(
            hash(&ComparableBigExponentFloat(x.clone())),
            hash(&ComparableBigExponentFloatRef(&x))
        );
        assert_eq!(
            hash(&ComparableBigExponentFloatRef(&x)),
            hash(&ComparableBigExponentFloatRef(&x.clone()))
        );
    });

    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        if ComparableBigExponentFloatRef(&x) == ComparableBigExponentFloatRef(&y) {
            assert_eq!(
                hash(&ComparableBigExponentFloatRef(&x)),
                hash(&ComparableBigExponentFloatRef(&y))
            );
        }
    });
}
