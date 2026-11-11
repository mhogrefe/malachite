// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::float::conversion::primitive_float_from_float::FloatFromFloatError;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, big_exponent_float_to_float,
};
use malachite_float::test_util::generators::big_exponent_float_gen_var_1;
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use std::cmp::Ordering::*;

const ROUNDING_MODES: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

#[test]
fn test_primitive_float_from_big_exponent_float() {
    let big = big_exponent_float_power_of_2(Integer::from(1u64 << 40));
    assert_eq!(f64::rounding_from(&big, Nearest), (f64::INFINITY, Greater));
    assert_eq!(f64::rounding_from(&big, Floor), (f64::MAX, Less));
    assert_eq!(f64::try_from(&big), Err(FloatFromFloatError::Overflow));
    let neg_big = big_exponent_float_negate(big);
    assert_eq!(f32::rounding_from(&neg_big, Ceiling), (-f32::MAX, Greater));
    let tiny = big_exponent_float_power_of_2(-Integer::from(1u64 << 40));
    assert_eq!(
        NiceFloat(f64::rounding_from(&tiny, Nearest).0),
        NiceFloat(0.0)
    );
    assert_eq!(
        f64::rounding_from(&tiny, Up),
        (f64::MIN_POSITIVE_SUBNORMAL, Greater)
    );
    assert_eq!(f64::try_from(&tiny), Err(FloatFromFloatError::Underflow));
    let neg_tiny = big_exponent_float_negate(tiny);
    assert_eq!(
        NiceFloat(f64::rounding_from(&neg_tiny, Nearest).0),
        NiceFloat(-0.0)
    );
    assert!(!f64::convertible_from(&neg_tiny));
}

fn primitive_float_from_big_exponent_float_properties_helper<T: PrimitiveFloat>()
where
    for<'a> T: RoundingFrom<&'a BigExponentFloat>
        + RoundingFrom<BigExponentFloat>
        + RoundingFrom<&'a Float>
        + TryFrom<&'a BigExponentFloat, Error = FloatFromFloatError>
        + TryFrom<&'a Float, Error = FloatFromFloatError>
        + ConvertibleFrom<&'a BigExponentFloat>
        + ConvertibleFrom<&'a Float>,
{
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(
            T::try_from(&x).map(NiceFloat),
            T::try_from(&f).map(NiceFloat)
        );
        assert_eq!(T::convertible_from(&x), T::convertible_from(&f));
        for rm in ROUNDING_MODES {
            let (y, o) = T::rounding_from(&x, rm);
            let (z, o_z) = T::rounding_from(&f, rm);
            assert_eq!(NiceFloat(y), NiceFloat(z));
            assert_eq!(o, o_z);
            let (y_alt, o_alt) = T::rounding_from(x.clone(), rm);
            assert_eq!(NiceFloat(y_alt), NiceFloat(y));
            assert_eq!(o_alt, o);
        }
    });
}

#[test]
fn primitive_float_from_big_exponent_float_properties() {
    apply_fn_to_primitive_floats!(primitive_float_from_big_exponent_float_properties_helper);
}
