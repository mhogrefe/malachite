// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, CanonicalUnitIPow};
use malachite_base::num::basic::traits::{NaN, NegativeInfinity, Zero};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1_float,
};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_canonical_unit_i_pow() {
    assert_eq!(BigExponentFloat::NAN.canonical_unit_i_pow(), 0);
    assert_eq!(
        BigExponentFloat::NEGATIVE_INFINITY.canonical_unit_i_pow(),
        2
    );
    assert_eq!(BigExponentFloat::ZERO.canonical_unit_i_pow(), 0);
    assert_eq!(BigExponentFloat::from(-1.5).canonical_unit_i_pow(), 2);
}

#[test]
fn canonical_unit_i_pow_properties() {
    big_exponent_float_gen().test_properties(|x| {
        // Multiplying x by i^k, that is, negating it when k = 2, gives |x|.
        let k = x.canonical_unit_i_pow();
        assert!(k == 0 || k == 2);
        assert!(same(&(if k == 2 { -&x } else { x.clone() }), &(&x).abs()));
    });
    big_exponent_float_gen_var_1_float().test_properties(|(f, x)| {
        if !f.is_zero() {
            assert_eq!(x.canonical_unit_i_pow(), f.canonical_unit_i_pow());
        }
    });
}
