// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Abs, AbsAssign};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{big_exponent_float_gen, float_gen};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_abs() {
    let test = |x: &BigExponentFloat, out: &str| {
        let a = x.clone().abs();
        assert!(a.is_valid());
        assert_eq!(format!("{a:#x}"), out);
        assert_eq!(format!("{:#x}", x.abs()), out);
        let mut m = x.clone();
        m.abs_assign();
        assert_eq!(format!("{m:#x}"), out);
    };
    test(&BigExponentFloat::NAN, "NaN");
    test(&BigExponentFloat::INFINITY, "Infinity");
    test(&BigExponentFloat::NEGATIVE_INFINITY, "Infinity");
    test(&BigExponentFloat::ZERO, "0x0.0");
    test(&BigExponentFloat::ONE, "0x1.0");
    test(&BigExponentFloat::from(-1.5), "0x1.8");
    let huge = big_exponent_float_power_of_2(Integer::from_str("100000000000000000000").unwrap());
    assert!(same(&(-&huge).abs(), &huge));
}

#[test]
fn abs_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let a = (&x).abs();
        assert!(a.is_valid());
        assert!(same(&x.clone().abs(), &a));
        let mut m = x.clone();
        m.abs_assign();
        assert!(same(&m, &a));
        assert!(same(&(-&x).abs(), &a));
        assert!(same(&(&a).abs(), &a));
        assert!(!a.is_sign_negative());
        if x.is_finite() && big_exponent_float_exponent_is_small(&x) {
            assert_eq!(Rational::exact_from(&a), Rational::exact_from(&x).abs());
        }
    });

    float_gen().test_properties(|x| {
        assert!(same(
            &float_to_big_exponent_float(&x).abs(),
            &float_to_big_exponent_float(&x.abs())
        ));
    });
}
