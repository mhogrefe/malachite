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
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{string_gen, string_gen_var_10};
use std::fmt::Debug;
use std::str::FromStr;

#[allow(clippy::approx_constant)]
#[test]
fn test_from_str() {
    fn test_ok<T: PrimitiveFloat>(s: &str, out: NiceFloat<T>)
    where
        <T as FromStr>::Err: Debug,
    {
        assert_eq!(NiceFloat::<T>::from_str(s).unwrap(), out);
    }
    test_ok::<f32>("NaN", NiceFloat(f32::NAN));
    test_ok::<f32>("Infinity", NiceFloat(f32::INFINITY));
    test_ok::<f32>("-Infinity", NiceFloat(f32::NEGATIVE_INFINITY));
    test_ok::<f32>("0", NiceFloat(0.0));
    test_ok::<f32>("00", NiceFloat(0.0));
    test_ok::<f32>("0.", NiceFloat(0.0));
    test_ok::<f32>(".0", NiceFloat(0.0));
    test_ok::<f32>("1", NiceFloat(1.0));
    test_ok::<f32>("1.0", NiceFloat(1.0));
    test_ok::<f32>("1.0000", NiceFloat(1.0));
    test_ok::<f32>("3.14", NiceFloat(3.14));
    test_ok::<f32>("1e2", NiceFloat(100.0));
    test_ok::<f32>("1e20", NiceFloat(1.0e20));
    test_ok::<f32>("1.0e1000", NiceFloat(f32::INFINITY));
    test_ok::<f32>("1.0e-1000", NiceFloat(0.0));
    test_ok::<f32>("-0", NiceFloat(-0.0));
    test_ok::<f32>("-00", NiceFloat(-0.0));
    test_ok::<f32>("-0.", NiceFloat(-0.0));
    test_ok::<f32>("-.0", NiceFloat(-0.0));
    test_ok::<f32>("-1", NiceFloat(-1.0));
    test_ok::<f32>("-1.0", NiceFloat(-1.0));
    test_ok::<f32>("-1.0000", NiceFloat(-1.0));
    test_ok::<f32>("-3.14", NiceFloat(-3.14));
    test_ok::<f32>("-1e2", NiceFloat(-100.0));
    test_ok::<f32>("-1e20", NiceFloat(-1.0e20));
    test_ok::<f32>("-1.0e1000", NiceFloat(f32::NEGATIVE_INFINITY));
    test_ok::<f32>("-1.0e-1000", NiceFloat(-0.0));

    test_ok::<f64>("NaN", NiceFloat(f64::NAN));
    test_ok::<f64>("Infinity", NiceFloat(f64::INFINITY));
    test_ok::<f64>("-Infinity", NiceFloat(f64::NEGATIVE_INFINITY));
    test_ok::<f64>("0", NiceFloat(0.0));
    test_ok::<f64>("00", NiceFloat(0.0));
    test_ok::<f64>("0.", NiceFloat(0.0));
    test_ok::<f64>(".0", NiceFloat(0.0));
    test_ok::<f64>("1", NiceFloat(1.0));
    test_ok::<f64>("1.0", NiceFloat(1.0));
    test_ok::<f64>("1.0000", NiceFloat(1.0));
    test_ok::<f64>("3.14", NiceFloat(3.14));
    test_ok::<f64>("1e2", NiceFloat(100.0));
    test_ok::<f64>("1e20", NiceFloat(1.0e20));
    test_ok::<f64>("1.0e1000", NiceFloat(f64::INFINITY));
    test_ok::<f64>("1.0e-1000", NiceFloat(0.0));
    test_ok::<f64>("-0", NiceFloat(-0.0));
    test_ok::<f64>("-00", NiceFloat(-0.0));
    test_ok::<f64>("-0.", NiceFloat(-0.0));
    test_ok::<f64>("-.0", NiceFloat(-0.0));
    test_ok::<f64>("-1", NiceFloat(-1.0));
    test_ok::<f64>("-1.0", NiceFloat(-1.0));
    test_ok::<f64>("-1.0000", NiceFloat(-1.0));
    test_ok::<f64>("-3.14", NiceFloat(-3.14));
    test_ok::<f64>("-1e2", NiceFloat(-100.0));
    test_ok::<f64>("-1e20", NiceFloat(-1.0e20));
    test_ok::<f64>("-1.0e1000", NiceFloat(f64::NEGATIVE_INFINITY));
    test_ok::<f64>("-1.0e-1000", NiceFloat(-0.0));

    let test_err = |s| {
        assert!(NiceFloat::<f32>::from_str(s).is_err());
        assert!(NiceFloat::<f64>::from_str(s).is_err());
    };
    test_err("-");
    test_err(".");
    test_err("e");
    test_err("z");
    test_err("0x01");
    test_err(" 0 ");
}

#[allow(unused_must_use)]
fn from_str_helper<T: PrimitiveFloat>() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 128);
    config.insert("mean_length_d", 1);
    string_gen().test_properties_with_config(&config, |s| {
        NiceFloat::<T>::from_str(&s);
    });
    string_gen_var_10().test_properties_with_config(&config, |s| {
        NiceFloat::<T>::from_str(&s);
    });
}

#[test]
fn from_str_properties() {
    apply_fn_to_primitive_floats!(from_str_helper);
}
