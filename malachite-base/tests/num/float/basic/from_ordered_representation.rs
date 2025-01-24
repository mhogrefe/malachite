// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{unsigned_gen_var_13, unsigned_pair_gen_var_22};
use std::panic::catch_unwind;

#[allow(clippy::approx_constant)]
#[test]
pub fn test_from_ordered_representation() {
    fn test<T: PrimitiveFloat>(n: u64, out: T) {
        assert_eq!(NiceFloat(T::from_ordered_representation(n)), NiceFloat(out));
    }
    test::<f32>(0, f32::NEGATIVE_INFINITY);
    test::<f32>(1, -f32::MAX_FINITE);
    test::<f32>(1000000000, -458.42188);
    test::<f32>(1046478848, -10.0);
    test::<f32>(1060565029, -core::f32::consts::PI);
    test::<f32>(1073741824, -1.0);
    test::<f32>(1102263091, -0.1);
    test::<f32>(2130706432, -f32::MIN_POSITIVE_NORMAL);
    test::<f32>(2130706433, -f32::MAX_SUBNORMAL);
    test::<f32>(2139095039, -f32::MIN_POSITIVE_SUBNORMAL);
    test::<f32>(2139095040, -0.0);
    test::<f32>(2139095041, 0.0);
    test::<f32>(2139095042, f32::MIN_POSITIVE_SUBNORMAL);
    test::<f32>(2147483648, f32::MAX_SUBNORMAL);
    test::<f32>(2147483649, f32::MIN_POSITIVE_NORMAL);
    test::<f32>(3175926990, 0.1);
    test::<f32>(3204448256, 0.99999994);
    test::<f32>(3204448257, 1.0);
    test::<f32>(3204448258, 1.0000001);
    test::<f32>(3217625051, 3.1415925);
    test::<f32>(3217625052, core::f32::consts::PI);
    test::<f32>(3217625053, 3.141593);
    test::<f32>(3231711233, 10.0);
    test::<f32>(4278190080, f32::MAX_FINITE);
    test::<f32>(4278190081, f32::INFINITY);

    test::<f64>(0, f64::NEGATIVE_INFINITY);
    test::<f64>(1, -f64::MAX_FINITE);
    test::<f64>(4597049319638433792, -10.0);
    test::<f64>(4604611780675359464, -core::f64::consts::PI);
    test::<f64>(4611686018427387904, -1.0);
    test::<f64>(4626998257160447590, -0.1);
    test::<f64>(9214364837600034816, -f64::MIN_POSITIVE_NORMAL);
    test::<f64>(9214364837600034817, -f64::MAX_SUBNORMAL);
    test::<f64>(9218868437227405311, -f64::MIN_POSITIVE_SUBNORMAL);
    test::<f64>(9218868437227405312, -0.0);
    test::<f64>(9218868437227405313, 0.0);
    test::<f64>(9218868437227405314, f64::MIN_POSITIVE_SUBNORMAL);
    test::<f64>(9223372036854775808, f64::MAX_SUBNORMAL);
    test::<f64>(9223372036854775809, f64::MIN_POSITIVE_NORMAL);
    test::<f64>(10000000000000000000, 1.9261352099337372e-256);
    test::<f64>(13810738617294363035, 0.1);
    test::<f64>(13826050856027422720, 0.9999999999999999);
    test::<f64>(13826050856027422721, 1.0);
    test::<f64>(13826050856027422722, 1.0000000000000002);
    test::<f64>(13833125093779451160, 3.1415926535897927);
    test::<f64>(13833125093779451161, core::f64::consts::PI);
    test::<f64>(13833125093779451162, 3.1415926535897936);
    test::<f64>(13840687554816376833, 10.0);
    test::<f64>(18437736874454810624, f64::MAX_FINITE);
    test::<f64>(18437736874454810625, f64::INFINITY);
}

#[test]
pub fn from_ordered_representation_fail() {
    assert_panic!(f32::from_ordered_representation(4278190082));
    assert_panic!(f32::from_ordered_representation(u64::MAX));
    assert_panic!(f64::from_ordered_representation(18437736874454810626));
    assert_panic!(f64::from_ordered_representation(u64::MAX));
}

fn from_ordered_representation_properties_helper<T: PrimitiveFloat>() {
    unsigned_gen_var_13::<T>().test_properties(|x| {
        let f = T::from_ordered_representation(x);
        assert!(!f.is_nan());
        assert_eq!(f.to_ordered_representation(), x);
    });

    unsigned_pair_gen_var_22::<T>().test_properties(|(x, y)| {
        assert_eq!(
            x.cmp(&y),
            NiceFloat(T::from_ordered_representation(x))
                .cmp(&NiceFloat(T::from_ordered_representation(y)))
        );
    });
}

#[test]
fn from_ordered_representation_properties() {
    apply_fn_to_primitive_floats!(from_ordered_representation_properties_helper);
}
