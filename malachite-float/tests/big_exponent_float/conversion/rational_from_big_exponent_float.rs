// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NaN};
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    big_exponent_float_to_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1,
};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

#[test]
fn test_rational_from_big_exponent_float() {
    let x = big_exponent_float_power_of_2(-Integer::from(1u64 << 20));
    assert_eq!(
        Rational::try_from(&x).unwrap(),
        Rational::from(1u32) >> (1u64 << 20)
    );
    assert!(Rational::try_from(&BigExponentFloat::NAN).is_err());
    assert!(Rational::try_from(BigExponentFloat::INFINITY).is_err());
}

#[test]
fn rational_from_big_exponent_float_properties() {
    big_exponent_float_gen().test_properties(|x| {
        if !big_exponent_float_exponent_is_small(&x) {
            return;
        }
        let result = Rational::try_from(&x);
        assert_eq!(result.is_ok(), Rational::convertible_from(&x));
        assert_eq!(result.is_ok(), x.is_finite());
        if let Ok(q) = result {
            // - round trip
            assert_eq!(
                format!("{:#x}", BigExponentFloat::try_from(&q).unwrap()),
                format!("{x:#x}")
            );
        }
    });

    big_exponent_float_gen_var_1().test_properties(|x| {
        if !big_exponent_float_exponent_is_small(&x) {
            return;
        }
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(Rational::try_from(&x).ok(), Rational::try_from(&f).ok());
    });
}
