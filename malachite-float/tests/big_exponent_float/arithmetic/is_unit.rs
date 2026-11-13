// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsUnit;
use malachite_base::num::basic::traits::{NaN, NegativeInfinity, Zero};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1_float,
};

#[test]
fn test_is_unit() {
    assert!(BigExponentFloat::from(-1.5).is_unit());
    assert!(!BigExponentFloat::ZERO.is_unit());
    assert!(!BigExponentFloat::NAN.is_unit());
    assert!(!BigExponentFloat::NEGATIVE_INFINITY.is_unit());
}

#[test]
fn is_unit_properties() {
    big_exponent_float_gen().test_properties(|x| {
        assert_eq!(x.is_unit(), x.is_finite() && !x.is_zero());
    });
    big_exponent_float_gen_var_1_float().test_properties(|(f, x)| {
        assert_eq!(x.is_unit(), f.is_unit());
    });
}
