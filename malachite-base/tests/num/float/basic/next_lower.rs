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
use malachite_base::test_util::generators::primitive_float_gen_var_10;
use std::panic::catch_unwind;

#[allow(clippy::approx_constant)]
#[test]
pub fn test_next_lower() {
    fn test<T: PrimitiveFloat>(x: T, out: T) {
        assert_eq!(NiceFloat(x.next_lower()), NiceFloat(out));
    }
    test::<f32>(-f32::MAX_FINITE, f32::NEGATIVE_INFINITY);
    test::<f32>(-458.42188, -458.4219);
    test::<f32>(-10.0, -10.000001);
    test::<f32>(-core::f32::consts::PI, -3.141593);
    test::<f32>(-1.0, -1.0000001);
    test::<f32>(-0.1, -0.10000001);
    test::<f32>(-f32::MIN_POSITIVE_NORMAL, -1.1754945e-38);
    test::<f32>(-f32::MAX_SUBNORMAL, -f32::MIN_POSITIVE_NORMAL);
    test::<f32>(-f32::MIN_POSITIVE_SUBNORMAL, -3.0e-45);
    test::<f32>(-0.0, -f32::MIN_POSITIVE_SUBNORMAL);
    test::<f32>(0.0, -0.0);
    test::<f32>(f32::MIN_POSITIVE_SUBNORMAL, 0.0);
    test::<f32>(f32::MAX_SUBNORMAL, 1.1754941e-38);
    test::<f32>(f32::MIN_POSITIVE_NORMAL, f32::MAX_SUBNORMAL);
    test::<f32>(0.1, 0.099999994);
    test::<f32>(0.99999994, 0.9999999);
    test::<f32>(1.0, 0.99999994);
    test::<f32>(1.0000001, 1.0);
    test::<f32>(3.1415925, 3.1415923);
    test::<f32>(core::f32::consts::PI, 3.1415925);
    test::<f32>(3.141593, core::f32::consts::PI);
    test::<f32>(10.0, 9.999999);
    test::<f32>(f32::MAX_FINITE, 3.4028233e38);
    test::<f32>(f32::INFINITY, f32::MAX_FINITE);

    test::<f64>(-f64::MAX_FINITE, f64::NEGATIVE_INFINITY);
    test::<f64>(-10.0, -10.000000000000002);
    test::<f64>(-core::f64::consts::PI, -3.1415926535897936);
    test::<f64>(-1.0, -1.0000000000000002);
    test::<f64>(-0.1, -0.10000000000000002);
    test::<f64>(-f64::MIN_POSITIVE_NORMAL, -2.225073858507202e-308);
    test::<f64>(-f64::MAX_SUBNORMAL, -f64::MIN_POSITIVE_NORMAL);
    test::<f64>(-f64::MIN_POSITIVE_SUBNORMAL, -1.0e-323);
    test::<f64>(-0.0, -f64::MIN_POSITIVE_SUBNORMAL);
    test::<f64>(0.0, -0.0);
    test::<f64>(f64::MIN_POSITIVE_SUBNORMAL, 0.0);
    test::<f64>(f64::MAX_SUBNORMAL, 2.2250738585072004e-308);
    test::<f64>(f64::MIN_POSITIVE_NORMAL, f64::MAX_SUBNORMAL);
    test::<f64>(1.9261352099337372e-256, 1.926135209933737e-256);
    test::<f64>(0.1, 0.09999999999999999);
    test::<f64>(0.9999999999999999, 0.9999999999999998);
    test::<f64>(1.0, 0.9999999999999999);
    test::<f64>(1.0000000000000002, 1.0);
    test::<f64>(3.1415926535897927, 3.1415926535897922);
    test::<f64>(core::f64::consts::PI, 3.1415926535897927);
    test::<f64>(3.1415926535897936, core::f64::consts::PI);
    test::<f64>(10.0, 9.999999999999998);
    test::<f64>(f64::MAX_FINITE, 1.7976931348623155e308);
    test::<f64>(f64::INFINITY, f64::MAX_FINITE);
}

fn next_lower_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(T::NAN.next_lower());
    assert_panic!(T::NEGATIVE_INFINITY.next_lower());
}

#[test]
pub fn next_lower_fail() {
    apply_fn_to_primitive_floats!(next_lower_fail_helper);
}

fn next_lower_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen_var_10::<T>().test_properties(|x| {
        let y = x.next_lower();
        assert_eq!(
            x.to_ordered_representation() - 1,
            y.to_ordered_representation()
        );
        assert_eq!(NiceFloat(y.next_higher()), NiceFloat(x));
    });
}

#[test]
fn next_lower_properties() {
    apply_fn_to_primitive_floats!(next_lower_properties_helper);
}
