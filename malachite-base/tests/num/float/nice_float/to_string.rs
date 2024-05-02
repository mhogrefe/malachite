// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::float::NiceFloat;
use malachite_base::strings::string_is_subset;
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::num::float::PRIMITIVE_FLOAT_CHARS;
use std::fmt::Debug;
use std::str::FromStr;

#[test]
pub fn test_to_string() {
    fn test<T: PrimitiveFloat>(x: T, out: &str) {
        assert_eq!(NiceFloat(x).to_string(), out);
    }
    test::<f32>(f32::NAN, "NaN");
    test::<f32>(f32::INFINITY, "Infinity");
    test::<f32>(f32::NEGATIVE_INFINITY, "-Infinity");
    test::<f32>(0.0, "0.0");
    test::<f32>(-0.0, "-0.0");
    test::<f32>(1.0, "1.0");
    test::<f32>(-1.0, "-1.0");
    test::<f32>(123.0, "123.0");
    test::<f32>(0.123, "0.123");
    test::<f32>(1000.0, "1000.0");
    test::<f32>(1000000.0, "1000000.0");
    test::<f32>(1.0e20, "1.0e20");
    test::<f32>(f32::MIN_POSITIVE_SUBNORMAL, "1.0e-45");
    test::<f32>(f32::MAX_SUBNORMAL, "1.1754942e-38");
    test::<f32>(f32::MIN_POSITIVE_NORMAL, "1.1754944e-38");
    test::<f32>(f32::MAX_FINITE, "3.4028235e38");
    test::<f32>(2.0f32.sqrt(), "1.4142135");
    test::<f32>(std::f32::consts::E, "2.7182817");
    test::<f32>(std::f32::consts::PI, "3.1415927");

    test::<f64>(f64::NAN, "NaN");
    test::<f64>(f64::INFINITY, "Infinity");
    test::<f64>(f64::NEGATIVE_INFINITY, "-Infinity");
    test::<f64>(0.0, "0.0");
    test::<f64>(-0.0, "-0.0");
    test::<f64>(1.0, "1.0");
    test::<f64>(-1.0, "-1.0");
    test::<f64>(123.0, "123.0");
    test::<f64>(0.123, "0.123");
    test::<f64>(1000.0, "1000.0");
    test::<f64>(1000000.0, "1000000.0");
    test::<f64>(1.0e100, "1.0e100");
    test::<f64>(f64::MIN_POSITIVE_SUBNORMAL, "5.0e-324");
    test::<f64>(f64::MAX_SUBNORMAL, "2.225073858507201e-308");
    test::<f64>(f64::MIN_POSITIVE_NORMAL, "2.2250738585072014e-308");
    test::<f64>(f64::MAX_FINITE, "1.7976931348623157e308");
    test::<f64>(2.0f64.sqrt(), "1.4142135623730951");
    test::<f64>(std::f64::consts::E, "2.718281828459045");
    test::<f64>(std::f64::consts::PI, "3.141592653589793");
}

fn to_string_properties_helper<T: PrimitiveFloat>()
where
    <T as FromStr>::Err: Debug,
{
    primitive_float_gen::<T>().test_properties(|f| {
        let s = NiceFloat(f).to_string();
        assert_eq!(NiceFloat::from_str(&s).unwrap(), NiceFloat(f));
        assert!(string_is_subset(&s, PRIMITIVE_FLOAT_CHARS));
        if f.is_finite() {
            assert!(s.contains('.'));
            assert_eq!(NiceFloat(T::from_str(&s).unwrap()), NiceFloat(f));
        }
    });
}

#[test]
fn to_string_properties() {
    apply_fn_to_primitive_floats!(to_string_properties_helper);
}
