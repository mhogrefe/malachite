// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, IsReal};
use malachite_base::num::float::NiceFloat;
use malachite_base::strings::ToDebugString;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::gaussian_integer::conversion::primitive_float_from_gaussian_integer::*;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use std::str::FromStr;

#[test]
fn test_try_from_gaussian_integer() {
    fn test<T>(s: &str, out: &str)
    where
        T: std::fmt::Debug
            + for<'a> TryFrom<&'a GaussianInteger, Error = PrimitiveFloatFromGaussianIntegerError>,
    {
        let x = GaussianInteger::from_str(s).unwrap();
        assert_eq!(T::try_from(&x).to_debug_string(), out);
    }
    test::<f32>("0", "Ok(0.0)");
    test::<f32>("123", "Ok(123.0)");
    test::<f32>("-123", "Ok(-123.0)");
    test::<f32>("16777216", "Ok(16777216.0)");
    test::<f32>("16777217", "Err(PrimitiveFloatFromGaussianIntegerError)");
    test::<f64>("16777217", "Ok(16777217.0)");
    test::<f32>("i", "Err(PrimitiveFloatFromGaussianIntegerError)");
    test::<f32>("2-3i", "Err(PrimitiveFloatFromGaussianIntegerError)");
    test::<f64>("2-3i", "Err(PrimitiveFloatFromGaussianIntegerError)");
}

#[test]
fn test_exact_from_gaussian_integer() {
    let x = GaussianInteger::from_str("123").unwrap();
    assert_eq!(NiceFloat(f32::exact_from(&x)), NiceFloat(123.0));
    assert_eq!(NiceFloat(f64::exact_from(&x)), NiceFloat(123.0));
}

#[test]
#[should_panic]
fn f32_exact_from_gaussian_integer_fail() {
    f32::exact_from(&GaussianInteger::from_str("16777217").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_gaussian_integer_fail() {
    f64::exact_from(&GaussianInteger::from_str("2-3i").unwrap());
}

#[test]
fn test_convertible_from_gaussian_integer() {
    fn test<T: for<'a> ConvertibleFrom<&'a GaussianInteger>>(s: &str, out: bool) {
        let x = GaussianInteger::from_str(s).unwrap();
        assert_eq!(T::convertible_from(&x), out);
    }
    test::<f32>("0", true);
    test::<f32>("123", true);
    test::<f32>("-123", true);
    test::<f32>("16777216", true);
    test::<f32>("16777217", false);
    test::<f64>("16777217", true);
    test::<f32>("i", false);
    test::<f32>("2-3i", false);
}

fn try_from_gaussian_integer_properties_helper<T>()
where
    T: PrimitiveFloat
        + for<'a> TryFrom<&'a GaussianInteger, Error = PrimitiveFloatFromGaussianIntegerError>
        + for<'a> TryFrom<&'a Integer>
        + for<'a> ConvertibleFrom<&'a GaussianInteger>
        + for<'a> ConvertibleFrom<&'a Integer>,
    GaussianInteger: TryFrom<T> + ConvertibleFrom<T>,
{
    gaussian_integer_gen().test_properties(|x| {
        let ot = T::try_from(&x);
        assert_eq!(T::convertible_from(&x), ot.is_ok());
        assert_eq!(ot.is_ok(), x.is_real() && T::convertible_from(&x.real));
        if let Ok(t) = ot {
            assert_eq!(NiceFloat(t), NiceFloat(T::try_from(&x.real).ok().unwrap()));
            assert_eq!(GaussianInteger::try_from(t).ok().unwrap(), x);
        }
    });
}

#[test]
fn try_from_gaussian_integer_properties() {
    apply_fn_to_primitive_floats!(try_from_gaussian_integer_properties_helper);
}
